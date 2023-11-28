import fitz  # PyMuPDF

def scrape_PDF(fileName) -> str:
    #fileName = "Test.pdf"

    pdf_document = fitz.open(fileName)

    markdown_text = ""

    for page_number in range(pdf_document.page_count):
        page = pdf_document[page_number]
        page_text = page.get_text()
        markdown_text += page_text

    return markdown_text

print(scrape_PDF("Test.pdf"))