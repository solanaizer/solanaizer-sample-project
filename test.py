import json

def get_test_json():
    #  Define a Python dictionary that we'll convert to a JSON object
    test_data = [
        {
        "severity": "HIGH",
        "message": "This error blablabla",
        "errorCode": "REENTRY_ERROR";
        "lines": [5,10];
        }
]
    
    # Convert the Python dictionary to a JSON string
    json_data = json.dumps(test_data)
    
    # Return the JSON string
    return json_data

# Call the function and print the result
print(get_test_json())
