# RustChain Memory Monitoring System

Real-time GPU and system memory monitoring for RustChain high-capacity stress testing.

## 🚀 Quick Start

```powershell
# 1. Start comprehensive memory monitoring
cd C:\Users\micha\repos\rustchain-community\memory-monitoring
.\start-memory-monitoring.ps1

# 2. In another terminal, run RustChain missions
cd C:\Users\micha\repos\rustchain-community
# ... execute RustChain missions ...

# 3. Stop monitoring (Ctrl+C) when complete

# 4. Analyze results
.\analyze-memory-logs.ps1
```

## 📊 Monitoring Components

### GPU Memory Monitor (`gpu-memory-monitor.ps1`)
- **Monitors**: VRAM usage, GPU utilization, temperature, power draw
- **Frequency**: Every 2 seconds
- **Alerts**: High memory usage warnings (>8GB, >10GB)
- **Output**: CSV log with timestamp correlation

### System Memory Monitor (`system-memory-monitor.ps1`)  
- **Monitors**: System RAM, RustChain process memory, Ollama memory, CPU usage
- **Frequency**: Every 2 seconds
- **Alerts**: High system RAM (>85%, >90%), process memory spikes
- **Output**: CSV log with process-specific tracking

### Orchestrator (`start-memory-monitoring.ps1`)
- **Function**: Coordinates both monitors with synchronized logging
- **Features**: Real-time status display, automatic log organization
- **Output**: Session-based log files with unique identifiers

### Analyzer (`analyze-memory-logs.ps1`)
- **Function**: Post-execution analysis of memory usage patterns
- **Generates**: Comprehensive markdown report with insights and recommendations
- **Analyzes**: Memory efficiency, performance patterns, optimization opportunities

## 📈 Key Metrics Tracked

### GPU Metrics
- **Memory Usage**: Current, peak, average VRAM consumption
- **Utilization**: GPU compute utilization percentage
- **Temperature**: Thermal monitoring for stability
- **Power Draw**: Energy consumption during operations
- **Clock Speeds**: SM and memory clock frequencies

### System Metrics
- **RAM Usage**: Total system memory consumption and percentage
- **Process Memory**: RustChain and Ollama specific memory usage
- **CPU Utilization**: Processor load during operations
- **Process Counts**: Total system process count
- **Page File**: Virtual memory usage

### Test Phases
- **Idle**: Baseline system state
- **Model-Loading**: AI model initialization phase
- **Model-Active**: Active AI processing phase
- **RustChain-Active**: RustChain framework execution
- **Full-Processing**: Combined RustChain + AI model processing

## 🎯 Memory Constraints and Targets

### GPU Memory (12GB Total)
- **Safe Operating Zone**: <8GB (67%)
- **Caution Zone**: 8-10GB (67-83%)
- **Critical Zone**: >10GB (>83%)
- **Model Memory**: ~2-3GB per loaded model
- **Target Efficiency**: Sequential execution with model rotation

### System Memory
- **Normal Usage**: <70% of total system RAM
- **High Usage Alert**: >85% of total system RAM
- **Critical Alert**: >90% of total system RAM
- **RustChain Target**: <2GB process memory
- **Ollama Target**: <4GB process memory

## 🔍 Analysis and Insights

The monitoring system provides:

1. **Memory Efficiency Analysis**: Peak vs average usage patterns
2. **Performance Correlation**: Memory usage vs processing phases
3. **Optimization Insights**: Bottleneck identification and recommendations
4. **Scaling Predictions**: Capacity planning for larger workloads
5. **Stability Assessment**: Memory leak detection and resource management

## 📁 Output Structure

```
memory-logs/
├── gpu-memory-RustChain-HighCapacity-YYYY-MM-DD_HH-mm-ss.csv
├── system-memory-RustChain-HighCapacity-YYYY-MM-DD_HH-mm-ss.csv  
├── test-execution-RustChain-HighCapacity-YYYY-MM-DD_HH-mm-ss.log
└── memory-analysis-report.md
```

## 🚨 Alert System

### Real-Time Alerts
- **GPU Memory >8GB**: Yellow warning
- **GPU Memory >10GB**: Red alert with background highlight
- **System RAM >85%**: Yellow warning
- **System RAM >90%**: Red alert with background highlight
- **Process Memory Spikes**: Cyan/magenta highlights for RustChain/Ollama

### Monitoring Status
- **Runtime Display**: Continuous elapsed time and monitor status
- **Job Health**: Automatic detection of monitor failures
- **Session Logging**: Periodic status updates in execution log

## 🎮 Integration with RustChain Stress Test

The monitoring system is designed to run alongside the RustChain high-capacity stress test:

1. **Phase Correlation**: Memory usage patterns correlated with RustChain execution phases
2. **Mission Tracking**: Performance characteristics of different mission types
3. **Model Switching**: Memory cleanup efficiency during Champion ↔ Rust-Llama rotation
4. **Chain Analysis**: Memory accumulation during long reasoning chains
5. **Agent Processing**: Memory usage patterns during multi-iteration agent reasoning

## 📊 Expected Memory Patterns

### Planning Missions (Champion Model)
- **Initial Load**: ~2-3GB VRAM spike
- **Processing**: Steady 2-3GB usage with periodic cleanup
- **Chain Steps**: Gradual context accumulation
- **Completion**: Memory cleanup before next mission

### Implementation Missions (Rust-Llama)
- **Model Switch**: Temporary memory spike during transition
- **Fast Processing**: Higher VRAM utilization due to speed
- **Code Generation**: Sustained high utilization periods
- **Efficiency**: Lower context accumulation due to focused tasks

### Analysis Missions (Multi-Agent)
- **Agent Iterations**: Multiple context accumulation cycles
- **Tool Usage**: Additional memory for tool execution
- **Cross-Context**: Variable preservation across reasoning steps
- **Synthesis**: Final memory cleanup and result generation

---

**Ready for RustChain High-Capacity Stress Test with Comprehensive Memory Monitoring** 🚀