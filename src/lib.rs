use pyo3::prelude::*;
use regex::{Regex};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use rayon::{ThreadPoolBuilder, ThreadPool};

#[pyclass]
struct RegexReplacer {
    re_set: Arc<Vec<(Regex, String)>>,
    executor: Arc<Mutex<Option<ThreadPool>>>
}

#[pymethods]
impl RegexReplacer {
    #[new]
    #[pyo3(signature=(re_list, n_jobs=1))]
    pub fn new(re_list: Vec<(&str, &str)>, n_jobs: usize) -> Self {
        let re_set = Arc::new(re_list.into_iter().map(|(r, t)| (Regex::new(r)
                                                                    .expect(("Error compile regex: ".to_string() + r).as_str()), String::from(t))).collect());
        let executor;
        if n_jobs > 1 {
            executor = Arc::new(Mutex::new(Some(ThreadPoolBuilder::new().num_threads(n_jobs).build().unwrap())));
        } else {
            executor = Arc::new(Mutex::new(None));
        }
        RegexReplacer { re_set, executor }
    }

    #[pyo3(signature=())]
    pub fn to_single_thread(&mut self) {
        let mut e = self.executor.lock().unwrap();
        *e = None;
    }

    #[pyo3(signature=(texts))]
    pub fn transform(&mut self, texts: Vec<String>) -> Vec<String> {
        let reg_set = self.re_set.clone();
        match self.executor.clone().lock().unwrap().as_ref() {
            Some(e) => e.install(|| texts.into_par_iter().map(move |t| self._process(t, &reg_set)).collect()),
            None => texts.into_iter().map(move |t| self._process(t, &reg_set)).collect()
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
        let res = rr.transform(Vec::from([String::from("hbkjfsbiu2746928764nbdkfasd")]));
        assert_eq!(res[0], "+-+");
    }
}