use pyo3::prelude::*;
use regex::{Regex};
use std::sync::Arc;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

#[pyclass]
struct RegexReplacer {
    re_set: Arc<Vec<(Regex, String)>>
}

#[pymethods]
impl RegexReplacer {
    #[new]
    #[args(n_jobs=0)]
    pub fn new(re_list: Vec<(&str, &str)>, n_jobs: usize) -> Self {
        let re_set = Arc::new(re_list.into_iter().map(|(r, t)| (Regex::new(r).unwrap(), String::from(t))).collect());
        if n_jobs > 1 {
            ThreadPoolBuilder::new().num_threads(n_jobs).build_global().unwrap();
        }
        RegexReplacer { re_set }
    }

    #[args(single_thread=false)]
    pub fn transform(&mut self, texts: Vec<String>, single_thread: bool) -> Vec<String> {
        let reg_set = Arc::clone(&(self.re_set));
        if single_thread {
            texts.into_iter().map(move |t| self._process(t, &reg_set)).collect()
        } else {
            texts.into_par_iter().map(move |t| self._process(t, &reg_set)).collect()
        }

    }
}

impl RegexReplacer {
    fn _process(&self, t: String, reg_set: &Arc<Vec<(Regex, String)>>) -> String {
        let mut res = t;
        for (reg, target) in reg_set.iter() {
            let k = reg.replace_all(res.as_str(), target);
            res = String::from(k);
        }
        String::from(res)
    }
}

#[pymodule]
fn regex_replacer(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RegexReplacer>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::RegexReplacer;

    #[test]
    fn it_works() {
        let mut rr = RegexReplacer::new(Vec::from([(r"\d+", "-"), (r"[a-z]+", "+")]), 2);
        let res = rr.transform(Vec::from([String::from("hbkjfsbiu2746928764nbdkfasd")]), false);
        assert_eq!(res[0], "+-+");
    }
}