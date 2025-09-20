# RustChain Memory Log Analysis Script
# Analyzes GPU and system memory usage patterns from high-capacity stress test

param(
    [string]$LogDirectory = "memory-logs",
    [string]$SessionId = "",
    [string]$OutputReport = "memory-analysis-report.md"
)

Write-Host "📈 RustChain Memory Analysis Starting..." -ForegroundColor Green

# Find log files
if ($SessionId) {
    $GPULogFile = Get-ChildItem "$LogDirectory\gpu-memory-*$SessionId*.csv" | Select-Object -First 1
    $SystemLogFile = Get-ChildItem "$LogDirectory\system-memory-*$SessionId*.csv" | Select-Object -First 1
} else {
    $GPULogFile = Get-ChildItem "$LogDirectory\gpu-memory-*.csv" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    $SystemLogFile = Get-ChildItem "$LogDirectory\system-memory-*.csv" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
}

if (-not $GPULogFile -or -not $SystemLogFile) {
    Write-Host "❌ Could not find memory log files in $LogDirectory" -ForegroundColor Red
    exit 1
}

Write-Host "📊 Analyzing GPU log: $($GPULogFile.Name)" -ForegroundColor Cyan
Write-Host "📊 Analyzing System log: $($SystemLogFile.Name)" -ForegroundColor Cyan

# Import and analyze GPU data
$GPUData = Import-Csv $GPULogFile.FullName
$SystemData = Import-Csv $SystemLogFile.FullName

Write-Host "📈 Processing $($GPUData.Count) GPU data points..." -ForegroundColor Yellow
Write-Host "📈 Processing $($SystemData.Count) system data points..." -ForegroundColor Yellow

# GPU Analysis
$GPUAnalysis = @{
    MaxMemoryUsed = ($GPUData | Measure-Object -Property MemoryUsed_MB -Maximum).Maximum
    MinMemoryUsed = ($GPUData | Measure-Object -Property MemoryUsed_MB -Minimum).Minimum
    AvgMemoryUsed = [math]::Round(($GPUData | Measure-Object -Property MemoryUsed_MB -Average).Average, 1)
    MaxGPUUtil = ($GPUData | Measure-Object -Property GPUUtilization% -Maximum).Maximum
    AvgGPUUtil = [math]::Round(($GPUData | Measure-Object -Property GPUUtilization% -Average).Average, 1)
    MaxTemp = ($GPUData | Measure-Object -Property Temperature_C -Maximum).Maximum
    AvgTemp = [math]::Round(($GPUData | Measure-Object -Property Temperature_C -Average).Average, 1)
    MaxPower = ($GPUData | Measure-Object -Property PowerDraw_W -Maximum).Maximum
    AvgPower = [math]::Round(($GPUData | Measure-Object -Property PowerDraw_W -Average).Average, 1)
    TotalMemory = ($GPUData | Select-Object -First 1).MemoryTotal_MB
    MemoryEfficiency = 0
}

$GPUAnalysis.MemoryEfficiency = [math]::Round(($GPUAnalysis.MaxMemoryUsed / $GPUAnalysis.TotalMemory) * 100, 1)

# System Analysis
$SystemAnalysis = @{
    MaxRAMUsed = ($SystemData | Measure-Object -Property UsedRAM_GB -Maximum).Maximum
    MinRAMUsed = ($SystemData | Measure-Object -Property UsedRAM_GB -Minimum).Minimum
    AvgRAMUsed = [math]::Round(($SystemData | Measure-Object -Property UsedRAM_GB -Average).Average, 1)
    MaxRAMPercent = ($SystemData | Measure-Object -Property "UsedRAM_%" -Maximum).Maximum
    AvgRAMPercent = [math]::Round(($SystemData | Measure-Object -Property "UsedRAM_%" -Average).Average, 1)
    MaxRustChain = ($SystemData | Measure-Object -Property RustChain_MB -Maximum).Maximum
    AvgRustChain = [math]::Round(($SystemData | Measure-Object -Property RustChain_MB -Average).Average, 1)
    MaxOllama = ($SystemData | Measure-Object -Property Ollama_MB -Maximum).Maximum
    AvgOllama = [math]::Round(($SystemData | Measure-Object -Property Ollama_MB -Average).Average, 1)
    MaxCPU = ($SystemData | Measure-Object -Property "CPUUsage_%" -Maximum).Maximum
    AvgCPU = [math]::Round(($SystemData | Measure-Object -Property "CPUUsage_%" -Average).Average, 1)
    TotalRAM = ($SystemData | Select-Object -First 1).TotalRAM_GB
}

