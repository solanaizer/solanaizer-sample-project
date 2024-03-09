import requests
import json

from json.decoder import JSONDecodeError

class ChatRequest:
    def __init__(self, model, messages):
        self.model = model
        self.messages = messages

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, 
            sort_keys=True, indent=4)


def analyze_vulnerability_with_gpt(api_key, file_content):
    url = "https://api.openai.com/v1/chat/completions"
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {api_key}",
        "Accept": "application/json"
    }
    prompt = f"Detect all vulnerabilities within the provided Rust smart contract code for Solana blockchain. \n '{file_content}' \n Respond in JSON format, filling the following properties for each vulnerability found \n vulnerabilities: {[]} \n message: Provide a detailed description of any vulnerabilities found.\n severity: Specify the severity of the vulnerability (low, medium, or high).\n line: This should be an array where the first element is the line and the second is the column where the vulnerability is present.\n\n---\n"

    chat_request = ChatRequest(
        model="gpt-3.5-turbo",
        messages= [{"role": "user", "content" : prompt}]
    )

    response = requests.post(url, headers=headers, data=chat_request.toJSON())

    if response.ok:
        response_json = response.json()
        print(response_json)
        response_content = response_json["choices"][0]["message"]["content"]
        if (response_content != ""):
            analyze_response_text(response_content)
        else:
            print("No vulnerabilities found.")
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


def analyze_response_text(response_content):
    try:
        response_content = response_content.strip('`').strip('json')
        response_dict = json.loads(response_content)
        if "vulnerabilities" not in response_dict:
            print(response_dict)
            print("No vulnerabilities found.")
            return
        
        vulnerabilities = response_dict["vulnerabilities"]
        for vulnerability in vulnerabilities:
            message = vulnerability["message"]
            severity = vulnerability["severity"]
            lines = vulnerability["line"]
            
            print("Message:", message)
            print("Severity:", severity)
            print("Lines:", lines)
            print("\n")

    except JSONDecodeError:
        print("Failed to parse response content as JSON. Response content may not be in the expected format.")
        print("Response Content:", response_content)

