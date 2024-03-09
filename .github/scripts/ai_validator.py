import requests
import json
from pathlib import Path


class ChatRequest:
    def __init__(self, model, messages):
        self.model = model
        self.messages = messages

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, 
            sort_keys=True, indent=4)


def analyze_vulnerability_with_gpt(api_key, file_content, filename: Path):
    url = "https://api.openai.com/v1/chat/completions"
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {api_key}",
        "Accept": "application/json"
    }

    prompt = f"""
[no prose]
You need to answer at all time using a JSON object of this format:

Array<{{ severity: "HIGH"|"MEDIUM"|"LOW"; message: string; errorCode: string; filename: string; lines: Array<number> }}>;

If you find no errors, you should return an empty array.

The filename key should contain the name of the module. 
You are an Solana smart contract auditor. You are an expert at finding vulnerabilities that can be exploited by bad people.

You have to find vulnerabilities in this smart contract written in Rust:

```rs
'{file_content}'
```

NEVER EVER EVER RETURN ANYTHING ELSE THAN JSON. DON'T RETURN MARKDOWN
"""



    chat_request = ChatRequest(
        model="gpt-3.5-turbo",
        messages= [{"role": "user", "content" : prompt}]
    )

    response = requests.post(url, headers=headers, data=chat_request.toJSON())

    if response.ok:
        response_json = response.json()
        response_content = response_json["choices"][0]["message"]["content"].replace("```json", "").replace("```", "")
        if (response_content != ""):
            parsed = json.loads(response_content)
            
            for item in parsed:
                item["filename"] = filename.relative_to(".").__str__()

            return parsed
        else:
            return []
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


