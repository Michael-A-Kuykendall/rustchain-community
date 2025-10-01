#!/bin/bash
set -e

echo "⚡ QUICK RELEASE VALIDATION"
echo "=========================="

# Quick essentials only
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "📋 Version: ${CARGO_VERSION}"

cargo check --no-default-features --features 'cli,transpiler' --quiet
echo "✅ Core build: PASSED"

cargo package --list > /dev/null
echo "✅ Package: VALID"

cargo build --release --no-default-features --features 'cli,transpiler' --quiet
./target/release/rustchain --version
echo "✅ Binary: WORKING"

echo ""
echo "🚀 RELEASE READY - All critical checks passed!"
echo "Current version: ${CARGO_VERSION}"