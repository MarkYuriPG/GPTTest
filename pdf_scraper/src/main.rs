// use pyo3::{
//     prelude::*,
//     types::{IntoPyDict, PyModule},
// };

// fn main() -> PyResult<()> {
// pyo3::prepare_freethreaded_python();

// Python::with_gil(|py| {
//    // pyo3::prepare_freethreaded_python();
//     let converter = PyModule::from_code(
//         py,
//         r#"
// import fitz

// def convert(src):
//     fileName = src
//     pdf_document = fitz.open(fileName)
//     markdown_text = ""
//     for page_number in range(pdf_document.page_count):
//         page = pdf_document[page_number]
//         page_text = page.get_text()
//         markdown_text += page_text
//     return markdown_text
//     "#,
//         "converter.py",
//         "converter",
//     )?;

//     let markdown_text: String = converter.getattr("convert")?.call(("../Test.pdf",), None)?.extract()?;
//     println!("{}", markdown_text);

//     Ok(())
// })
// }
// use pyo3::{
//     prelude::*,
//     types::{IntoPyDict, PyModule},
// };


// pub fn scrape_pdf() -> PyResult<String> {
//     pyo3::prepare_freethreaded_python();
// Python::with_gil(|py| {
//     let scraper = PyModule::from_code(
//         py,
//         r#"
// import fitz

// def scrape(src):
//     fileName = src
//     pdf_document = fitz.open(fileName)
//     markdown_text = ""
//     for page_number in range(pdf_document.page_count):
//         page = pdf_document[page_number]
//         page_text = page.get_text()
//         markdown_text += page_text
//     return markdown_text
//     "#,
//         "scraper.py",
//         "scraper",
//     )?;

//     let lesson_source: String = scraper.getattr("scrape")?.call(("../Test.pdf",), None)?.extract()?;
//     //println!("{}", lesson_source);

//     Ok(lesson_source)
// })
// }

mod ai;
mod pdf_scraper;
mod web_scraper;
mod url_check;
mod validate_url;
mod scraper;
use std::env;

//use validate_url::validate_url;
//use url_check::check_url;
//use pdf_scraper::scrape_pdf;
use ai::ai_generate_lesson;
use scraper::{
    scrape_pdf,
    scrape_txt,
    scrape_docx,
    scrape_pptx,
    scrape_url,
    scrape_xlsx,
    scrape_csv
};
//use web_scraper::scrape_url;

//test
fn main() {
    if let Ok(current_dir) = env::current_dir() {
        println!("Current working directory: {:?}", current_dir);
    } else {
        eprintln!("Failed to get the current working directory");
        return;
    }

    let text_result = scraper::scrape_xlsx("..\\pdf_scraper\\test.xlsx".to_string());
    match text_result {
        Ok(result) => println!("Text Scraped: {}", result),
        Err(err) => eprintln!("Error scraping file or url: {}", err),
    }
}


    // if let Ok(lesson_source) = scrape_url() {
        
    //     if let Ok(protection) = check_url(&lesson_source){
    //         if protection
    //         {
    //             println!("{}", lesson_source);
    //         }   
    //         else
    //         {
    //             println!("This website is protected!");
    //         }
    //     }
    // }
    // else if let Err(error) = scrape_url()
    // {
    //     eprintln!("{}", error);
    // } 
    // else {
    //     println!("Failed to scrape URL");
    // }


    //PDF SCRAPER
    // if let Ok(lesson_source) = scrape_pdf() {
    //     // Pass markdown_text to another function in a different module/file
    //     // pdf_scraper::process_markdown(markdown_text);
    //     //println!("{}", lesson_source);
    //     if let Ok(generated_lesson) = ai_generate_lesson()
    //     {
    //         println!("{}", generated_lesson);
    //     }
    //     else if let Err(error) = ai_generate_lesson(){
    //         //println!("Failed to generate lesson!");
    //         eprintln!("Failed to generate lesson: {}", error);
    //     }
    // } else {
    //     println!("Failed to scrape PDF");
    // }
    // match scrape_pdf() {
    //     Ok(lesson_source) => {
    //         // Handle the case where scraping is successful
    //         println!("{}", lesson_source);
    //     }
    //     Err(error) => {
    //         // Handle the case where scraping failed
    //         eprintln!("Failed to scrape PDF: {}", error);
    //     }
    //}