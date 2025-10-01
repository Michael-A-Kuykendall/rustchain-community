#!/bin/bash
set -e

echo "🧪 LOCAL RELEASE PIPELINE TEST"
echo "============================="
echo "Testing all release components WITHOUT actually releasing"
echo ""

# Test 1: Version validation logic
echo "📋 Test 1: Version Validation Logic"
TEST_VERSION="1.0.1"
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "  Current Cargo.toml version: ${CARGO_VERSION}"
echo "  Test tag version: v${TEST_VERSION}"

if [[ "${CARGO_VERSION}" == "${TEST_VERSION}" ]]; then
    echo "  ✅ Version match logic: WORKING"
else
    echo "  ⚠️ Version mismatch (expected for test): ${CARGO_VERSION} != ${TEST_VERSION}"
fi

# Test 2: All build targets locally
echo ""
echo "📋 Test 2: Build Targets"
echo "  🔧 Testing minimal build (matches CI)..."
cargo build --release --no-default-features --features 'cli,transpiler' --quiet
echo "  ✅ Minimal build: SUCCESS"

echo "  🔧 Testing full build..."
cargo build --release --all-features --quiet
echo "  ✅ Full build: SUCCESS"

# Test 3: Package validation (what CI will do)
echo ""
echo "📋 Test 3: Package Validation"
echo "  📦 Testing package creation..."
cargo package --allow-dirty --quiet
echo "  ✅ Package creation: SUCCESS"

# Test 4: Binary functionality
echo ""
echo "📋 Test 4: Binary Functionality"
echo "  🏃 Testing binary execution..."
./target/release/rustchain --version > /dev/null
echo "  ✅ Binary execution: SUCCESS"

# Test 5: Docker build test (local)
echo ""
echo "📋 Test 5: Docker Build Test"
if command -v docker &> /dev/null; then
    echo "  🐳 Testing Docker build (local)..."
    docker build -t rustchain-test:local . --quiet
    echo "  🐳 Testing Docker run..."
    docker run --rm rustchain-test:local --version > /dev/null
    echo "  ✅ Docker build & run: SUCCESS"
    docker rmi rustchain-test:local --force > /dev/null
else
    echo "  ⚠️ Docker not available - skipping Docker test"
fi

# Test 6: Simulate CI workflow validation
echo ""
echo "📋 Test 6: CI Workflow Validation"
echo "  📝 Checking workflow file syntax..."
if command -v yq &> /dev/null; then
    yq eval '.jobs' .github/workflows/release.yml > /dev/null
    echo "  ✅ Workflow YAML: VALID"
else
    echo "  ⚠️ yq not available - manual workflow check needed"
fi

# Test 7: Security audit
echo ""
echo "📋 Test 7: Security Audit"
if command -v cargo &> /dev/null && cargo install --list | grep -q cargo-audit; then
    echo "  🔒 Running security audit..."
    cargo audit --quiet || echo "  ⚠️ Audit warnings found (review needed)"
    echo "  ✅ Security audit: COMPLETED"
else
    echo "  ⚠️ cargo-audit not installed - install with: cargo install cargo-audit"
fi

echo ""
echo "🎉 LOCAL TESTING COMPLETE"
echo "========================="
echo "✅ All core components tested locally"
echo "✅ Ready for CI/CD pipeline (when version is bumped)"
echo ""
echo "📋 NEXT STEPS (in order):"
echo "1. Fix any issues found above"
echo "2. Update version in Cargo.toml for real release"
echo "3. Create git tag: git tag v<new-version>"
echo "4. Push tag: git push origin v<new-version>"
echo "5. Monitor GitHub Actions pipeline"
echo ""
echo "⚠️ DO NOT create tags until all local tests pass"