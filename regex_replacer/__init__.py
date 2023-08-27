from .regex_replacer import RegexReplacer
from sklearn.base import BaseEstimator, TransformerMixin
from typing import List, Tuple


class RegexReplacerTransformer(TransformerMixin, BaseEstimator):
    def __init__(self, re_list: List[Tuple[str, str]], n_jobs: int = 0):
        super().__init__()
        self.transformer = RegexReplacer(re_list, n_jobs)
        self.n_jobs = n_jobs

    def fit(self, X, y=None):
        return self

    def to_single_thread(self):
        self.n_jobs = 0

    def transform(self, X: List[str]) -> List[str]:
        single_thread = self.n_jobs <= 0
        return self.transformer.transform(X, single_thread=single_thread)
