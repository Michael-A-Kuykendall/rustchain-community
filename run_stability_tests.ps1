#!/usr/bin/env pwsh
# RustChain Comprehensive Stability Test Runner
# Uses invariant-based testing for full coverage and stability verification

param(
    [switch]$Coverage,
    [switch]$Verbose,
    [switch]$NoCleanup,
    [string]$TestFilter = "",
    [int]$Iterations = 1
)

Write-Host "🚀 RustChain Comprehensive Stability Test Runner" -ForegroundColor Cyan
Write-Host "===============================================" -ForegroundColor Cyan

$ErrorActionPreference = "Stop"
$StartTime = Get-Date

# Ensure we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Error "Must be run from RustChain root directory"
    exit 1
}

Write-Host "📊 Test Configuration:" -ForegroundColor Yellow
Write-Host "  Coverage Analysis: $Coverage"
Write-Host "  Verbose Output: $Verbose"
Write-Host "  Test Filter: $(if ($TestFilter) { $TestFilter } else { 'None' })"
Write-Host "  Iterations: $Iterations"
Write-Host ""

# Function to run tests with proper error handling
function Invoke-TestSuite {
    param(
        [string]$SuiteName,
        [string]$TestCommand,
        [string]$Description
    )
    
    Write-Host "🧪 Running $SuiteName" -ForegroundColor Green
    Write-Host "   $Description" -ForegroundColor Gray
    
    try {
        $output = Invoke-Expression $TestCommand 2>&1
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "   ✅ $SuiteName PASSED" -ForegroundColor Green
            
            if ($Verbose) {
                Write-Host "   Output:" -ForegroundColor Gray
                $output | ForEach-Object { Write-Host "     $_" -ForegroundColor DarkGray }
            }
            
            return $true
        } else {
            Write-Host "   ❌ $SuiteName FAILED (Exit Code: $LASTEXITCODE)" -ForegroundColor Red
            Write-Host "   Output:" -ForegroundColor Red
            $output | ForEach-Object { Write-Host "     $_" -ForegroundColor Red }
            return $false
        }
    }
    catch {
        Write-Host "   💥 $SuiteName CRASHED: $($_.Exception.Message)" -ForegroundColor Magenta
        return $false
    }
}

# Initialize results tracking
$TestResults = @{
    TotalSuites = 0
    PassedSuites = 0
    FailedSuites = 0
    CrashedSuites = 0
    TestDetails = @()
}

Write-Host "🏗️ Building RustChain with all features..." -ForegroundColor Cyan
$buildResult = Invoke-TestSuite -SuiteName "Build Check" -TestCommand "cargo build --all-features" -Description "Verify compilation with all features"
if (-not $buildResult) {
    Write-Error "Build failed - cannot proceed with tests"
    exit 1
}

Write-Host ""
Write-Host "🧪 Running Test Suites..." -ForegroundColor Cyan
Write-Host "========================" -ForegroundColor Cyan

# Test Suite Definitions
$TestSuites = @(
    @{
        Name = "Unit Tests"
        Command = "cargo test --lib --all-features $(if ($Verbose) { '--' '--nocapture' } else { '' })"
        Description = "Core unit tests with all features"
    },
    @{
        Name = "Integration Tests (Basic)"
        Command = "cargo test --test integration_invariant_tests --all-features $(if ($Verbose) { '--' '--nocapture' } else { '' })"
        Description = "Basic integration tests with invariant checking"
    },
    @{
        Name = "Comprehensive Stability Tests"
        Command = "cargo test --test comprehensive_stability_tests --all-features $(if ($Verbose) { '--' '--nocapture' } else { '' })"
        Description = "Full end-to-end stability testing with invariant system"
    },
    @{
        Name = "Regression Tests"
        Command = "cargo test --test regression_tests --all-features $(if ($Verbose) { '--' '--nocapture' } else { '' })"
        Description = "Regression test suite for critical functionality"
    },
    @{
        Name = "Document Loader Tests"
        Command = "cargo test document_loader --all-features $(if ($Verbose) { '--' '--nocapture' } else { '' })"
        Description = "CSV, JSON, HTML document loader comprehensive tests"
    },
    @{
        Name = "Tool System Tests"
        Command = "cargo test tool --all-features $(if ($Verbose) { '--' '--nocapture' } else { '' })"
        Description = "Tool registry, bridge, and execution tests"
    },
    @{
        Name = "Mission System Tests"
        Command = "cargo test mission --all-features $(if ($Verbose) { '--' '--nocapture' } else { '' })"
        Description = "Mission loading, parsing, and execution tests"
    },
    @{
        Name = "Safety & Policy Tests"
        Command = "cargo test -p rustchain-community safety policy --all-features $(if ($Verbose) { '--' '--nocapture' } else { '' })"
        Description = "Safety validation and policy engine tests"
    },
    @{
        Name = "Build Dashboard Tests"
        Command = "cargo test build_dashboard --all-features $(if ($Verbose) { '--' '--nocapture' } else { '' })"
        Description = "Build dashboard and system health tests"
    }
)

