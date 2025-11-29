import requests
import tomllib
from tqdm import tqdm
from concurrent.futures import ThreadPoolExecutor, as_completed

#sem = Semaphore(1)
def extract_code(url: str) -> str:
    try:
        return url.split("/beatmapsets/")[1].split("/")[0]
    except:
        raise ValueError("URL không phải beatmap osu hợp lệ")

def downloader(map_code, cookie):

    #url = f"https://osu.ppy.sh/beatmapsets/{map_code}/download"
    url = map_code
    beatmap_code = extract_code(url)
    #print(url)
    cookies = {
        "osu_session": cookie,
    }
    headers = {
        "User-Agent": "Mozilla/5.0",
        "Referer": "https://osu.ppy.sh/"
    }
    
    filename = f"{beatmap_code}.osz"
    resp = requests.get(url, headers=headers, cookies=cookies, stream=True)

    if resp.status_code != 200:
        print(f"Failed to download {beatmap_code}: {resp.status_code}")
        return
    #print("good status")
    total = int(resp.headers.get("Content-Length", 0))
    chunk_size = 1024 * 256  # 256KB
    with open(f"Map/{filename}", "wb") as f:
        #print("requesting...")
        pbar = tqdm(total=total, unit="B", unit_scale=True, desc=f"Downloading {map_code}")
        for chunk in resp.iter_content(chunk_size=chunk_size):
            if chunk:
                f.write(chunk)
                pbar.update(len(chunk))
        pbar.close()




#"osu_session" : "eyJpdiI6IjF2SmN4R0JBM1kwcUNyUmlYS0RqSlE9PSIsInZhbHVlIjoiNUY4ZncrK001RXNTWTlVSHBpZ3A5ZEVtc29KU2dXN082eFMvaUFaYnZBZlVwSUdPNVZMOUdjOG56N2pVTVZEMWNRQzU1OGZjamxxa0JlTSttS3F1ZU9ZekhmU0ZTSWpMUjZLVkZwZnlKZ21USDd0M0pLa2tzeTZUR2xjM1JhaDFBUlJhS040dDFzUjZuWTdyNWhYUzFRPT0iLCJtYWMiOiIyYjY4MTlmYzU2ZjA4YjJjMzlmZDVhODU2YTU3Nzc4ZjIzMzQ3M2Y4ODc3YmQ4NmUwOTVmNmI5MDUyODIzYTM3IiwidGFnIjoiIn0%3D",
def main():
    #osu_cook = "eyJpdiI6Im1PQXVocUZPUERmUDkwSU1IVXdzNGc9PSIsInZhbHVlIjoiZ1pJK1VNMnNnNmVqWExDZ3FTVUs3MG1ieVFsRGtEZExISVMyL2djOEhSemFLL1VreHB1U09UUkExaTIxbDZjVW12blNyKzVnTWg1dDNTYkF3dDJzY1J4dmliM1VjRGJXUnVUQnRvWVgzTld3M3hPV203cmlMUW9FUHpJUXF1ajJtdG9xc2dPMUVxcUZkYWRwcVhnQk5RPT0iLCJtYWMiOiJkNTEzNDFhMmU5ZmYxY2ExZGJhZThkM2U3NTg0YjkwZjNiYjA3YzNkNWM1OWU1Y2E5MDM3ODEwYjc0YTUzN2UxIiwidGFnIjoiIn0%3D"
    #g_map_code = ["663499","581729","591748"]
     
    with open("setting.toml", "rb") as t:
        data = tomllib.load(t)
    osu_cook = data["setting"]["cookies"]
    path = data["setting"]["input_p"]
    print(f"{osu_cook}, {path}")
    with open(path, "r") as f:
        print("loading beatmap file...")
        g_map_code = [line.strip() for line in f if line.strip()]
    print(g_map_code)
    print("beatmaps have loaded")
    #for code in g_map_code:
        #downloader(code, osu_cook)
    print("downloading..")
    with ThreadPoolExecutor(max_workers=4) as executor :
        futures = [executor.submit(downloader, code, osu_cook) for code in g_map_code]
        for fut in as_completed(futures):
            pass
if __name__ == "__main__":
    main()
