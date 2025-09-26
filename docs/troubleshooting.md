# 🛠️ RustChain Error Handling Guide

*Complete guide to understanding and resolving RustChain errors*

## 📋 Table of Contents

- [Understanding Error Messages](#understanding-error-messages)
- [Common Error Categories](#common-error-categories)
- [Troubleshooting Workflow](#troubleshooting-workflow)
- [Error Recovery Strategies](#error-recovery-strategies)
- [Prevention Best Practices](#prevention-best-practices)

## 🔍 Understanding Error Messages

RustChain provides enhanced error messages with:

- **🎯 Clear Title**: What went wrong
- **📝 Description**: Detailed explanation
- **📋 Context**: Additional relevant information
- **💡 Suggestions**: Actionable steps to fix the problem
- **🔧 Help Commands**: Specific commands to diagnose or resolve

### Error Message Format

```
🔧 Configuration Missing
Required configuration key 'api_key' is not set.

📋 Context:
RustChain needs this configuration to operate properly.

💡 Suggestions:
  • Add 'api_key' to your configuration file
  • Set environment variable: export RUSTCHAIN_API_KEY
  • Run 'rustchain config init' to create default configuration
  • Check documentation: docs/CONFIGURATION.md

🔧 Try this command:
  rustchain config show | grep api_key
```

## 🗂️ Common Error Categories

### Configuration Errors

#### Missing API Keys
**Problem**: LLM provider API keys not configured
```bash
# Error
🔐 AI Authentication Failed
OpenAI authentication failed.

# Solutions
export OPENAI_API_KEY="your-api-key-here"
rustchain config init
rustchain llm test openai
```

#### Invalid Configuration Format
**Problem**: YAML/TOML syntax errors in config files
```bash
# Error
📝 Configuration Parse Error
Configuration file has syntax errors.

# Solutions
# Check indentation (use spaces, not tabs)
# Validate with online YAML/JSON validator
rustchain config validate
```

#### Missing Configuration Files
**Problem**: Configuration files not found
```bash
# Error
📄 Configuration File Missing
Configuration file not found: /path/to/config.yaml

# Solutions
rustchain config init
# Check file path and permissions
ls -la rustchain.toml
```

### Mission Execution Errors

#### Mission File Not Found
**Problem**: Mission YAML file doesn't exist or isn't accessible
```bash
# Error
🚀 Mission Not Found
Mission 'nonexistent.yaml' not found.

# Solutions
ls examples/                    # List available missions
rustchain mission list          # Show all missions
pwd                            # Check current directory
```

#### Mission Step Failures
**Problem**: Individual mission steps fail during execution
```bash
# Error
❌ Mission Step Failed
Step 'llm_call' failed in mission 'analysis'.

# Solutions
rustchain mission validate mission.yaml  # Validate before running
rustchain run mission.yaml --dry-run     # Test without execution
rustchain safety validate mission.yaml   # Check safety
```

#### Dependency Cycles
**Problem**: Circular dependencies between mission steps
```bash
# Error
🔄 Dependency Cycle Detected
Circular dependency in mission 'complex_analysis'.

# Solutions
# Review 'depends_on' fields in mission YAML
# Create linear dependency chain
rustchain mission validate mission.yaml
```

### Tool and LLM Errors

#### Tool Not Found
**Problem**: Requested tool doesn't exist or isn't enabled
```bash
# Error
🔧 Tool Not Found
Tool 'custom_analyzer' is not available.

# Solutions
rustchain tools list           # See available tools
rustchain features list        # Check enabled features
# Enable required features: --features tools
```

#### LLM Service Unavailable
**Problem**: AI provider service is down or unreachable
```bash
# Error
🤖 AI Service Unavailable
OpenAI service is currently unavailable.

# Solutions
curl https://api.openai.com/v1/models  # Test connectivity
rustchain llm test                     # Test all providers
# Try different provider or wait and retry
```

#### Rate Limit Exceeded
**Problem**: Too many API requests to LLM provider
```bash
# Error
🚦 Rate Limit Exceeded
OpenAI rate limit exceeded.

# Solutions
# Wait before retrying (usually 60 seconds)
# Upgrade API plan for higher limits
# Use different provider temporarily
rustchain llm models --provider anthropic
```

### Security and Safety Errors

#### Policy Violations
**Problem**: Operation blocked by security policies
```bash
# Error
🛡️ Security Violation
Security policy violation detected.

# Solutions
rustchain policy status        # Check active policies
rustchain safety validate mission.yaml
# Review operation for security implications
```

#### Safety Validation Failures
**Problem**: Mission contains potentially unsafe operations
```bash
# Error
⚠️ Mission has safety concerns:
🔴 Critical: File system access outside sandbox
🟡 Warning: Network access to external services

# Solutions
# Review mission steps for security risks
# Use --skip-safety flag only for trusted missions
rustchain safety validate mission.yaml --strict
```

### System and Resource Errors

#### Memory Exhausted
**Problem**: System running out of memory
```bash
# Error
💻 System Resources Exhausted
System resource 'memory' exhausted.

# Solutions
# Free up system memory
# Reduce mission complexity
# Increase system memory limits
# Run fewer concurrent operations
```

#### File Permission Errors
**Problem**: Insufficient permissions to access files
```bash
# Error
🚫 Permission Denied
Tool 'file_reader' access denied.

# Solutions
chmod +r filename              # Make file readable
ls -la filename               # Check permissions
# Run with appropriate user privileges
rustchain policy status       # Check access policies
```

## 🔄 Troubleshooting Workflow

### Step 1: Identify Error Type
Look at the error icon and title to understand the category:
- 🔧 Configuration issues
- 🚀 Mission problems
- 🤖 LLM/AI issues
- 🔧 Tool problems
- 🛡️ Security concerns
- 💻 System resources

### Step 2: Read Context and Suggestions
Always read the full error message:
- **Context**: Understand why the error occurred
- **Suggestions**: Follow the recommended actions
- **Help Command**: Run the suggested diagnostic command

### Step 3: Apply Systematic Fixes
1. **Quick Fixes**: Try the first 1-2 suggestions
2. **Validation**: Use built-in validation commands
3. **Testing**: Test with simpler scenarios first
4. **Documentation**: Check relevant documentation

### Step 4: Escalate if Needed
If basic troubleshooting doesn't work:
1. Run diagnostic commands
2. Check system logs
3. Search documentation and discussions
4. Report issue with full error details

## 🛡️ Error Recovery Strategies

### Configuration Recovery
```bash
# Backup current config
cp rustchain.toml rustchain.toml.backup

# Reset to defaults
rustchain config init

# Validate and test
rustchain config validate
rustchain llm test
```

### Mission Recovery
```bash
# Validate mission structure
rustchain mission validate mission.yaml

# Test with dry run
rustchain run mission.yaml --dry-run

# Check safety
rustchain safety validate mission.yaml

# Execute with caution
rustchain run mission.yaml
```

### System Recovery
```bash
# Check system resources
df -h                          # Disk space
free -h                        # Memory
ps aux | grep rustchain       # Processes

# Clean up if needed
rustchain audit export --output cleanup.json
# Review and clean old data
```

## ✅ Prevention Best Practices

### 1. Configuration Management
- Use version control for configuration files
- Validate configs before deployment
- Keep API keys secure and rotated
- Document configuration changes

```bash
# Good practices
rustchain config validate      # Always validate
git add rustchain.toml        # Version control
rustchain config show         # Review settings
```

### 2. Mission Development
- Start with simple missions
- Use dry-run mode for testing
- Validate before execution
- Handle dependencies carefully

```bash
# Mission development workflow
rustchain mission validate mission.yaml
rustchain run mission.yaml --dry-run
rustchain safety validate mission.yaml
rustchain run mission.yaml
```

### 3. Error Monitoring
- Enable audit logging
- Monitor system resources
- Review error patterns
- Set up alerts for critical issues

```bash
# Monitoring commands
rustchain audit stats         # Review error patterns
rustchain build status        # System health
rustchain policy status       # Security compliance
```

### 4. Dependency Management
- Keep RustChain updated
- Verify tool dependencies
- Test after updates
- Maintain backup configurations

```bash
# Dependency checks
rustchain --version           # Check version
rustchain features list       # Verify features
rustchain tools list          # Check available tools
```

## 🔧 Advanced Debugging

### Debug Mode
Enable verbose logging for detailed error information:
```bash
export RUST_LOG=debug
rustchain run mission.yaml

# Or for specific components
export RUST_LOG=rustchain::core::executor=debug
```

### System Diagnostics
```bash
# Comprehensive system check
rustchain build dashboard      # Overall health
rustchain features summary     # Feature status
rustchain audit verify        # Audit integrity
rustchain policy validate     # Policy health
```

### Performance Debugging
```bash
# Profile mission execution
time rustchain run mission.yaml

# Memory usage monitoring
/usr/bin/time -v rustchain run mission.yaml

# System resource monitoring
rustchain build status
```

## 📞 Getting Help

### Self-Service Resources
1. **Built-in Help**: `rustchain <command> --help`
2. **Documentation**: Check docs/ directory
3. **Examples**: Review examples/ directory
4. **Diagnostics**: Use built-in diagnostic commands

### Community Support
1. **GitHub Discussions**: Ask questions and share solutions
2. **Issue Tracker**: Report bugs with full error details
3. **Documentation**: Contribute improvements
4. **Examples**: Share working mission examples

### Error Reporting
When reporting errors, include:
- Full error message (with colors/formatting)
- Command that caused the error
- System information (`rustchain --version`)
- Configuration details (sanitized)
- Steps to reproduce

```bash
# Collect diagnostic info
rustchain --version
rustchain features list
rustchain config show  # Remove sensitive data
rustchain build status
```

## 📚 Error Message Reference

### Icons and Meanings
- 🔧 **Configuration**: Setup and settings issues
- 🚀 **Mission**: Mission file or execution problems
- 🤖 **AI/LLM**: AI provider or model issues
- 🔧 **Tools**: Tool execution or availability
- 🛡️ **Security**: Safety and policy violations
- 💻 **System**: Resource or infrastructure problems
- ⚡ **Execution**: Runtime or process errors
- 📄 **File**: File system or IO operations
- 🌐 **Network**: Connectivity or API issues
- ❓ **Unknown**: Unexpected or uncategorized errors

### Severity Levels
- 🚨 **Critical**: System cannot continue
- ❌ **Error**: Operation failed
- ⚠️ **Warning**: Issue detected but operation continues
- ℹ️ **Info**: Informational message

---

*This guide covers the most common RustChain errors and their solutions. For specific issues not covered here, check the API documentation or community discussions.*