from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="regex_replacer",
    version="0.1.1",
    rust_extensions=[RustExtension("regex_replacer.regex_replacer", binding=Binding.PyO3)],
    packages=["regex_replacer"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    install_requires=['scikit-learn']
)