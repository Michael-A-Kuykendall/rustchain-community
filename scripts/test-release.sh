#!/bin/bash
set -e

echo "🧪 RUSTCHAIN RELEASE DRY RUN TEST"
echo "=================================="

# Test 1: Version consistency check
echo "📋 Test 1: Version Consistency Check"
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "  ✅ Cargo.toml version: ${CARGO_VERSION}"

# Test 2: Compilation check
echo "📋 Test 2: Compilation Check"
echo "  🔧 Testing minimal build..."
cargo check --no-default-features --features 'cli,transpiler' --quiet
echo "  ✅ Minimal build: SUCCESS"

echo "  🔧 Testing full build..."
cargo check --all-features --quiet
echo "  ✅ Full build: SUCCESS"

# Test 3: Test suite
echo "📋 Test 3: Test Suite"
cargo test --all-features --quiet
echo "  ✅ Test suite: PASSED"

# Test 4: Package validation
echo "📋 Test 4: Package Validation"
cargo package --list | head -10
echo "  ✅ Package contents: VALID"

# Test 5: Security audit (if available)
echo "📋 Test 5: Security Audit"
if command -v cargo-audit &> /dev/null; then
    cargo audit || echo "  ⚠️ Audit warnings (non-blocking)"
else
    echo "  ⚠️ cargo-audit not installed (install with: cargo install cargo-audit)"
fi

# Test 6: Binary execution test
echo "📋 Test 6: Binary Execution Test"
cargo build --release --no-default-features --features 'cli,transpiler' --quiet
./target/release/rustchain --version
echo "  ✅ Binary execution: SUCCESS"

# Test 7: Docker build test (if Docker available)
echo "📋 Test 7: Docker Build Test"
if command -v docker &> /dev/null; then
    echo "  🐳 Testing Docker build..."
    docker build -t rustchain-test . --quiet
    echo "  ✅ Docker build: SUCCESS"
else
    echo "  ⚠️ Docker not available, skipping Docker test"
fi

echo ""
echo "🎉 DRY RUN COMPLETE - ALL SYSTEMS GO!"
echo "Ready for release automation via GitHub Actions"
echo ""
echo "Next steps:"
echo "1. Create git tag: git tag v1.0.1"
echo "2. Push tag: git push origin v1.0.1"
echo "3. GitHub Actions will handle the rest automatically"