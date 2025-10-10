import requests
import concurrent.futures


input = globals()["input"]
response = requests.get("https://services.app.aplines.com/ars/assets/sandbox", headers={"X-CLIENT-CERT": input["SECRET"], "Content-Type": "application/json"})
references = response.json()["data"]["asset_references"]
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as executor:
    futures = [executor.submit(requests.get, f"https://services.app.aplines.com/ars/assets/sandbox/{reference}", headers={"X-CLIENT-CERT": input["SECRET"], "Content-Type": "application/json"}) for reference in references]
    results = [future.result() for future in concurrent.futures.as_completed(futures)]
    assets = [result.json()["data"] for result in results]

result = assets

