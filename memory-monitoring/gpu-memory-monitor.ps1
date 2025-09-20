# GPU Memory Monitoring Script for RustChain High-Capacity Test
# Monitors NVIDIA GPU memory usage during mission execution

param(
    [string]$LogFile = "gpu-memory-log.csv",
    [int]$IntervalSeconds = 2,
    [string]$TestName = "RustChain-HighCapacity-Test"
)

# Create header for CSV log
$Header = "Timestamp,TestPhase,GPUUtilization%,MemoryUsed_MB,MemoryTotal_MB,MemoryFree_MB,Temperature_C,ProcessCount,PowerDraw_W,ClockSM_MHz,ClockMem_MHz"
$Header | Out-File -FilePath $LogFile -Encoding UTF8

Write-Host "🔋 GPU Memory Monitor Started" -ForegroundColor Green
Write-Host "📊 Logging to: $LogFile" -ForegroundColor Cyan
Write-Host "⏱️  Interval: $IntervalSeconds seconds" -ForegroundColor Yellow
Write-Host "🏁 Press Ctrl+C to stop monitoring" -ForegroundColor Magenta

$TestPhase = "Initialization"

try {
    while ($true) {
        $Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss.fff"
        
        # Get GPU metrics using nvidia-smi
        $GPUInfo = nvidia-smi --query-gpu=utilization.gpu,memory.used,memory.total,memory.free,temperature.gpu,count,power.draw,clocks.sm,clocks.mem --format=csv,noheader,nounits
        
        if ($GPUInfo) {
            $Metrics = $GPUInfo.Split(',')
            
            $GPUUtil = $Metrics[0].Trim()
            $MemUsed = $Metrics[1].Trim()
            $MemTotal = $Metrics[2].Trim() 
            $MemFree = $Metrics[3].Trim()
            $Temp = $Metrics[4].Trim()
            $ProcessCount = $Metrics[5].Trim()
            $PowerDraw = $Metrics[6].Trim()
            $ClockSM = $Metrics[7].Trim()
            $ClockMem = $Metrics[8].Trim()
            
            # Detect test phase based on memory usage patterns
            $MemUsedMB = [int]$MemUsed
            if ($MemUsedMB -gt 3000) {
                $TestPhase = "Model-Active"
            } elseif ($MemUsedMB -gt 1000) {
                $TestPhase = "Model-Loading"
            } elseif ($MemUsedMB -lt 500) {
                $TestPhase = "Idle"
            } else {
                $TestPhase = "Background"
            }
            
            $LogEntry = "$Timestamp,$TestPhase,$GPUUtil,$MemUsed,$MemTotal,$MemFree,$Temp,$ProcessCount,$PowerDraw,$ClockSM,$ClockMem"
            $LogEntry | Out-File -FilePath $LogFile -Append -Encoding UTF8
            
            # Real-time display
            Write-Host "$(Get-Date -Format "HH:mm:ss") | Phase: $TestPhase | GPU: $GPUUtil% | VRAM: $MemUsed/$MemTotal MB | Temp: $Temp°C | Power: $PowerDraw W" -ForegroundColor $(if ($MemUsedMB -gt 8000) { "Red" } elseif ($MemUsedMB -gt 4000) { "Yellow" } else { "Green" })
            
            # Alert if memory usage is high
            if ($MemUsedMB -gt 10000) {
                Write-Host "⚠️  HIGH MEMORY USAGE WARNING: $MemUsedMB MB" -ForegroundColor Red -BackgroundColor Yellow
            }
        }
        
        Start-Sleep -Seconds $IntervalSeconds
    }
}
catch {
    Write-Host "❌ Monitoring stopped: $($_.Exception.Message)" -ForegroundColor Red
}
finally {
    Write-Host "📊 Memory monitoring log saved to: $LogFile" -ForegroundColor Green
}