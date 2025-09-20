#!/bin/bash
# PUNCH Auto-Updater Mission Test Script
# Tests all 6 auto-updater missions for PUNCH family tools

set -euo pipefail

echo "=== PUNCH Auto-Updater Mission Test Suite ==="
echo "Testing all 6 mission templates for PUNCH family maintenance"
echo ""

MISSIONS_DIR="missions"
RUSTCHAIN_CMD="cargo run --bin rustchain --features llm --"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Test function
test_mission() {
    local mission_file=$1
    local description=$2
    
    echo -e "${YELLOW}Testing: ${description}${NC}"
    echo "Mission file: $mission_file"
    
    # Validate mission syntax
    echo "  1. Validating mission syntax..."
    if $RUSTCHAIN_CMD mission validate "$mission_file"; then
        echo -e "  ${GREEN}✅ Validation passed${NC}"
    else
        echo -e "  ${RED}❌ Validation failed${NC}"
        return 1
    fi
    
    # Optional: Run dry-run if available
    echo "  2. Checking mission info..."
    $RUSTCHAIN_CMD mission info "$mission_file"
    
    echo -e "  ${GREEN}✅ Mission test completed${NC}"
    echo ""
}

echo "=== 1. Security Update Mission ==="
test_mission "$MISSIONS_DIR/security_update.yaml" "AI-powered security vulnerability analysis and patch generation"

echo "=== 2. Compatibility Update Mission ==="
test_mission "$MISSIONS_DIR/compatibility_update.yaml" "AI-powered dependency compatibility analysis and version management"

echo "=== 3. Feature Discovery Mission ==="
test_mission "$MISSIONS_DIR/feature_discovery.yaml" "AI-powered new feature detection and enhancement planning"

echo "=== 4. Performance Update Mission ==="
test_mission "$MISSIONS_DIR/performance_update.yaml" "AI-powered performance optimization analysis and implementation"

echo "=== 5. Breaking Change Analysis Mission ==="
test_mission "$MISSIONS_DIR/breaking_change_analysis.yaml" "AI-powered breaking change impact assessment and migration planning"

echo "=== 6. Rollback Mission ==="
test_mission "$MISSIONS_DIR/rollback_mission.yaml" "AI-powered automated rollback procedures and incident response"

echo "=== PUNCH Auto-Updater Mission Test Suite Complete ==="
echo ""
echo "NEXT STEPS:"
echo "1. Start Ollama service: ollama serve"
echo "2. Pull Champion model: ollama pull llama32-champion:latest"
echo "3. Run individual missions: $RUSTCHAIN_CMD run missions/[mission_name].yaml"
echo ""
echo "MISSION USAGE EXAMPLES:"
echo "# Security analysis"
echo "$RUSTCHAIN_CMD run missions/security_update.yaml"
echo ""
echo "# Feature discovery"
echo "$RUSTCHAIN_CMD run missions/feature_discovery.yaml"
echo ""
echo "# Performance optimization"
echo "$RUSTCHAIN_CMD run missions/performance_update.yaml"
echo ""
echo "All missions generate output in the 'output/' directory with:"
echo "- Detailed analysis reports (Markdown format)"
echo "- Implementation plans and roadmaps"
echo "- Git workflow commands and automation scripts"
echo "- Rollback and recovery procedures"