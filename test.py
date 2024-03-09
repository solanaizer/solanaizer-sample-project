import json

def get_test_json():
    # Define a Python dictionary that we'll convert to a JSON object
    test_data = {
        "name": "John Doe",
        "age": 30,
        "is_active": True,
        "skills": ["Python", "JavaScript", "SQL"]
    }
    
    # Convert the Python dictionary to a JSON string
    json_data = json.dumps(test_data)
    
    # Return the JSON string
    return json_data

# Call the function and print the result
print(get_test_json())
