# RustChain Comprehensive Regression Test Runner
# This script must pass before any new feature development proceeds

param(
    [switch]$Verbose,
    [switch]$FailFast = $true,
    [string]$TestFilter = ""
)

Write-Host "🧪 RUSTCHAIN REGRESSION TEST SUITE" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan

$ErrorActionPreference = "Continue"
$testsFailed = 0
$testsTotal = 0

function Run-Test {
    param([string]$TestName, [string]$Command, [string]$Description)
    
    $testsTotal++
    Write-Host "🔍 Running: $TestName" -ForegroundColor Yellow
    Write-Host "   $Description" -ForegroundColor Gray
    
    $startTime = Get-Date
    try {
        if ($Verbose) {
            Invoke-Expression $Command
        } else {
            Invoke-Expression "$Command 2>&1 | Out-Null"
        }
        
        if ($LASTEXITCODE -eq 0) {
            $duration = ((Get-Date) - $startTime).TotalSeconds
            Write-Host "   ✅ PASSED ($([math]::Round($duration, 2))s)" -ForegroundColor Green
            return $true
        } else {
            Write-Host "   ❌ FAILED (Exit code: $LASTEXITCODE)" -ForegroundColor Red
            $testsFailed++
            if ($FailFast) {
                Write-Host "🚨 FAIL FAST ENABLED - STOPPING ON FIRST FAILURE" -ForegroundColor Red
                exit 1
            }
            return $false
        }
    }
    catch {
        Write-Host "   ❌ EXCEPTION: $($_.Exception.Message)" -ForegroundColor Red
        $testsFailed++
        if ($FailFast) {
            Write-Host "🚨 FAIL FAST ENABLED - STOPPING ON FIRST FAILURE" -ForegroundColor Red
            exit 1
        }
        return $false
    }
}

Write-Host "`n📋 PHASE 1: COMPILATION CHECKS" -ForegroundColor Magenta
Write-Host "==============================" -ForegroundColor Magenta

Run-Test "Basic Compilation" "cargo check" "Verify codebase compiles without errors"
Run-Test "Full Feature Compilation" "cargo check --all-features" "Verify all features compile together"
Run-Test "Release Compilation" "cargo check --release" "Verify release build compiles"

Write-Host "`n🧪 PHASE 2: UNIT TESTS" -ForegroundColor Magenta  
Write-Host "=====================" -ForegroundColor Magenta

Run-Test "Core Unit Tests" "cargo test --lib --all-features" "Run all library unit tests"
Run-Test "Integration Tests" "cargo test --test test_suite --all-features" "Run existing integration tests"
Run-Test "Regression Tests" "cargo test --test regression_tests --all-features" "Run comprehensive regression tests"

Write-Host "`n🔧 PHASE 3: TOOL INTEGRATION TESTS" -ForegroundColor Magenta
Write-Host "===================================" -ForegroundColor Magenta

Run-Test "CLI Tools List" "cargo run --bin rustchain --features tools -- tools list" "Verify CLI tools are accessible"
Run-Test "Mission Validation" "cargo run --bin rustchain --features tools -- mission validate test_csv_integration.yaml" "Verify mission validation works"

Write-Host "`n📊 PHASE 4: DOCUMENT LOADER INTEGRATION" -ForegroundColor Magenta
Write-Host "=========================================" -ForegroundColor Magenta

Run-Test "CSV Loader Mission" "cargo run --bin rustchain --features tools -- run test_csv_integration.yaml" "Verify CSV loader works in missions"
Run-Test "YAML Loader Mission" "cargo run --bin rustchain --features tools -- run test_json_yaml_integration.yaml" "Verify YAML loader works in missions"
Run-Test "HTML Loader Mission" "cargo run --bin rustchain --features tools -- run test_html_integration.yaml" "Verify HTML loader works in missions"

Write-Host "`n🎯 PHASE 5: COMPREHENSIVE WORKFLOW TEST" -ForegroundColor Magenta
Write-Host "=======================================" -ForegroundColor Magenta

# Create a comprehensive test mission
$comprehensiveMission = @"
name: "Comprehensive Regression Test"
description: "Test multiple RustChain features together"
version: "1.0"

steps:
  - id: "test_csv_load"
    name: "Test CSV Loading"
    step_type: "tool"
    parameters:
      tool: "csv_loader"
      parameters:
        file_path: "test_integration.csv"
        delimiter: ","
        has_headers: true
    timeout_seconds: 30

  - id: "test_file_create"
    name: "Test File Creation"
    step_type: "create_file"
    parameters:
      path: "regression_test_output.txt"
      content: "Comprehensive regression test completed successfully!"
    depends_on:
      - "test_csv_load"
    timeout_seconds: 30

config:
  max_parallel_steps: 1
  fail_fast: true
  timeout_seconds: 120
"@

Set-Content -Path "comprehensive_regression_test.yaml" -Value $comprehensiveMission
Run-Test "Comprehensive Mission" "cargo run --bin rustchain --features tools -- run comprehensive_regression_test.yaml" "Execute comprehensive multi-feature mission"

Write-Host "`n📈 REGRESSION TEST RESULTS SUMMARY" -ForegroundColor Cyan
Write-Host "===================================" -ForegroundColor Cyan

$testsPass = $testsTotal - $testsFailed
$passRate = if ($testsTotal -gt 0) { ($testsPass / $testsTotal) * 100 } else { 0 }

Write-Host "Total Tests: $testsTotal" -ForegroundColor White
Write-Host "Passed:      $testsPass" -ForegroundColor Green  
Write-Host "Failed:      $testsFailed" -ForegroundColor Red
Write-Host "Pass Rate:   $([math]::Round($passRate, 1))%" -ForegroundColor $(if ($passRate -eq 100) { 'Green' } else { 'Yellow' })

if ($testsFailed -eq 0) {
    Write-Host "`n🎉 ALL REGRESSION TESTS PASSED!" -ForegroundColor Green
    Write-Host "✅ System is stable and ready for new development" -ForegroundColor Green
    Write-Host "✅ Document loaders integration verified" -ForegroundColor Green
    Write-Host "✅ No regressions detected in existing functionality" -ForegroundColor Green
    exit 0
} else {
    Write-Host "`n🚨 REGRESSION TESTS FAILED!" -ForegroundColor Red
    Write-Host "❌ $testsFailed test(s) failed out of $testsTotal total" -ForegroundColor Red
    Write-Host "❌ Fix failing tests before proceeding with development" -ForegroundColor Red
    exit 1
}