# Phase Analysis
$PhaseAnalysis = @{}
$GPUData | Group-Object TestPhase | ForEach-Object {
    $PhaseAnalysis[$_.Name] = @{
        Count = $_.Count
        Duration = "{0:F1}" -f (($_.Count * 2) / 60)  # Approximate minutes
        AvgMemory = [math]::Round(($_.Group | Measure-Object -Property MemoryUsed_MB -Average).Average, 1)
        MaxMemory = ($_.Group | Measure-Object -Property MemoryUsed_MB -Maximum).Maximum
        AvgGPUUtil = [math]::Round(($_.Group | Measure-Object -Property GPUUtilization% -Average).Average, 1)
    }
}

# Memory spikes detection
$MemorySpikes = $GPUData | Where-Object { [int]$_.MemoryUsed_MB -gt 8000 }
$HighRAMUsage = $SystemData | Where-Object { [double]$_."UsedRAM_%" -gt 85 }

# Generate analysis report
$ReportContent = @"
# RustChain High-Capacity Memory Analysis Report

**Generated:** $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
**GPU Log:** $($GPULogFile.Name)
**System Log:** $($SystemLogFile.Name)
**Data Points:** $($GPUData.Count) GPU samples, $($SystemData.Count) system samples

## 🔋 GPU Memory Analysis

| Metric | Value |
|--------|--------|
| **Total GPU Memory** | $($GPUAnalysis.TotalMemory) MB |
| **Peak Memory Usage** | $($GPUAnalysis.MaxMemoryUsed) MB |
| **Average Memory Usage** | $($GPUAnalysis.AvgMemoryUsed) MB |
| **Minimum Memory Usage** | $($GPUAnalysis.MinMemoryUsed) MB |
| **Memory Efficiency** | $($GPUAnalysis.MemoryEfficiency)% of total |
| **Peak GPU Utilization** | $($GPUAnalysis.MaxGPUUtil)% |
| **Average GPU Utilization** | $($GPUAnalysis.AvgGPUUtil)% |
| **Peak Temperature** | $($GPUAnalysis.MaxTemp)°C |
| **Average Temperature** | $($GPUAnalysis.AvgTemp)°C |
| **Peak Power Draw** | $($GPUAnalysis.MaxPower)W |
| **Average Power Draw** | $($GPUAnalysis.AvgPower)W |

## 💾 System Memory Analysis

| Metric | Value |
|--------|--------|
| **Total System RAM** | $($SystemAnalysis.TotalRAM) GB |
| **Peak RAM Usage** | $($SystemAnalysis.MaxRAMUsed) GB ($($SystemAnalysis.MaxRAMPercent)%) |
| **Average RAM Usage** | $($SystemAnalysis.AvgRAMUsed) GB ($($SystemAnalysis.AvgRAMPercent)%) |
| **Peak RustChain Memory** | $($SystemAnalysis.MaxRustChain) MB |
| **Average RustChain Memory** | $($SystemAnalysis.AvgRustChain) MB |
| **Peak Ollama Memory** | $($SystemAnalysis.MaxOllama) MB |
| **Average Ollama Memory** | $($SystemAnalysis.AvgOllama) MB |
| **Peak CPU Usage** | $($SystemAnalysis.MaxCPU)% |
| **Average CPU Usage** | $($SystemAnalysis.AvgCPU)% |

## 📊 Test Phase Breakdown

"@

$PhaseAnalysis.GetEnumerator() | Sort-Object Name | ForEach-Object {
    $ReportContent += @"
### $($_.Name)
- **Duration:** ~$($_.Value.Duration) minutes ($($_.Value.Count) samples)
- **Average GPU Memory:** $($_.Value.AvgMemory) MB
- **Peak GPU Memory:** $($_.Value.MaxMemory) MB  
- **Average GPU Utilization:** $($_.Value.AvgGPUUtil)%

"@
}

$ReportContent += @"

## ⚠️ Memory Alerts and Issues

### GPU Memory Spikes (>8GB)
$(if ($MemorySpikes.Count -gt 0) {
    "**Found $($MemorySpikes.Count) instances of high GPU memory usage:**`n"
    $MemorySpikes | ForEach-Object { "- $($_.Timestamp): $($_.MemoryUsed_MB) MB during $($_.TestPhase) phase`n" }
} else {
    "✅ No GPU memory spikes detected (stayed within safe limits)"
})

### High System RAM Usage (>85%)
$(if ($HighRAMUsage.Count -gt 0) {
    "**Found $($HighRAMUsage.Count) instances of high system RAM usage:**`n"
    $HighRAMUsage | Select-Object -First 10 | ForEach-Object { "- $($_.Timestamp): $($_.'UsedRAM_%')% ($($_.UsedRAM_GB) GB) during $($_.TestPhase) phase`n" }
    if ($HighRAMUsage.Count -gt 10) { "- ... and $($HighRAMUsage.Count - 10) more instances`n" }
} else {
    "✅ No critical system RAM usage detected"
})

## 🎯 Performance Insights

