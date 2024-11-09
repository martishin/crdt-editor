use crate::crdt::{LWWElementDictionary, Timestamp};
use pyo3::prelude::*;
use pyo3::types::PyString;
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
struct PyTimestamp(Timestamp);

#[pymethods]
impl PyTimestamp {
    #[new]
    pub fn new() -> Self {
        PyTimestamp(Timestamp::now())
    }

    #[staticmethod]
    pub fn now() -> Self {
        PyTimestamp(Timestamp::now())
    }

    pub fn value(&self) -> u64 {
        self.0.value()
    }
}

#[pyclass]
struct PyLWWElementDictionary {
    inner: Arc<LWWElementDictionary<String, String>>,
}

#[pymethods]
impl PyLWWElementDictionary {
    #[new]
    pub fn new() -> Self {
        PyLWWElementDictionary {
            inner: Arc::new(LWWElementDictionary::new()),
        }
    }

    #[pyo3(signature = (key, value, timestamp=None))]
    pub fn add(&self, key: String, value: String, timestamp: Option<PyTimestamp>) {
        let ts = timestamp.map(|t| t.0).unwrap_or_else(Timestamp::now);
        self.inner.add(key, value, ts);
    }

    #[pyo3(signature = (key, timestamp=None))]
    pub fn remove(&self, key: String, timestamp: Option<PyTimestamp>) {
        let ts = timestamp.map(|t| t.0).unwrap_or_else(Timestamp::now);
        self.inner.remove(&key, ts);
    }

    pub fn lookup<'py>(&self, py: Python<'py>, key: String) -> PyResult<Option<Py<PyString>>> {
        if let Some(value_arc) = self.inner.lookup(&key) {
            let py_string: Py<PyString> = PyString::new_bound(py, &value_arc).into_py(py);
            Ok(Some(py_string))
        } else {
            Ok(None)
        }
    }

    #[pyo3(signature = (key, value, timestamp=None))]
    pub fn update(&self, key: String, value: String, timestamp: Option<PyTimestamp>) {
        let ts = timestamp.map(|t| t.0).unwrap_or_else(Timestamp::now);
        self.inner.update(key, value, ts);
    }

    pub fn merge(&self, other: &PyLWWElementDictionary) {
        self.inner.merge(&other.inner);
    }

    pub fn keys<'py>(&self, py: Python<'py>) -> PyResult<Vec<Py<PyString>>> {
        let keys: Vec<Py<PyString>> = self
            .inner
            .keys()
            .map(|key| PyString::new_bound(py, &key).into_py(py))
            .collect();
        Ok(keys)
    }
}

#[pymodule]
fn crdt_lww(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyTimestamp>()?;
    m.add_class::<PyLWWElementDictionary>()?;
    Ok(())
}
