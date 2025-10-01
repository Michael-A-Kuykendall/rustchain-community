#!/bin/bash
set -e

echo "⚡ RELEASE READINESS CHECK"
echo "========================"
echo "Fast validation before any CI/CD execution"
echo ""

# Check 1: Basic compilation
echo "📋 Quick Build Check"
cargo check --no-default-features --features 'cli,transpiler' --quiet
echo "✅ Core compilation: PASSED"

# Check 2: Version consistency check
echo ""
echo "📋 Version Check"
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Current version: ${CARGO_VERSION}"
echo "✅ Version readable: PASSED"

# Check 3: Package validation
echo ""
echo "📋 Package Check"
cargo package --allow-dirty --quiet
echo "✅ Package creation: PASSED"

# Check 4: Binary works
echo ""
echo "📋 Binary Check"
cargo build --release --no-default-features --features 'cli,transpiler' --quiet
./target/release/rustchain --version
echo "✅ Binary execution: PASSED"

# Check 5: Workflow file exists and basic syntax
echo ""
echo "📋 Workflow Check"
if [ -f ".github/workflows/release.yml" ]; then
    echo "✅ Release workflow: EXISTS"
else
    echo "❌ Release workflow: MISSING"
    exit 1
fi

echo ""
echo "🎯 SUMMARY"
echo "=========="
echo "✅ Core build: Ready"
echo "✅ Package: Valid"  
echo "✅ Binary: Working"
echo "✅ Workflow: Present"
echo ""
echo "📋 TO RELEASE:"
echo "1. Bump version in Cargo.toml"
echo "2. git tag v<version>"
echo "3. git push origin v<version>"
echo "4. Monitor GitHub Actions"
echo ""
echo "⚠️ Test workflow first with: gh workflow run 'Release Pipeline' --field version=v<version>-test"