#!/bin/bash
# RustChain Website CI/CD Trigger Script
# This script should be called whenever you want to update the website

set -e

echo "🚀 Starting RustChain Website CI/CD Pipeline..."

# Get current RustChain version from Cargo.toml
RUSTCHAIN_VERSION=$(grep "^version = " Cargo.toml | cut -d'"' -f2)
COMMIT_HASH=$(git rev-parse --short HEAD)
CURRENT_TIMESTAMP=$(date -u +"%Y-%m-%d %H:%M:%S UTC")

echo "📦 RustChain Version: $RUSTCHAIN_VERSION"
echo "📝 Commit Hash: $COMMIT_HASH"
echo "🕐 Timestamp: $CURRENT_TIMESTAMP"

# Set environment variables for the mission
export RUSTCHAIN_VERSION="$RUSTCHAIN_VERSION"
export COMMIT_HASH="$COMMIT_HASH"
export CURRENT_TIMESTAMP="$CURRENT_TIMESTAMP"

# Run the website update mission
echo "🎯 Executing website update mission..."
./target/debug/rustchain run missions/website_update.yaml \
  --variable "rustchain_version=$RUSTCHAIN_VERSION" \
  --variable "commit_hash=$COMMIT_HASH" \
  --variable "current_timestamp=$CURRENT_TIMESTAMP"

echo "✅ Website CI/CD Pipeline Completed!"
echo "🌐 Your website should now be updated and deployed automatically."
echo ""
echo "To manually check the website locally:"
echo "  cd website && npm run dev"
echo "  Open: http://localhost:8080"