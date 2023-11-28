use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyModule},
};


pub fn check_url(scraped_url:&String) -> PyResult<bool> {
    pyo3::prepare_freethreaded_python();
Python::with_gil(|py| {
    let checker = PyModule::from_code(
        py,
        r#"
def check_availability(text):
    # Add security keywords here
    security_keywords = ["cloudflare", "captcha"]

    # Check if scraped text is a response from security checks
    text = text.lower()
    for keyword in security_keywords:
        if keyword in text:
            return False
    if not text.strip():
        return False

    # Return true if website is scrapable and ready to feed to GPT-4
    return True
    "#,
        "checker.py",
        "checker",
    )?;

    let protection: bool = checker
    .getattr("check_availability")?
    .call((scraped_url,), None)?
    .extract()?;

    Ok(protection)
})
}