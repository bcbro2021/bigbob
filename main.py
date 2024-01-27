import requests
from bs4 import BeautifulSoup

def google_search(query):
    responses = {}
    try:
        base_url = "https://www.google.com/search"
        params = {'q': query}
        
        headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
        }

        response = requests.get(base_url, params=params, headers=headers)
        
        if response.status_code == 200:
            soup = BeautifulSoup(response.text, 'html.parser')
            results = soup.find_all('div', class_='tF2Cxc')
            
            if len(results) >= 1:
                for result in results:
                    title = result.find('h3').get_text()
                    link = result.find('a')['href']

                    # Check if the snippet is present
                    snippet_element = result.find(class_='IsZvec')
                    snippet = snippet_element.get_text() if snippet_element else "No snippet available"

                    responses[title] = link
            else:
                responses["Error"] = "Failed to get a response..."
        else:
            responses["Error"] = "Failed to get a response..."
    except requests.exceptions.ConnectionError:
        responses["Error"] = "Connection Failed.. Check your Internet!"

    return responses

if __name__ == "__main__":
    response = None
    with open(".control/query.tst", "r") as file:
        data = file.readlines()
        query = data[0].strip()
        response = google_search(query)

    with open(".control/response.tst", "w") as file:
        full_text = ""
        for key, val in response.items():
            full_text = full_text + f"\u001b[32m{key} \u001b[33m: \u001b[34m{val}\n"
        file.write(full_text)