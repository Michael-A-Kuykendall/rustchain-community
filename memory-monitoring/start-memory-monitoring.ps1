# RustChain High-Capacity Test - Comprehensive Memory Monitoring Orchestrator
# Starts both GPU and system memory monitoring with synchronized logging

param(
    [string]$TestName = "RustChain-HighCapacity-Stress-Test",
    [int]$IntervalSeconds = 2,
    [string]$OutputDir = "memory-logs"
)

# Create output directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

$Timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$SessionId = "$TestName-$Timestamp"

$GPULogFile = "$OutputDir\gpu-memory-$SessionId.csv"
$SystemLogFile = "$OutputDir\system-memory-$SessionId.csv"
$TestLogFile = "$OutputDir\test-execution-$SessionId.log"

Write-Host "🚀 RustChain High-Capacity Memory Monitoring Started" -ForegroundColor Green -BackgroundColor Black
Write-Host "=" * 70 -ForegroundColor Green
Write-Host "📊 Test Session: $SessionId" -ForegroundColor Cyan
Write-Host "🎯 Output Directory: $OutputDir" -ForegroundColor Yellow
Write-Host "⏱️  Monitoring Interval: $IntervalSeconds seconds" -ForegroundColor Magenta
Write-Host "📝 Logs:" -ForegroundColor White
Write-Host "   - GPU Memory: $GPULogFile" -ForegroundColor Gray
Write-Host "   - System Memory: $SystemLogFile" -ForegroundColor Gray
Write-Host "   - Test Execution: $TestLogFile" -ForegroundColor Gray
Write-Host "=" * 70 -ForegroundColor Green

# Log test start
"$(Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff') | TEST_START | RustChain High-Capacity Stress Test Beginning" | Out-File -FilePath $TestLogFile -Encoding UTF8

Write-Host "🔋 Starting GPU Memory Monitor..." -ForegroundColor Green
$GPUMonitorJob = Start-Job -ScriptBlock {
    param($ScriptPath, $LogFile, $Interval)
    & $ScriptPath -LogFile $LogFile -IntervalSeconds $Interval
} -ArgumentList "$PSScriptRoot\gpu-memory-monitor.ps1", $GPULogFile, $IntervalSeconds

Write-Host "💾 Starting System Memory Monitor..." -ForegroundColor Green
$SystemMonitorJob = Start-Job -ScriptBlock {
    param($ScriptPath, $LogFile, $Interval)
    & $ScriptPath -LogFile $LogFile -IntervalSeconds $Interval
} -ArgumentList "$PSScriptRoot\system-memory-monitor.ps1", $SystemLogFile, $IntervalSeconds

# Wait a moment for monitors to initialize
Start-Sleep -Seconds 3

Write-Host "✅ Both monitors are now running in background" -ForegroundColor Green
Write-Host ""
Write-Host "🎮 READY FOR RUSTCHAIN EXECUTION" -ForegroundColor Green -BackgroundColor DarkGreen
Write-Host ""
Write-Host "📋 Next Steps:" -ForegroundColor Yellow
Write-Host "   1. Navigate to RustChain directory: cd C:\Users\micha\repos\rustchain-community" -ForegroundColor White
Write-Host "   2. Start mission validation and execution" -ForegroundColor White
Write-Host "   3. Monitor this window for memory alerts" -ForegroundColor White
Write-Host "   4. Press Ctrl+C here when testing is complete" -ForegroundColor White
Write-Host ""

# Mission execution suggestions
Write-Host "🚀 Suggested RustChain Execution Commands:" -ForegroundColor Cyan
Write-Host "   # Phase 1: Validate all missions" -ForegroundColor Gray
Write-Host "   Get-ChildItem missions\**\*.yaml | ForEach-Object { cargo run --bin rustchain -- mission validate `$_.FullName }" -ForegroundColor White
Write-Host ""
Write-Host "   # Phase 2: Execute missions sequentially" -ForegroundColor Gray  
Write-Host "   Get-ChildItem missions\planning\*.yaml | ForEach-Object { cargo run --bin rustchain --features llm -- run `$_.FullName }" -ForegroundColor White
Write-Host ""

# Real-time monitoring display
$StartTime = Get-Date
$MissionCount = 0

try {
    Write-Host "⏱️  Memory Monitoring Active - Real-Time Status:" -ForegroundColor Green
    Write-Host "=" * 50 -ForegroundColor Green
    
    while ($true) {
        # Check if jobs are still running
        $GPUStatus = $GPUMonitorJob.State
        $SystemStatus = $SystemMonitorJob.State
        
        if ($GPUStatus -ne "Running" -or $SystemStatus -ne "Running") {
            Write-Host "⚠️  One or more monitoring jobs stopped unexpectedly" -ForegroundColor Red
            Write-Host "   GPU Monitor: $GPUStatus" -ForegroundColor Yellow
            Write-Host "   System Monitor: $SystemStatus" -ForegroundColor Yellow
            break
        }
        
        # Display runtime info
        $Runtime = (Get-Date) - $StartTime
        $RuntimeStr = "{0:D2}:{1:D2}:{2:D2}" -f $Runtime.Hours, $Runtime.Minutes, $Runtime.Seconds
        
        Write-Host "⏱️  Runtime: $RuntimeStr | GPU Monitor: ✅ | System Monitor: ✅ | Press Ctrl+C to stop" -ForegroundColor Green
        
        # Log periodic status
        if ($Runtime.Minutes -gt 0 -and $Runtime.Seconds -eq 0) {
            "$(Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff') | MONITORING_STATUS | Runtime: $RuntimeStr | Both monitors active" | Out-File -FilePath $TestLogFile -Append -Encoding UTF8
        }
        
        Start-Sleep -Seconds 10
    }
}
catch {
    Write-Host "🛑 Monitoring interrupted by user or system" -ForegroundColor Yellow
}
finally {
    Write-Host "🛑 Stopping memory monitoring..." -ForegroundColor Yellow
    
    # Log test end
    "$(Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff') | TEST_END | Memory monitoring session completed" | Out-File -FilePath $TestLogFile -Append -Encoding UTF8
    
    # Stop monitoring jobs
    Stop-Job $GPUMonitorJob -ErrorAction SilentlyContinue
    Stop-Job $SystemMonitorJob -ErrorAction SilentlyContinue
    Remove-Job $GPUMonitorJob -ErrorAction SilentlyContinue  
    Remove-Job $SystemMonitorJob -ErrorAction SilentlyContinue
    
    $EndTime = Get-Date
    $TotalRuntime = $EndTime - $StartTime
    
    Write-Host "=" * 70 -ForegroundColor Green
    Write-Host "📊 Memory Monitoring Session Complete" -ForegroundColor Green -BackgroundColor Black
    Write-Host "⏱️  Total Runtime: $($TotalRuntime.Hours):$($TotalRuntime.Minutes):$($TotalRuntime.Seconds)" -ForegroundColor Cyan
    Write-Host "📝 Log Files Generated:" -ForegroundColor White
    Write-Host "   - GPU Memory: $GPULogFile" -ForegroundColor Gray
    Write-Host "   - System Memory: $SystemLogFile" -ForegroundColor Gray
    Write-Host "   - Test Execution: $TestLogFile" -ForegroundColor Gray
    Write-Host ""
    Write-Host "📈 Run memory analysis with:" -ForegroundColor Yellow
    Write-Host "   .\analyze-memory-logs.ps1 -LogDirectory `"$OutputDir`" -SessionId `"$SessionId`"" -ForegroundColor White
    Write-Host "=" * 70 -ForegroundColor Green
}