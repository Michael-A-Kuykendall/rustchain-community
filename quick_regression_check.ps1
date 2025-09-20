# Quick Regression Check - Essential Tests Only
# This runs the most critical tests to verify system stability

Write-Host "🧪 RUSTCHAIN QUICK REGRESSION CHECK" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan

$ErrorActionPreference = "Continue"
$testsFailed = 0

function Test-Command {
    param([string]$Description, [string]$Command)
    
    Write-Host "🔍 $Description" -ForegroundColor Yellow
    $startTime = Get-Date
    
    try {
        Invoke-Expression "$Command 2>&1 | Out-Null"
        if ($LASTEXITCODE -eq 0) {
            $duration = ((Get-Date) - $startTime).TotalSeconds
            Write-Host "   ✅ PASSED ($([math]::Round($duration, 1))s)" -ForegroundColor Green
            return $true
        } else {
            Write-Host "   ❌ FAILED" -ForegroundColor Red
            $script:testsFailed++
            return $false
        }
    }
    catch {
        Write-Host "   ❌ ERROR: $($_.Exception.Message)" -ForegroundColor Red
        $script:testsFailed++
        return $false
    }
}

# PHASE 1: Compilation Checks
Write-Host "`n📋 COMPILATION VERIFICATION" -ForegroundColor Magenta
Test-Command "Basic Compilation" "cargo check"
Test-Command "All Features Compilation" "cargo check --all-features"

# PHASE 2: Core Unit Tests  
Write-Host "`n🧪 CORE FUNCTIONALITY TESTS" -ForegroundColor Magenta
Test-Command "Engine Tests (Mission Execution)" "cargo test engine::tests --lib --all-features"
Test-Command "Document Loader Tests" "cargo test document_loaders::tests --lib --all-features"
Test-Command "Policy Engine Tests" "cargo test policy::tests --lib --all-features"
Test-Command "Tool System Tests" "cargo test tools::tests --lib --all-features"

# PHASE 3: Integration Verification
Write-Host "`n🔧 INTEGRATION TESTS" -ForegroundColor Magenta
Test-Command "CLI Tools Available" "cargo run --bin rustchain --features tools -- tools list"
Test-Command "Mission Validation Working" "cargo run --bin rustchain --features tools -- mission validate test_csv_integration.yaml"

# PHASE 4: Document Loader Integration
Write-Host "`n📊 DOCUMENT LOADER INTEGRATION" -ForegroundColor Magenta
Test-Command "CSV Loader Mission Integration" "cargo run --bin rustchain --features tools -- run test_csv_integration.yaml"

# Results Summary
Write-Host "`n📈 QUICK REGRESSION RESULTS" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan

if ($testsFailed -eq 0) {
    Write-Host "✅ ALL CRITICAL TESTS PASSED!" -ForegroundColor Green
    Write-Host "🎯 System is stable for continued development" -ForegroundColor Green
    Write-Host "🚀 Document loader integration verified" -ForegroundColor Green
    exit 0
} else {
    Write-Host "❌ $testsFailed CRITICAL TESTS FAILED!" -ForegroundColor Red
    Write-Host "🛑 Fix issues before proceeding with development" -ForegroundColor Red
    exit 1
}