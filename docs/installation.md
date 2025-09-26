# 🚀 RustChain Stable Installation Guide

**Version**: Community Edition with Enhanced Chain System  
**Build Features**: `agent,chain,tools,llm`  
**Status**: ✅ Production Ready

## 📦 Quick Installation

```bash
# Clone and build stable version
git clone https://github.com/rustchain-community/rustchain-community.git
cd rustchain-community

# Build stable feature set (tested and working)
cargo build --release --features "agent,chain,tools,llm"

# Install locally (optional)
cp target/release/rustchain.exe ~/.local/bin/rustchain.exe
# OR (Windows)
copy "target\release\rustchain.exe" "C:\Users\%USERNAME%\.local\bin\rustchain.exe"
```

## ✅ Verified Working Features

### 🎯 Core Mission System
```bash
# Validate any mission
rustchain mission validate missions/your_mission.yaml

# Execute missions  
rustchain run missions/your_mission.yaml

# Safety validation
rustchain safety validate missions/your_mission.yaml
```

### 🔗 Enhanced Chain System (NEW)
```bash
# Execute nested chain missions
rustchain run missions/punch_web_typescript_foundation_chain.yaml

# Validate complex chains
rustchain mission validate missions/punch_web_typescript_foundation_chain.yaml
```

**Chain Capabilities**:
- ✅ Nested step execution with variable passing
- ✅ Async recursion support (Box::pin optimization)  
- ✅ Variable scoping between chain and parent contexts
- ✅ Integration with Agent, LLM, and Tool systems

### 🤖 Agent & LLM System
```bash
# Test LLM connectivity
rustchain llm test

# Execute agent missions  
rustchain run missions/agent_mission.yaml
```

### 🛠️ Tool Framework
```bash
# List available tools
rustchain tools list

# Execute tool operations
rustchain tools execute create_file --params '{"path":"test.txt","content":"Hello"}'
```

## 📋 Optional: Compliance Features

```bash
# Build with compliance (requires comply-sdk)
cargo build --release --features "agent,chain,tools,llm,compliance"

# Compliance verification
rustchain compliance verify missions/gdpr_article_6_analysis.yaml

# List compliance standards
rustchain compliance list
```

## 🏥 System Health Check

```bash
# Comprehensive system status
cat SYSTEM_HEALTH.md

# Feature availability
rustchain features summary

# Test core functionality
rustchain mission validate missions/art_01_foundation.yaml
```

## 🎯 Recommended Workflow

1. **Start with Mission Validation**: Always validate before execution
2. **Use Stable Features**: Stick to `agent,chain,tools,llm` for maximum reliability
3. **Monitor Health**: Check `SYSTEM_HEALTH.md` for current system status
4. **Test Incrementally**: Start with simple missions, progress to complex chains

## 🔧 Known Working Missions

- ✅ `missions/art_01_foundation.yaml` - Basic mission execution
- ✅ `missions/punch_web_typescript_foundation_chain.yaml` - Chain system test
- ✅ `missions/gdpr_article_6_analysis.yaml` - Compliance analysis
- ✅ All missions in `missions/` directory (with appropriate features)

---

**Installation Status**: ✅ **STABLE AND READY FOR USE**  
**Last Verified**: 2025-08-31  
**Chain System**: ✅ **Enhanced with Nested Execution**