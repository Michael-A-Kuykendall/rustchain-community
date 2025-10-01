#!/usr/bin/env python3
import requests
import json
import sys

def check_shimmy_health():
    try:
        r = requests.get("http://localhost:11435/health", timeout=2)
        return r.status_code == 200
    except requests.RequestException:
        return False

def chat_with_llm(user_input):
    chat_payload = {
        "model": "llama3.2:1b", 
        "messages": [{"role": "user", "content": user_input}],
        "max_tokens": 60
    }
    
    response = requests.post("http://localhost:11435/v1/chat/completions", 
                           json=chat_payload, timeout=15)
    response.raise_for_status()
    return response.json()["choices"][0]["message"]["content"]

def run_demo():
    print("RustChain + Shimmy Integration Demo")
    print("=" * 35)
    
    if not check_shimmy_health():
        print("Cannot reach Shimmy at localhost:11435")
        print("Install: github.com/Michael-A-Kuykendall/shimmy")
        sys.exit(1)
    
    print("Shimmy connection established")
    
    user_query = input("\nYour question: ")
    if not user_query.strip():
        print("No input provided")
        return
        
    try:
        llm_output = chat_with_llm(user_query)
        print(f"\nResponse: {llm_output}")
    except requests.HTTPError as http_err:
        print(f"HTTP error: {http_err}")
    except Exception as err:
        print(f"Unexpected error: {err}")
    
    print("\n[Demo orchestrated by RustChain mission engine]")

if __name__ == "__main__":
    run_demo()