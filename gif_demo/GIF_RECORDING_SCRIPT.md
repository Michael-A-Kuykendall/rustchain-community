# 🎬 GIF Recording Script - RustChain + Shimmy Demo

## Setup (before recording)
1. Open terminal in: `C:\Users\micha\repos\rustchain-community\gif_demo`
2. Make sure directory only has: `simple_demo.yaml`
3. Clear terminal screen: `cls` or `clear`

## Recording Script (follow exactly)

### Step 1: Show starting state (2 seconds)
```bash
echo "RustChain + Shimmy AI Demo"
echo "=========================="
dir
```

### Step 2: Run RustChain mission (5 seconds)
```bash
..\target\release\rustchain run simple_demo.yaml
```
*Wait for "Mission completed successfully!" message*

### Step 3: Show what was created (2 seconds)
```bash
echo "AI tool created! Testing it now..."
dir
```

### Step 4: Test the AI tool (8 seconds)
```bash
py chat_with_shimmy.py
```
*When prompted "Ask AI:", type: `hello`*
*Show the response (even if shimmy not available)*

### Step 5: Finish (3 seconds)
```bash
echo "Demo complete! RustChain built working AI tool."
echo "Get RustChain: github.com/Michael-A-Kuykendall/rustchain-community"
```

## Total: ~20 seconds

## Key points for GIF:
- **Fast execution**: RustChain builds tool instantly
- **Real functionality**: Creates actual Python AI chat client  
- **Ecosystem bridge**: Shows RustChain → Shimmy connection
- **Reproducible**: Anyone can run `simple_demo.yaml`

## Files to include in repo:
- `simple_demo.yaml` (the mission)
- `GIF_RECORDING_SCRIPT.md` (this script)
- Final GIF file