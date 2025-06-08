import requests
import json

with open('example/example.usc', 'r') as f1, open('example/example2.usc', 'r') as f2:
    usc1 = json.load(f1)
    usc2 = json.load(f2)

response = requests.post('http://localhost:3030/merge', json=[usc1, usc2])

if response.status_code == 200:
    merged_usc = response.json()
    with open('merged.usc', 'w') as f:
        json.dump(merged_usc, f, indent=2)
    print("success")
else:
    print(f"error: {response.status_code}")