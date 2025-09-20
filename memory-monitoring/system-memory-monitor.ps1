# System Memory Monitoring Script for RustChain High-Capacity Test
# Monitors system RAM, process memory, and RustChain/Ollama specific usage

param(
    [string]$LogFile = "system-memory-log.csv",
    [int]$IntervalSeconds = 2,
    [string]$TestName = "RustChain-HighCapacity-Test"
)

# Create header for CSV log
$Header = "Timestamp,TestPhase,TotalRAM_GB,AvailableRAM_GB,UsedRAM_GB,UsedRAM_%,RustChain_MB,Ollama_MB,TotalProcesses,CPUUsage_%,PageFile_GB"
$Header | Out-File -FilePath $LogFile -Encoding UTF8

Write-Host "💾 System Memory Monitor Started" -ForegroundColor Green
Write-Host "📊 Logging to: $LogFile" -ForegroundColor Cyan
Write-Host "⏱️  Interval: $IntervalSeconds seconds" -ForegroundColor Yellow
Write-Host "🏁 Press Ctrl+C to stop monitoring" -ForegroundColor Magenta

try {
    while ($true) {
        $Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss.fff"
        
        # Get system memory info
        $Memory = Get-CimInstance -ClassName Win32_OperatingSystem
        $TotalRAM = [math]::Round($Memory.TotalVisibleMemorySize / 1MB, 2)
        $AvailableRAM = [math]::Round($Memory.FreePhysicalMemory / 1MB, 2)
        $UsedRAM = [math]::Round($TotalRAM - $AvailableRAM, 2)
        $UsedRAMPercent = [math]::Round(($UsedRAM / $TotalRAM) * 100, 1)
        
        # Get CPU usage
        $CPU = Get-CimInstance -ClassName Win32_Processor | Measure-Object -Property LoadPercentage -Average
        $CPUUsage = [math]::Round($CPU.Average, 1)
        
        # Get page file info
        $PageFile = Get-CimInstance -ClassName Win32_PageFileUsage
        $PageFileGB = if ($PageFile) { [math]::Round($PageFile.CurrentUsage / 1024, 2) } else { 0 }
        
        # Get RustChain process memory
        $RustChainProcesses = Get-Process | Where-Object { $_.ProcessName -like "*rustchain*" -or $_.ProcessName -like "*cargo*" }
        $RustChainMB = if ($RustChainProcesses) { 
            ($RustChainProcesses | Measure-Object -Property WorkingSet64 -Sum).Sum / 1MB 
        } else { 0 }
        $RustChainMB = [math]::Round($RustChainMB, 1)
        
        # Get Ollama process memory
        $OllamaProcesses = Get-Process | Where-Object { $_.ProcessName -like "*ollama*" }
        $OllamaMB = if ($OllamaProcesses) { 
            ($OllamaProcesses | Measure-Object -Property WorkingSet64 -Sum).Sum / 1MB 
        } else { 0 }
        $OllamaMB = [math]::Round($OllamaMB, 1)
        
        # Total process count
        $TotalProcesses = (Get-Process).Count
        
        # Determine test phase based on process activity
        $TestPhase = "Idle"
        if ($RustChainMB -gt 100) {
            $TestPhase = "RustChain-Active"
        }
        if ($OllamaMB -gt 500) {
            $TestPhase = "Model-Processing"
        }
        if ($RustChainMB -gt 100 -and $OllamaMB -gt 500) {
            $TestPhase = "Full-Processing"
        }
        
        # Log entry
        $LogEntry = "$Timestamp,$TestPhase,$TotalRAM,$AvailableRAM,$UsedRAM,$UsedRAMPercent,$RustChainMB,$OllamaMB,$TotalProcesses,$CPUUsage,$PageFileGB"
        $LogEntry | Out-File -FilePath $LogFile -Append -Encoding UTF8
        
        # Real-time display
        $Color = if ($UsedRAMPercent -gt 85) { "Red" } elseif ($UsedRAMPercent -gt 70) { "Yellow" } else { "Green" }
        Write-Host "$(Get-Date -Format "HH:mm:ss") | Phase: $TestPhase | RAM: $UsedRAM/$TotalRAM GB ($UsedRAMPercent%) | RustChain: $RustChainMB MB | Ollama: $OllamaMB MB | CPU: $CPUUsage%" -ForegroundColor $Color
        
        # Alerts
        if ($UsedRAMPercent -gt 90) {
            Write-Host "⚠️  HIGH SYSTEM MEMORY WARNING: $UsedRAMPercent%" -ForegroundColor Red -BackgroundColor Yellow
        }
        
        if ($RustChainMB -gt 2000) {
            Write-Host "🦀 RustChain High Memory Usage: $RustChainMB MB" -ForegroundColor Magenta
        }
        
        if ($OllamaMB -gt 4000) {
            Write-Host "🤖 Ollama High Memory Usage: $OllamaMB MB" -ForegroundColor Blue
        }
        
        Start-Sleep -Seconds $IntervalSeconds
    }
}
catch {
    Write-Host "❌ Monitoring stopped: $($_.Exception.Message)" -ForegroundColor Red
}
finally {
    Write-Host "📊 System memory monitoring log saved to: $LogFile" -ForegroundColor Green
}