# Run test iterations
for ($iteration = 1; $iteration -le $Iterations; $iteration++) {
    if ($Iterations -gt 1) {
        Write-Host ""
        Write-Host "🔄 Test Iteration $iteration of $Iterations" -ForegroundColor Magenta
        Write-Host "================================" -ForegroundColor Magenta
    }

    foreach ($suite in $TestSuites) {
        # Apply test filter if specified
        if ($TestFilter -and $suite.Name -notlike "*$TestFilter*") {
            continue
        }
        
        $TestResults.TotalSuites++
        
        $suiteResult = Invoke-TestSuite -SuiteName $suite.Name -TestCommand $suite.Command -Description $suite.Description
        
        $TestResults.TestDetails += @{
            Name = $suite.Name
            Iteration = $iteration
            Passed = $suiteResult
            Timestamp = Get-Date
        }
        
        if ($suiteResult) {
            $TestResults.PassedSuites++
        } else {
            $TestResults.FailedSuites++
        }
        
        Write-Host ""
    }
}

# Coverage Analysis (if requested)
if ($Coverage) {
    Write-Host "📊 Running Coverage Analysis..." -ForegroundColor Cyan
    
    if (Get-Command "cargo-tarpaulin" -ErrorAction SilentlyContinue) {
        Write-Host "   Using cargo-tarpaulin for coverage analysis" -ForegroundColor Gray
        $coverageResult = Invoke-TestSuite -SuiteName "Coverage Analysis" -TestCommand "cargo tarpaulin --all-features --out Html --output-dir coverage-report" -Description "Generate code coverage report"
        
        if ($coverageResult -and (Test-Path "coverage-report")) {
            Write-Host "   📈 Coverage report generated in: coverage-report/" -ForegroundColor Green
        }
    } else {
        Write-Host "   ⚠️  cargo-tarpaulin not installed - skipping coverage analysis" -ForegroundColor Yellow
        Write-Host "   Install with: cargo install cargo-tarpaulin" -ForegroundColor Gray
    }
}

# Generate final report
Write-Host ""
Write-Host "📊 FINAL TEST REPORT" -ForegroundColor Cyan
Write-Host "===================" -ForegroundColor Cyan

$TotalTime = (Get-Date) - $StartTime
$SuccessRate = if ($TestResults.TotalSuites -gt 0) { 
    [math]::Round(($TestResults.PassedSuites / $TestResults.TotalSuites) * 100, 1) 
} else { 
    0 
}

Write-Host "📈 Summary Statistics:" -ForegroundColor White
Write-Host "   Total Test Suites: $($TestResults.TotalSuites)" -ForegroundColor Gray
Write-Host "   Passed: $($TestResults.PassedSuites) ✅" -ForegroundColor Green
Write-Host "   Failed: $($TestResults.FailedSuites) ❌" -ForegroundColor Red
Write-Host "   Success Rate: $SuccessRate%" -ForegroundColor $(if ($SuccessRate -eq 100) { "Green" } elseif ($SuccessRate -ge 80) { "Yellow" } else { "Red" })
Write-Host "   Total Runtime: $($TotalTime.ToString('mm\:ss'))" -ForegroundColor Gray

Write-Host ""
Write-Host "📋 Detailed Results:" -ForegroundColor White
foreach ($detail in $TestResults.TestDetails) {
    $status = if ($detail.Passed) { "✅" } else { "❌" }
    $iteration = if ($Iterations -gt 1) { " (Iteration $($detail.Iteration))" } else { "" }
    Write-Host "   $status $($detail.Name)$iteration" -ForegroundColor $(if ($detail.Passed) { "Green" } else { "Red" })
}

# RustChain Build Dashboard Update
Write-Host ""
Write-Host "🏗️ Updating RustChain Build Dashboard..." -ForegroundColor Cyan
try {
    & cargo run --bin rustchain -- build update
    Write-Host "   ✅ Build dashboard updated successfully" -ForegroundColor Green
} catch {
    Write-Host "   ⚠️  Could not update build dashboard: $($_.Exception.Message)" -ForegroundColor Yellow
}

# Cleanup (unless disabled)
if (-not $NoCleanup) {
    Write-Host ""
    Write-Host "🧹 Cleaning up temporary test files..." -ForegroundColor Gray
    
    # Remove temporary test files
    Get-ChildItem -Path "." -Filter "temp_*.json" -ErrorAction SilentlyContinue | Remove-Item -Force
    Get-ChildItem -Path "." -Filter "*_test.txt" -ErrorAction SilentlyContinue | Remove-Item -Force
    Get-ChildItem -Path "." -Filter "test_*.json" -ErrorAction SilentlyContinue | Remove-Item -Force
    
    Write-Host "   ✅ Cleanup completed" -ForegroundColor Green
}

# Final status and exit
Write-Host ""
if ($TestResults.FailedSuites -eq 0) {
    Write-Host "🎉 ALL TESTS PASSED! RustChain is stable and ready." -ForegroundColor Green
    Write-Host "   Full system verification complete with invariant coverage." -ForegroundColor Green
    exit 0
} else {
    Write-Host "⚠️  SOME TESTS FAILED! Review the results above." -ForegroundColor Red
    Write-Host "   $($TestResults.FailedSuites) out of $($TestResults.TotalSuites) test suites failed." -ForegroundColor Red
    exit 1
}