### Memory Efficiency
$(if ($GPUAnalysis.MemoryEfficiency -lt 70) {
    "✅ **Excellent**: GPU memory usage stayed well within limits ($($GPUAnalysis.MemoryEfficiency)% peak)"
} elseif ($GPUAnalysis.MemoryEfficiency -lt 85) {
    "✅ **Good**: GPU memory usage was reasonable ($($GPUAnalysis.MemoryEfficiency)% peak)"
} else {
    "⚠️ **Caution**: GPU memory usage approached limits ($($GPUAnalysis.MemoryEfficiency)% peak)"
})

### Model Loading Efficiency
$(if ($SystemAnalysis.MaxOllama -gt 4000) {
    "⚠️ **High Ollama Memory**: Peak Ollama usage was $($SystemAnalysis.MaxOllama) MB - consider model optimization"
} else {
    "✅ **Efficient**: Ollama memory usage stayed reasonable (peak $($SystemAnalysis.MaxOllama) MB)"
})

### RustChain Process Efficiency
$(if ($SystemAnalysis.MaxRustChain -gt 2000) {
    "⚠️ **High RustChain Memory**: Peak usage was $($SystemAnalysis.MaxRustChain) MB - investigate memory management"
} else {
    "✅ **Efficient**: RustChain memory usage was reasonable (peak $($SystemAnalysis.MaxRustChain) MB)"
})

## 📈 Recommendations

1. **Memory Management**: $(if ($GPUAnalysis.MemoryEfficiency -gt 80) { "Consider reducing model complexity or implementing more aggressive memory cleanup" } else { "Current memory management is effective for this workload scale" })

2. **Performance Optimization**: $(if ($SystemAnalysis.AvgCPU -gt 80) { "High CPU usage detected - consider optimizing computational workload" } else { "CPU utilization was reasonable for this scale of operation" })

3. **Scalability**: $(if ($GPUAnalysis.MaxMemoryUsed -gt 10000) { "Approaching GPU memory limits - may need optimization for larger workloads" } else { "Current setup can handle larger workloads with available memory headroom" })

4. **Thermal Management**: $(if ($GPUAnalysis.MaxTemp -gt 80) { "Monitor GPU temperatures - peak was $($GPUAnalysis.MaxTemp)°C" } else { "GPU temperatures remained within safe operating range" })

## 🔍 Next Steps for Optimization

1. **Memory Profiling**: Focus on phases with highest memory usage: $(($PhaseAnalysis.GetEnumerator() | Sort-Object {$_.Value.MaxMemory} -Descending | Select-Object -First 3 | ForEach-Object {$_.Name}) -join ", ")

2. **Performance Tuning**: Investigate CPU optimization opportunities if average usage was $($SystemAnalysis.AvgCPU)%

3. **Model Optimization**: $(if ($SystemAnalysis.MaxOllama -gt 3000) { "Consider model quantization or optimization for Ollama memory usage" } else { "Current model configuration is efficient" })

---

**Analysis Complete** | **Total Test Duration**: ~$(($GPUData.Count * 2) / 60) minutes | **Memory Monitoring Success**: ✅
"@

# Save report
$ReportContent | Out-File -FilePath $OutputReport -Encoding UTF8

Write-Host "=" * 70 -ForegroundColor Green
Write-Host "📈 Memory Analysis Complete!" -ForegroundColor Green -BackgroundColor Black
Write-Host "📊 Report generated: $OutputReport" -ForegroundColor Cyan
Write-Host "=" * 70 -ForegroundColor Green

# Display key findings
Write-Host "🔋 GPU Memory Findings:" -ForegroundColor Yellow
Write-Host "   Peak Usage: $($GPUAnalysis.MaxMemoryUsed) MB / $($GPUAnalysis.TotalMemory) MB ($($GPUAnalysis.MemoryEfficiency)%)" -ForegroundColor White
Write-Host "   Average Usage: $($GPUAnalysis.AvgMemoryUsed) MB" -ForegroundColor White
Write-Host "   Memory Spikes: $($MemorySpikes.Count) instances >8GB" -ForegroundColor $(if ($MemorySpikes.Count -gt 0) { "Red" } else { "Green" })

Write-Host ""
Write-Host "💾 System Memory Findings:" -ForegroundColor Yellow  
Write-Host "   Peak RAM: $($SystemAnalysis.MaxRAMUsed) GB ($($SystemAnalysis.MaxRAMPercent)%)" -ForegroundColor White
Write-Host "   RustChain Peak: $($SystemAnalysis.MaxRustChain) MB" -ForegroundColor White
Write-Host "   Ollama Peak: $($SystemAnalysis.MaxOllama) MB" -ForegroundColor White
Write-Host "   High RAM Usage: $($HighRAMUsage.Count) instances >85%" -ForegroundColor $(if ($HighRAMUsage.Count -gt 0) { "Red" } else { "Green" })

Write-Host ""
Write-Host "📊 Full analysis report: $OutputReport" -ForegroundColor Green