import pdfplumber
import openai

with pdfplumber.open('Design patterns Midterm Notes.pdf') as pdf:
    # iterate over each page
    for page in pdf.pages:
        # extract text
        text = page.extract_text()
        #print(text)

openai.api_key = "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"

prompt = "Can you copy this: " + text

completion = openai.ChatCompletion.create(
  model="gpt-3.5-turbo",
  messages=[
    {"role": "system", "content": "You are a computer science teacher."},
    {"role": "user", "content": prompt}
  ]
)

print(completion.choices[0].message)