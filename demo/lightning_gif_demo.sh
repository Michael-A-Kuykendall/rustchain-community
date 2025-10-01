#!/bin/bash
# 🎬 LIGHTNING GIF DEMO - Ready for recording!
# This builds and demonstrates a working AI tool in ~20 seconds

set -e
clear

echo "🦀 RustChain Community Edition"
echo "==============================="
echo "⚡ Building AI tool in real-time..."
echo ""
sleep 1

# Phase 1: Build the AI tool using RustChain (5-8 seconds)
echo "🔧 RustChain Mission: Creating AI chat tool..."
cargo run --release --bin rustchain --features 'cli,llm' -- run demo/build_ai_tool.yaml --quiet

echo "✅ AI tool built successfully!"
echo ""
sleep 1

# Phase 2: Demonstrate the working tool (10-12 seconds)
echo "🚀 Demonstrating the AI tool..."
echo "================================"
sleep 0.5

# Run the demo
./demo_ai_tool.sh

echo ""
echo "🎯 DEMO COMPLETE!"
echo "=================="
echo "✅ RustChain built a working AI tool"
echo "⚡ Total time: ~20 seconds"
echo "🔗 github.com/Michael-A-Kuykendall/rustchain-community"
echo ""
echo "📦 cargo install rustchain-community"