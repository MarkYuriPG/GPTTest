use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyModule},
};

// # Some urls to test:
//     # https://sourcemaking.com/design_patterns
//     # https://www.w3schools.com/html/
//     # https://www.javatpoint.com/theory-of-automata
//     # https://www.indeed.com/career-advice/career-development/types-of-networks (has bot protection)

pub fn scrape_url() -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
Python::with_gil(|py| {
    let scraper = PyModule::from_code(
        py,
        r#"
from bs4 import BeautifulSoup
import requests

def scrape(url):
    webpage = requests.get(url)
    soup = BeautifulSoup(webpage.text, "html.parser")

    result_string = ""

    def contains_link(element):
        return element.find('a') is not None

    # Elements to exclude
    def is_valid_element(element):
        if element.name == 'a':
            return False
        if element.find_parent('a'):
            return False
        if element.find_parent('nav'):
            return False
        if element.find_parent('div', id=lambda value: value and ('menu' in value or 'footer' in value)):
            return False # Divs with id that includes 'menu' and 'footer'
        return True

    # Find all headers (h1 to h6), paragraphs, and their parent elements while excluding links and their child elements
    valid_elements = filter(is_valid_element, soup.recursiveChildGenerator())
    valid = ['h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'p', 'pre', 'li', 'span', 'div']

    # Iterate through the valid elements and print their text
    for element in valid_elements:
        if element.name and element.name in valid and not contains_link(element):
            result_string += element.text.strip() + "\n"
    
    return result_string
    "#,
        "scraper.py",
        "scraper",
    )?;

    let lesson_source: String = scraper
    .getattr("scrape")?
    .call(("https://www.indeed.com/career-advice/career-development/types-of-networks",), None)?
    .extract()?;

    Ok(lesson_source)
})
}