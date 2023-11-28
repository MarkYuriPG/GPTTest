from dotenv import load_dotenv
import os
import openai

def get_reply(source:str)->str:
    load_dotenv()

    openai.api_key = os.getenv("API_KEY")

    prompt = "Can you make a markdown format lesson based on this source: " + source

    completion = openai.ChatCompletion.create(
    model="gpt-3.5-turbo",
    messages=[
        {"role": "system", "content": "You are a college teacher."},
        {"role": "user", "content": prompt}
    ]
    )

    #REPLY of gpt
    reply = completion['choices'][0]['message']['content']
    return reply
    
    #print(completion['choices'][0]['message']['content'])