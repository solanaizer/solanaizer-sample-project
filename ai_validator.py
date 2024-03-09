import requests
import json

class ChatRequest:
    def __init__(self, model, messages):
        self.model = model
        self.messages = messages

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, 
            sort_keys=True, indent=4)


def validate_vulnerability_with_gpt(api_key, title, severity, line_number, line_of_code, file_content):
    url = "https://api.openai.com/v1/chat/completions"
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {api_key}"
    }

    prompt = f"A SAST tool detects a potential Rust vulnerability titled '{title}' with severity '{severity}' at line number {line_number}. The line of code flagged is:\n\n{line_of_code}\n\nFull code for context:\n\n{file_content}\n\nIs this a valid vulnerability or a false positive? If valid, suggest a fix."

    chat_request = ChatRequest(
        model="gpt-3.5-turbo",
        messages= [{"role": "user", "content" : prompt}]
    )

    response = requests.post(url, headers=headers, data=chat_request.toJSON())

    if response.ok:
        response_json = response.json()
        response_content = response_json["choices"][0]["message"]["content"]
        status = analyze_response_text(response_content, "gpt-response.txt")
        return status, response_content
    else:
        if response.status_code == 403:
            error_message = "Authorization failed. Please check your API key permissions."
        elif response.status_code == 429:
            error_message = "Rate limit exceeded. Please try again later."
        elif response.status_code == 400:
            error_message = "Bad request. Please check your request parameters."
        else:
            error_message = f"Failed to get a valid response from OpenAI: {response.status_code} - {response.text}"
        raise IOError(error_message)

def analyze_response_text(text, output_file):
    with open(output_file, "w") as file:
        file.write(text)
    return "Content dumped into file successfully."
