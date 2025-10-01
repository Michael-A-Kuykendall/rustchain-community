#!/bin/bash
set -e

echo "🚀 Building Rustchain cross-platform binaries..."

# Create output directory
mkdir -p dist

# Function to build for a specific target
build_target() {
    local target=$1
    local platform=$2
    local arch=$3
    
    echo "🔧 Building for $target..."
    
    docker run --rm \
        -v "$(pwd):/workspace" \
        -w /workspace \
        --platform="$platform" \
        rust:1.70 \
        bash -c "
            apt-get update && apt-get install -y gcc-multilib || true
            rustup target add $target
            cargo build --release --target $target --verbose
        "
    
    # Copy binary to dist
    if [[ "$target" == *"windows"* ]]; then
        cp "target/$target/release/rustchain.exe" "dist/rustchain-$arch.exe"
    else
        cp "target/$target/release/rustchain" "dist/rustchain-$arch"
    fi
    
    echo "✅ Built rustchain-$arch"
}

# Build for different platforms
echo "Building Linux x86_64..."
build_target "x86_64-unknown-linux-gnu" "linux/amd64" "linux-x64"

echo "Building Linux ARM64..."
build_target "aarch64-unknown-linux-gnu" "linux/arm64" "linux-arm64"

echo "🎉 Cross-platform build complete!"
echo "📦 Binaries available in dist/ directory:"
ls -la dist/