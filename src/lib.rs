use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::PyList;

pub fn webdriver(py: pyo3::Python, headless: bool, no_image: bool)
    -> PyResult<Py<PyAny>> {
        PyModule::from_code(py,
            include_str!("utils.py"), "", "")?
            .getattr("webdriver")?
            .call((),
                Some([("headless", headless), ("no_image", no_image)]
                    .into_py_dict(py)))?
            .extract()
    }

pub fn find_element(py: pyo3::Python, parent: &Py<PyAny>, xpath: &str)
    -> PyResult<Py<PyAny>> {
    PyModule::from_code(py,
        include_str!("utils.py"), "", "")?
        .getattr("find_element")?
        .call1((parent, xpath))?
        .extract()
}

pub fn find_elements(py: pyo3::Python, parent: &Py<PyAny>, xpath: &str)
    -> PyResult<Py<PyList>> {
    PyModule::from_code(py,
        include_str!("utils.py"), "", "")?
        .getattr("find_elements")?
        .call1((parent, xpath))?
        .extract()
}