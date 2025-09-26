# 🚀 RustChain CLI Usage Guide

*Complete command-line reference with examples and best practices*

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Core Commands](#core-commands)
- [Mission Management](#mission-management)
- [AI & LLM Integration](#ai--llm-integration)
- [Tools & Execution](#tools--execution)
- [Security & Safety](#security--safety)
- [Enterprise Features](#enterprise-features)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)

## 🎯 Quick Start

### Basic Command Structure
```bash
rustchain <COMMAND> [OPTIONS] [ARGS]
```

### Most Common Commands
```bash
# Run a mission file
rustchain run examples/hello_world.yaml

# Validate a mission before execution
rustchain mission validate my_mission.yaml

# Chat with an LLM
rustchain llm chat "Explain Rust ownership"

# List available tools
rustchain tools list

# Check system health
rustchain build status
```

### Help & Version Information
```bash
rustchain --help           # Show all available commands
rustchain --version        # Show RustChain version
rustchain run --help       # Show help for specific command
```

## 🎯 Core Commands

### Interactive Mode
Start conversational mode similar to Claude Code:

```bash
# Start interactive session
rustchain interactive

# Example session:
$ rustchain interactive
RustChain Interactive Mode - Type 'exit' to quit
> create a simple mission to analyze a codebase
> run the generated mission
> show me the results in a formatted report
```

### Mission Execution
Execute YAML-defined mission files:

```bash
# Basic execution
rustchain run path/to/mission.yaml

# Dry run (validate without executing)
rustchain run mission.yaml --dry-run

# Skip safety validation (use with caution)
rustchain run mission.yaml --skip-safety

# Combine flags
rustchain run mission.yaml --dry-run --skip-safety
```

**Example Mission File:**
```yaml
# simple_analysis.yaml
name: "Code Analysis"
description: "Analyze codebase for patterns"
version: "1.0"
steps:
  - id: "analyze"
    step_type: "llm"
    parameters:
      provider: "openai"
      model: "gpt-4"
      prompt: "Analyze this Rust codebase for common patterns"
```

## 📊 Mission Management

### Mission Operations
```bash
# List available example missions
rustchain mission list

# Validate mission syntax and structure
rustchain mission validate path/to/mission.yaml

# Show detailed mission information
rustchain mission info path/to/mission.yaml
```

**Example Output:**
```bash
$ rustchain mission info examples/hello_world.yaml
Mission: Hello World Demo
Description: Simple demonstration of RustChain capabilities
Version: 1.0
Steps: 3
- create_file: Create greeting file
- llm: Generate personalized message
- command: Display results
Safety Level: LOW
Estimated Duration: 30 seconds
```

### Mission Validation Examples
```bash
# Basic validation
$ rustchain mission validate mission.yaml
✅ Mission syntax valid
✅ All required fields present
✅ Step dependencies resolved
✅ Safety checks passed

# Validation with errors
$ rustchain mission validate broken_mission.yaml
❌ Mission validation failed
- Line 15: Unknown step_type 'invalid_step'
- Line 22: Missing required parameter 'prompt'
- Step 'step2' depends on non-existent 'step0'
```

## 🤖 AI & LLM Integration

### LLM Provider Management
```bash
# List all available models
rustchain llm models

# List models from specific provider
rustchain llm models --provider openai
rustchain llm models --provider anthropic
rustchain llm models --provider ollama

# Test connectivity to providers
rustchain llm test
rustchain llm test openai
rustchain llm test anthropic
```

### Interactive Chat
```bash
# Basic chat with default model
rustchain llm chat "What is Rust ownership?"

# Specify model and provider
rustchain llm chat "Explain async/await" --model gpt-4 --provider openai

# Adjust temperature for creativity
rustchain llm chat "Write a creative story" --temperature 1.2

# Technical discussion with low temperature
rustchain llm chat "Explain memory safety" --temperature 0.1 --model claude-3-sonnet
```

**Example Chat Session:**
```bash
$ rustchain llm chat "How do I implement a trait in Rust?" --model gpt-4
🤖 GPT-4 Response:

To implement a trait in Rust, you use the `impl` keyword followed by the trait name for your type:

```rust
// Define a trait
trait Drawable {
    fn draw(&self);
}

// Implement the trait for a struct
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}
```

Key points:
- Use `impl TraitName for TypeName`
- Implement all required methods
- You can add default implementations in the trait definition
```

## 🔧 Tools & Execution

### Tool Management
```bash
# List all available tools
rustchain tools list

# Get detailed information about a specific tool
rustchain tools info file_create
rustchain tools info http_request
rustchain tools info command_execute

# Execute a tool directly
rustchain tools execute file_create --params '{"path":"test.txt","content":"Hello World"}'
rustchain tools execute command_execute --params '{"command":"ls","args":["-la"]}'
```

**Example Tool Execution:**
```bash
$ rustchain tools execute http_request --params '{
    "url": "https://api.github.com/repos/rust-lang/rust",
    "method": "GET",
    "headers": {"User-Agent": "RustChain-CLI"}
}'

🔧 Tool Execution Result:
Status: 200 OK
Response Time: 234ms
Body: {
  "name": "rust",
  "full_name": "rust-lang/rust",
  "description": "Empowering everyone to build reliable and efficient software.",
  "stargazers_count": 89234,
  ...
}
```

### Available Tools Categories
```bash
# File operations
rustchain tools info file_create     # Create files
rustchain tools info file_read       # Read file contents
rustchain tools info file_write      # Write to files

# Network operations  
rustchain tools info http_request    # HTTP requests
rustchain tools info websocket       # WebSocket connections

# System operations
rustchain tools info command_execute # Run system commands
rustchain tools info process_info    # Process information

# AI operations
rustchain tools info llm_call        # Direct LLM calls
rustchain tools info embedding       # Generate embeddings
```

## 🛡️ Security & Safety

### Safety Validation
```bash
# Validate mission safety
rustchain safety validate path/to/mission.yaml

# Strict mode (fail on warnings)
rustchain safety validate mission.yaml --strict

# Comprehensive safety check
rustchain safety check

# Include policy validation
rustchain safety check --include-policies

# Generate safety report
rustchain safety report mission.yaml
rustchain safety report mission.yaml --format json
rustchain safety report mission.yaml --format yaml
```

**Example Safety Report:**
```bash
$ rustchain safety validate examples/file_operations.yaml
🛡️ Safety Validation Report

Overall Risk Level: LOW
Issues Found: 2 warnings, 0 errors

Warnings:
⚠️  Step 'create_file': Writing to system directory
    - Path: /tmp/rustchain_output.txt
    - Recommendation: Use relative paths when possible

⚠️  Step 'command_exec': Shell command execution
    - Command: ls -la /tmp
    - Recommendation: Validate command parameters

Safety Checks Passed:
✅ No network access to external services
✅ No file system access outside sandbox
✅ No privileged operations requested
✅ All LLM prompts reviewed for safety
```

### Policy Management
```bash
# List active policies
rustchain policy list

# Validate policy configuration
rustchain policy validate

# Show policy enforcement status
rustchain policy status
```

**Example Policy Status:**
```bash
$ rustchain policy status
📋 Policy Enforcement Status

Active Policies: 8
├── File Access Policy: ENFORCED
│   ├── Allowed directories: ./workspace, /tmp
│   └── Blocked patterns: /etc/*, /root/*
├── Network Policy: ENFORCED  
│   ├── Allowed domains: api.openai.com, api.anthropic.com
│   └── Blocked IPs: All private ranges
├── Command Execution Policy: ENFORCED
│   ├── Allowed commands: ls, cat, echo, find
│   └── Blocked patterns: rm -rf, sudo, chmod
└── LLM Safety Policy: ENFORCED
    ├── Content filtering: ENABLED
    └── Prompt injection detection: ENABLED

Policy Violations (Last 24h): 0
```

## 🏢 Enterprise Features

### Authentication & RBAC
```bash
# Initialize JWT authentication
rustchain enterprise auth init-jwt

# Setup OAuth2 integration
rustchain enterprise auth setup-oauth2 google --client-id your-client-id

# Configure RBAC system
rustchain enterprise auth setup-rbac --roles-file roles.yaml

# Test authentication
rustchain enterprise auth test
```

### Compliance & Auditing
```bash
# Verify mission compliance
rustchain enterprise compliance verify mission.yaml

# Check against specific standard
rustchain enterprise compliance verify mission.yaml --standard GDPR

# Check against all standards
rustchain enterprise compliance verify mission.yaml --all-standards

# Generate compliance report
rustchain enterprise compliance report mission.yaml --output compliance_report.pdf

# List available compliance standards
rustchain enterprise compliance list-standards

# Run compliance audit
rustchain enterprise compliance audit
```

**Example Compliance Check:**
```bash
$ rustchain enterprise compliance verify data_processing.yaml --standard GDPR
🏛️ GDPR Compliance Check

Mission: Data Processing Pipeline
Standard: GDPR (General Data Protection Regulation)

✅ COMPLIANT - Overall Assessment

Compliance Checks:
✅ Data minimization: Only necessary data processed
✅ Purpose limitation: Clear processing purpose stated
✅ Storage limitation: Retention period specified (30 days)
✅ Consent mechanism: User consent collection implemented
✅ Data subject rights: Access and deletion capabilities present
✅ Security measures: Encryption and access controls in place

Recommendations:
💡 Consider implementing automated data anonymization
💡 Add data breach notification procedures
💡 Document legal basis for processing
```

### Monitoring & Performance
```bash
# Start metrics collection
rustchain enterprise monitoring start-metrics --port 9090

# Show performance dashboard
rustchain enterprise monitoring dashboard

# Configure alerting
rustchain enterprise monitoring setup-alerts --config alerts.yaml

# Show current metrics
rustchain enterprise monitoring metrics
```

### Multi-Tenancy
```bash
# Create new tenant
rustchain enterprise multi-tenant create-tenant acme-corp "ACME Corporation"

# List all tenants
rustchain enterprise multi-tenant list-tenants

# Configure tenant isolation
rustchain enterprise multi-tenant setup-isolation acme-corp --level strict
```

## 🔧 Advanced Usage

### RAG (Retrieval-Augmented Generation)
```bash
# Add documents to RAG system
rustchain rag add --id doc1 --file document.pdf --metadata '{"type":"technical"}'
rustchain rag add --id doc2 --file readme.md --metadata '{"type":"documentation"}'

# Search documents
rustchain rag search "machine learning algorithms" --limit 5 --threshold 0.8

# List all documents
rustchain rag list --offset 0 --limit 10

# Get context for a query
rustchain rag context "How to implement neural networks" --max-length 2000

# Delete document
rustchain rag delete doc1
```

### Sandbox Operations
```bash
# Create isolated sandbox
rustchain sandbox create

# Execute commands in sandbox
rustchain sandbox execute --session session1 "ls -la"

# Write files to sandbox
rustchain sandbox write --session session1 --file test.py --content "print('Hello')"

# Read files from sandbox
rustchain sandbox read --session session1 --file test.py

# List sandbox files
rustchain sandbox files --session session1

# Get sandbox information
rustchain sandbox info --session session1

# Clean up sandbox
rustchain sandbox destroy --session session1
```

### Server Mode
```bash
# Start API server
rustchain server start --host 0.0.0.0 --port 8080 --cors

# Get server configuration
rustchain server config

# Custom configuration
rustchain server start --host 127.0.0.1 --port 9090
```

### Audit & Compliance
```bash
# Query audit logs
rustchain audit query --start-time 2024-01-01T00:00:00Z --limit 50

# Generate audit report
rustchain audit report --format csv --start-time 2024-01-01T00:00:00Z

# Verify audit chain integrity
rustchain audit verify

# Export audit data
rustchain audit export --format json --output audit_dump.json

# Show audit statistics
rustchain audit stats
```

### Configuration Management
```bash
# Show current configuration
rustchain config show

# Validate configuration
rustchain config validate

# Initialize default configuration
rustchain config init
```

**Example Configuration Output:**
```yaml
# rustchain config show
server:
  host: "127.0.0.1"
  port: 8080
  workers: 4

llm:
  providers:
    openai:
      enabled: true
      model: "gpt-4"
    anthropic:
      enabled: true
      model: "claude-3-sonnet"

safety:
  enforce_policies: true
  sandbox_enabled: true
  max_execution_time: 300

storage:
  type: "sqlite"
  path: "./rustchain.db"

memory:
  type: "in_memory"
  capacity_mb: 512
```

### Feature Detection
```bash
# List all features and their status
rustchain features list

# Filter by category
rustchain features list --category llm
rustchain features list --category enterprise

# Show only available features
rustchain features list --available-only

# Check specific feature
rustchain features check agent
rustchain features check compliance

# Show feature summary
rustchain features summary

# Show upgrade recommendations
rustchain features upgrade
```

## 🐛 Troubleshooting

### Common Issues & Solutions

#### Command Not Found
```bash
$ rustchain: command not found

Solution:
# Ensure RustChain is in your PATH
export PATH="$PATH:/usr/local/bin"

# Or use full path
/usr/local/bin/rustchain --version
```

#### Mission Validation Errors
```bash
$ rustchain mission validate fails

Solution:
# Check YAML syntax
rustchain mission validate mission.yaml
# Fix errors shown in output
# Common issues: indentation, missing required fields, unknown step types
```

#### LLM Connection Issues
```bash
$ rustchain llm test fails

Solution:
# Check API keys
export OPENAI_API_KEY="your-key-here"
export ANTHROPIC_API_KEY="your-key-here"

# Test connectivity
curl -H "Authorization: Bearer $OPENAI_API_KEY" https://api.openai.com/v1/models

# Check configuration
rustchain config show
```

#### Permission Denied Errors
```bash
$ Permission denied when running tools

Solution:
# Check sandbox configuration
rustchain policy status

# Verify file permissions
ls -la /path/to/files

# Run with appropriate user permissions
```

### Debug Mode
```bash
# Enable verbose logging
RUST_LOG=debug rustchain run mission.yaml

# Show additional verbose information
RUST_LOG=debug rustchain mission validate mission.yaml

# Trace execution steps
RUST_LOG=trace rustchain run mission.yaml --dry-run
```

### Performance Tuning
```bash
# Monitor execution time
time rustchain run mission.yaml

# Profile memory usage
RUST_LOG=info rustchain run large_mission.yaml

# Optimize for speed
rustchain run mission.yaml --skip-safety  # Use carefully!
```

## 📚 Best Practices

### Mission Design
1. **Start Small**: Begin with simple missions and add complexity gradually
2. **Validate Early**: Always validate missions before execution
3. **Use Dry Runs**: Test missions with `--dry-run` first
4. **Safety First**: Don't skip safety checks unless absolutely necessary

### LLM Usage
1. **Be Specific**: Provide clear, detailed prompts
2. **Set Appropriate Temperature**: Low (0.1-0.3) for factual, high (0.7-1.2) for creative
3. **Choose Right Model**: GPT-4 for complex reasoning, GPT-3.5 for speed
4. **Monitor Costs**: Track API usage and costs

### Security
1. **Review Policies**: Regularly check `rustchain policy status`
2. **Monitor Audit Logs**: Use `rustchain audit stats` frequently
3. **Validate Inputs**: Always validate external data
4. **Use Sandboxes**: Isolate untrusted operations

### Performance
1. **Cache Results**: Reuse LLM responses when possible
2. **Batch Operations**: Group similar tasks together
3. **Monitor Resources**: Check memory and CPU usage
4. **Optimize Models**: Use smaller models for simple tasks

---

*For additional help, see [API Reference](API_REFERENCE.md), [Deployment Guide](DEPLOYMENT.md), or visit [GitHub Discussions](https://github.com/rustchain-community/rustchain-community/discussions).*