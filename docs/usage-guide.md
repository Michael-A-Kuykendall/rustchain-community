# 🚀 RustChain Quick Usage Guide

## ✅ **Global Installation Complete**
- Binary location: `C:\Users\micha\.cargo\bin\rustchain.exe`
- Features enabled: `agent`, `chain`, `tools`, `llm`, `cli`
- Version: 0.1.0

## 🎯 **Common Commands**

### **Execute Missions**
```bash
# From any directory - relative paths work
rustchain run mission_file.yaml

# Or with absolute paths
rustchain run "C:\full\path\to\mission.yaml"

# Validate before running
rustchain mission validate mission.yaml

# Dry run (no execution)
rustchain run mission.yaml --dry-run
```

### **LLM Operations**
```bash
# Direct LLM chat
rustchain llm chat "Your prompt here"

# List available models
rustchain llm models

# Test LLM connectivity
rustchain llm test
```

### **Compliance Verification**
```bash
# Verify against specific standard
rustchain compliance verify mission.yaml --standard GDPR

# Verify against all 8 standards
rustchain compliance verify mission.yaml --all-standards

# List available standards
rustchain compliance list-standards
```

### **Tools & System Info**
```bash
# List available tools
rustchain tools list

# Check system configuration
rustchain config show

# Check safety settings
rustchain safety status

# View audit trail
rustchain audit report
```

## 📋 **Mission File Locations**

**Works from any directory:**
- `rustchain run mission.yaml` (in current directory)
- `rustchain run ../other-dir/mission.yaml` (relative path)
- `rustchain run C:\full\path\mission.yaml` (absolute path)

## ⚡ **Performance Notes**
- **LLM Steps**: ~2-3 seconds with phi3:mini
- **Command Steps**: ~0.3 seconds  
- **Agent Steps**: ~2-5 seconds (with tool access)
- **Validation**: Instant

## 🔧 **No Runtime Feature Flags**
Features are compiled in at install time. The global binary includes all features:
- ✅ LLM integration (Ollama + Shimmy)
- ✅ Agent reasoning with tools
- ✅ Chain execution
- ✅ Compliance verification
- ✅ Safety validation
- ✅ Audit trails

## 🎉 **Ready for Production Use**
Your RustChain installation is fully functional and ready for AI agent development workflows!