import json

test_data = [
    {
        "severity": "HIGH",
        "message": "This error blablabla",
        "errorCode": "REENTRY_ERROR",
        "lines": [5, 10],
    },
    {
        "severity": "LOW",
        "message": "This error OOOOOO WEEEEE!",
        "errorCode": "REENTRY_ERROR",
        "lines": [2, 10],
    },
    {
        "severity": "HIGH",
        "message": "This error wabba lubba dub dub!",
        "errorCode": "REENTRY_ERROR",
        "lines": [2, 10],
    }
]

# Print the test data as a JSON string
print(json.dumps(test_data))
