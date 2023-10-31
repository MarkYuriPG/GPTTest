import pdfplumber
import aspose.words as aw
import openai
import markdown
import re

with pdfplumber.open('Design patterns Midterm Notes.pdf') as pdf:
    # iterate over each page
    for page in pdf.pages:
        # extract text
        text = page.extract_text()
        # print(text)

fileName = "Design patterns Midterm Notes.pdf"

output = aw.Document()
output.remove_all_children()

input = aw.Document(fileName)
output.append_document(input, aw.ImportFormatMode.KEEP_SOURCE_FORMATTING)

output.save("Output.md")

with open("Output.md", "r", encoding="utf-8") as file:
    markdown_text = file.read()

    start_index = markdown_text.find("!Evaluation Only")
    end_index = markdown_text.rfind("https://products.aspose.com/words/")

    if start_index != -1 and end_index != -1:
    # Remove the watermark
        markdown_text = markdown_text[:start_index] + markdown_text[end_index + len("https://products.aspose.com/words/"):]

    print(markdown_text)

# with open("Output.md", "r") as file:
#     markdown_text = file.read()
#     html_text = markdown.markdown(markdown_text)

# openai.api_key = "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"

# prompt = "Can you copy this: " + text

# completion = openai.ChatCompletion.create(
#   model="gpt-3.5-turbo",
#   messages=[
#     {"role": "system", "content": "You are a computer science teacher."},
#     {"role": "user", "content": prompt}
#   ]
# )

# print(completion.choices[0].message)