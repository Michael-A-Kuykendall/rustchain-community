# 🚀 RustChain Quick Start Guide

**Ready to deploy in 5 minutes for your white paper experiments!**

## ✅ Prerequisites Checklist

Before starting, ensure you have:

- **Rust 1.70+**: `rustc --version`
- **Git**: `git --version`
- **Ollama** (for LLM features): Download from [ollama.ai](https://ollama.ai)

## 🎯 Installation & Build

```bash
# 1. Clone the repository
git clone https://github.com/rustchain-community/rustchain-community.git
cd rustchain-community

# 2. Build with all features (recommended)
cargo build --release --all-features

# 3. Verify installation
cargo run --bin rustchain -- --version
```

## 🧠 LLM Setup (Required for AI Features)

RustChain works with your local models via Ollama:

```bash
# 1. Install and start Ollama
# Download from ollama.ai, then:
ollama serve

# 2. Pull a small model for testing (in another terminal)
ollama pull llama3.2:1b

# 3. Test RustChain LLM integration
cargo run --bin rustchain --features llm -- run missions/test_llm_simple.yaml
```

**💡 Pro Tip**: If you have a custom Champion model like `llama32-champion:latest`, RustChain will automatically detect and use it!

## 🎮 Essential Commands

### Basic Mission Execution
```bash
# Run a simple file operation mission
cargo run --bin rustchain -- run missions/hello_world_mission.yaml

# Run with LLM features enabled
cargo run --bin rustchain --features llm -- run missions/gdpr_article_6_analysis.yaml

# Run with safety validation
cargo run --bin rustchain -- safety validate missions/your_mission.yaml
```

### Configuration Management
```bash
# Initialize default configuration
cargo run --bin rustchain -- config init

# View current configuration
cargo run --bin rustchain -- config show

# Check system status
cargo run --bin rustchain -- features status
```

### Mission Management
```bash
# List available missions
cargo run --bin rustchain -- mission list

# Validate a mission file
cargo run --bin rustchain -- mission validate your_mission.yaml

# Get mission information
cargo run --bin rustchain -- mission info your_mission.yaml
```

### Tool System
```bash
# List available tools
cargo run --bin rustchain --features tools -- tools list

# Execute a tool directly
cargo run --bin rustchain --features tools -- tools execute file_create \
  --params '{"path":"test.txt","content":"Hello from RustChain!"}'

# Get tool help
cargo run --bin rustchain --features tools -- tools info file_create
```

## 📁 Mission File Structure

Create your own missions with this template:

```yaml
# my_experiment.yaml
name: "White Paper Experiment"
description: "Testing RustChain for research paper"
version: "1.0"

steps:
  - id: "create_data"
    name: "Create experimental data"
    step_type: "create_file"
    parameters:
      path: "experiment_data.json"
      content: |
        {
          "experiment": "white_paper_test",
          "timestamp": "2025-01-01T00:00:00Z",
          "results": []
        }

  - id: "analyze_data" 
    name: "Analyze data with AI"
    step_type: "llm"
    parameters:
      provider: "ollama"
      model: "llama32-champion:latest"  # or llama3.2:1b
      prompt: |
        Analyze this experimental setup for a white paper.
        What insights can you provide about the methodology?
        Keep response focused and under 200 words.
      temperature: 0.1
      max_tokens: 500

dependencies:
  - from: "create_data"
    to: "analyze_data"
```

## 🔍 Feature Flags Reference

Control exactly which features you need:

```bash
# Core features (always available)
cargo run --bin rustchain -- run mission.yaml

# With LLM support
cargo run --bin rustchain --features llm -- run mission.yaml

# With all tools
cargo run --bin rustchain --features tools -- run mission.yaml

# With agent reasoning
cargo run --bin rustchain --features agent -- run mission.yaml

# With memory/context
cargo run --bin rustchain --features memory -- run mission.yaml

# Kitchen sink (all features)
cargo run --bin rustchain --features full -- run mission.yaml
```

## 🧪 Ready-to-Use Examples

RustChain includes several missions perfect for white paper experiments:

### GDPR Compliance Analysis
```bash
# Analyze GDPR Article 6 (Legal Basis)
cargo run --bin rustchain --features llm -- run missions/gdpr_article_6_analysis.yaml

# Analyze GDPR Article 17 (Right to Erasure) 
cargo run --bin rustchain --features llm -- run missions/gdpr_article_17_analysis.yaml

# Analyze GDPR Article 25 (Privacy by Design)
cargo run --bin rustchain --features llm -- run missions/gdpr_article_25_analysis.yaml
```

### Basic AI Workflow
```bash
# Test LLM integration
cargo run --bin rustchain --features llm -- run test_llm_simple.yaml

# Multi-step reasoning
cargo run --bin rustchain --features agent -- run examples/agent_demo.yaml
```

### Standards Compliance Demo
```bash
# Show mathematical compliance proof
cargo run --bin rustchain --features smt -- run missions/gdpr_compliance_demo.yaml
```

## ✅ Production Deployment Checklist

Before using RustChain in your project:

- [ ] **Build succeeds**: `cargo build --release --all-features`
- [ ] **Tests pass**: `cargo test --all-features` 
- [ ] **LLM connected**: `ollama list` shows your models
- [ ] **Basic mission works**: Test with a simple mission
- [ ] **Features needed**: Enable only the features you need
- [ ] **Logging configured**: Check `RUST_LOG=info` for verbose output

## 🚨 Troubleshooting

### LLM Steps Fail Immediately (0.00s duration)
- **Cause**: Missing `--features llm` flag
- **Fix**: Always use `cargo run --bin rustchain --features llm -- run mission.yaml`

### "Unrecognized subcommand 'execute'"
- **Cause**: Wrong command structure
- **Fix**: Use `run` not `execute`: `cargo run --bin rustchain -- run mission.yaml`

### Model Not Found
- **Cause**: Model not pulled to Ollama
- **Fix**: `ollama pull llama3.2:1b` (or your model name)

### Compilation Errors
- **Cause**: Rust version too old
- **Fix**: Update Rust: `rustup update`

## 🎯 Performance Tips

1. **Use release builds**: `--release` for faster execution
2. **Choose right model size**: Small models (1B-3B params) for simple tasks
3. **Batch operations**: Combine related steps in single missions
4. **Feature flags**: Enable only needed features to reduce binary size

## 📞 Support

- **GitHub Issues**: Report bugs and request features
- **Documentation**: Check `CLAUDE.md` for advanced configuration
- **Examples**: See `missions/` directory for working examples

**Ready to build AI agents for your white paper? Start experimenting!** 🚀