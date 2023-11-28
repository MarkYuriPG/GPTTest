use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyModule},
};


pub fn GET_url(url:&String) -> PyResult<bool> {
    pyo3::prepare_freethreaded_python();
Python::with_gil(|py| {
    let get = PyModule::from_code(
        py,
        r#"
import requests

def get_url(url):
    pageToScrape = None
    try:
        pageToScrape = requests.get(url)
    except Exception as e:
        #Handle the error
        print(f"Invalid URL Error: {e}")
    finally:
        if pageToScrape is None
            return False

    return True
    "#,
        "validator.py",
        "validator",
    )?;

    let status: bool = get
    .getattr("get_url")?
    .call((url,), None)?
    .extract()?;

    Ok(status)
})
}