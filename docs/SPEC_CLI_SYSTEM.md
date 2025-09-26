# RustChain CLI System Specification

## Overview

The RustChain CLI provides a comprehensive command-line interface for AI agent orchestration, mission execution, safety validation, and system management. Built with clap-rs, it offers both simple single commands and complex multi-step workflows.

## Version

- **CLI Version**: 1.0.0
- **RustChain Version**: 0.1.0+
- **Specification**: GitHub Spec Kit compliant

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     RustChain CLI System                    │
├─────────────────────────────────────────────────────────────┤
│  rustchain [COMMAND] [SUBCOMMAND] [OPTIONS]                │
├─────────────────────────────────────────────────────────────┤
│  Core Commands                                              │
│  ├── interactive          (Conversational mode)            │
│  ├── run                  (Mission execution)              │
│  ├── mission              (Mission management)             │
│  ├── policy               (Security governance)            │
│  ├── safety               (Validation & checks)            │
│  ├── tools                (Tool ecosystem)                 │
│  ├── llm                  (AI model operations)            │
│  └── audit                (Audit & compliance)             │
├─────────────────────────────────────────────────────────────┤
│  Advanced Commands                                          │
│  ├── rag                  (Document processing)            │
│  ├── sandbox              (Isolated execution)             │
│  ├── server               (API server management)          │
│  ├── config               (Configuration management)       │
│  ├── build                (Build dashboard)                │
│  └── enterprise           (Enterprise features)            │
└─────────────────────────────────────────────────────────────┘
```

## Core Commands

### 1. Interactive Mode

**Command**: `rustchain interactive`

Starts an interactive conversational mode similar to Claude Code, providing natural language interface for RustChain operations.

**Features**:
- Natural conversation with AI models
- Dynamic mission creation and execution
- Real-time help and guidance
- Capability exploration

**Usage**:
```bash
$ rustchain interactive
> create a mission to analyze my Rust codebase
> run the generated mission
> show me performance metrics
```

### 2. Mission Execution

**Command**: `rustchain run [OPTIONS] <MISSION>`

Executes RustChain missions with comprehensive safety checks and validation.

**Arguments**:
- `<MISSION>` - Path to YAML mission file

**Options**:
- `-d, --dry-run` - Validate and plan execution without running tools
- `-s, --skip-safety` - Skip safety validation (use with caution)

**Usage**:
```bash
# Safe execution with validation
rustchain run examples/hello_world.yaml --dry-run
rustchain run examples/hello_world.yaml

# Skip safety for trusted missions
rustchain run trusted_mission.yaml --skip-safety
```

**Mission File Format**:
```yaml
name: "Hello World"
description: "Simple demonstration"
version: "1.0"
steps:
  - id: "greet"
    step_type: "llm"
    parameters:
      provider: "openai"
      model: "gpt-4"
      prompt: "Say hello in a creative way"
```

### 3. Mission Management

**Command**: `rustchain mission <ACTION>`

Comprehensive mission file management and validation.

#### Subcommands

##### `rustchain mission list`
Lists available example missions.

##### `rustchain mission validate <FILE>`
Validates mission file syntax and structure.

**Arguments**:
- `<FILE>` - Path to mission file

**Validation Checks**:
- ✅ YAML syntax correctness
- ✅ Required fields present
- ✅ Step dependencies resolved
- ✅ Parameter requirements met

##### `rustchain mission info <FILE>`
Shows detailed mission information and analysis.

**Usage**:
```bash
rustchain mission list
rustchain mission validate mission.yaml
rustchain mission info mission.yaml
```

### 4. Policy Management

**Command**: `rustchain policy <ACTION>`

Security policy management for safe AI operations.

#### Subcommands

##### `rustchain policy list`
Lists all active policies.

##### `rustchain policy validate`
Validates current policy configuration.

##### `rustchain policy status`
Shows comprehensive policy status.

**Policy Types**:
- 📁 **File Access** - Control file system operations
- 🌐 **Network Policy** - Manage external connections
- ⚙️ **Command Execution** - Restrict system commands
- 🤖 **LLM Safety** - Filter AI interactions

**Usage**:
```bash
rustchain policy list
rustchain policy status
rustchain policy validate
```

### 5. Safety Validation

**Command**: `rustchain safety <ACTION>`

Comprehensive security analysis and risk assessment.

#### Subcommands

##### `rustchain safety validate <MISSION> [OPTIONS]`
Validates mission security and safety.

**Arguments**:
- `<MISSION>` - Path to mission file

**Options**:
- `--strict` - Use strict mode (fail on warnings)

##### `rustchain safety check [OPTIONS]`
Runs comprehensive system safety checks.

**Options**:
- `--include-policies` - Include policy validation

##### `rustchain safety report <MISSION>`
Generates detailed safety report.

**Risk Levels**:
- 🟢 **LOW** - Safe to execute
- 🟡 **MEDIUM** - Review recommended
- 🟠 **HIGH** - Caution required
- 🔴 **CRITICAL** - Do not execute

**Usage**:
```bash
rustchain safety validate mission.yaml --strict
rustchain safety check --include-policies
rustchain safety report mission.yaml
```

### 6. Tool Management

**Command**: `rustchain tools <ACTION>` (requires `tools` feature)

Rich ecosystem of tools for various operations.

#### Subcommands

##### `rustchain tools list`
Lists all available tools with descriptions.

##### `rustchain tools info <NAME>`
Shows detailed tool information.

**Arguments**:
- `<NAME>` - Tool name

##### `rustchain tools execute <NAME> [OPTIONS]`
Executes a tool directly.

**Arguments**:
- `<NAME>` - Tool name

**Options**:
- `-p, --params <JSON>` - Tool parameters as JSON

**Tool Categories**:
- **File Operations**: read, write, edit files
- **Network Operations**: HTTP requests, web scraping
- **System Commands**: shell execution, process management
- **Data Processing**: JSON, CSV, XML manipulation
- **AI Operations**: embeddings, classification

**Usage**:
```bash
rustchain tools list
rustchain tools info file_create
rustchain tools execute file_create --params '{"path":"test.txt","content":"Hello"}'
```

### 7. LLM Operations

**Command**: `rustchain llm <ACTION>` (requires `llm` feature)

AI model interactions and management.

#### Subcommands

##### `rustchain llm models [OPTIONS]`
Lists available models and providers.

**Options**:
- `-p, --provider <PROVIDER>` - Filter by specific provider

##### `rustchain llm chat <MESSAGE> [OPTIONS]`
Chat directly with an LLM.

**Arguments**:
- `<MESSAGE>` - Message to send

**Options**:
- `-m, --model <MODEL>` - Specific model to use
- `-p, --provider <PROVIDER>` - Provider to use
- `--max-tokens <NUM>` - Maximum response tokens
- `--temperature <FLOAT>` - Creativity level (0.0-1.0)

##### `rustchain llm test [OPTIONS]`
Tests LLM connectivity and functionality.

**Options**:
- `--provider <PROVIDER>` - Test specific provider

**Supported Providers**:
- **OpenAI**: GPT-4, GPT-3.5-turbo, etc.
- **Anthropic**: Claude-3, Claude-2, etc.
- **Ollama**: Local models
- **Custom**: User-defined providers

**Usage**:
```bash
rustchain llm models --provider openai
rustchain llm chat "Explain quantum computing" --model gpt-4
rustchain llm test --provider ollama
```

### 8. Audit Operations

**Command**: `rustchain audit <ACTION>`

Comprehensive audit logging and compliance tracking.

#### Subcommands

##### `rustchain audit query [OPTIONS]`
Queries audit entries with filtering.

**Options**:
- `--start-time <ISO8601>` - Start time filter
- `--end-time <ISO8601>` - End time filter
- `--event-types <TYPES>` - Filter by event types
- `-l, --limit <NUM>` - Maximum results (default: 10)
- `--offset <NUM>` - Skip number of results

##### `rustchain audit export [OPTIONS]`
Exports audit data in various formats.

**Options**:
- `-f, --format <FORMAT>` - Export format (json, csv, xml)
- `-o, --output <FILE>` - Output file path

##### `rustchain audit report [OPTIONS]`
Generates comprehensive audit reports.

**Options**:
- `--format <FORMAT>` - Report format
- `--template <FILE>` - Custom report template

**Usage**:
```bash
rustchain audit query --start-time "2024-01-01T00:00:00Z" --limit 50
rustchain audit export --format json --output audit_log.json
rustchain audit report --format html
```

## Advanced Commands

### 9. RAG Operations

**Command**: `rustchain rag <ACTION>` (requires `rag` feature)

Retrieval-Augmented Generation document processing.

#### Subcommands

##### `rustchain rag add [OPTIONS]`
Adds documents to the RAG system.

**Options**:
- `-i, --id <ID>` - Document ID
- `-f, --file <FILE>` - Document file path
- `-m, --metadata <JSON>` - Document metadata

##### `rustchain rag search <QUERY> [OPTIONS]`
Searches documents in the RAG system.

**Arguments**:
- `<QUERY>` - Search query

**Options**:
- `-l, --limit <NUM>` - Maximum results
- `--threshold <FLOAT>` - Similarity threshold

##### `rustchain rag list`
Lists all documents in the RAG system.

**Usage**:
```bash
rustchain rag add --id doc1 --file document.pdf --metadata '{"type":"manual"}'
rustchain rag search "installation instructions" --limit 5
rustchain rag list
```

### 10. Sandbox Operations

**Command**: `rustchain sandbox <ACTION>` (requires `sandbox` feature)

Isolated execution environment management.

#### Subcommands

##### `rustchain sandbox create`
Creates a new sandbox session.

##### `rustchain sandbox execute [OPTIONS] <COMMAND> [ARGS]`
Executes commands in a sandbox.

**Options**:
- `-s, --session <ID>` - Session ID

**Arguments**:
- `<COMMAND>` - Command to execute
- `[ARGS]` - Command arguments

##### `rustchain sandbox write [OPTIONS] <PATH> <CONTENT>`
Writes files to sandbox.

**Options**:
- `-s, --session <ID>` - Session ID

**Usage**:
```bash
session=$(rustchain sandbox create)
rustchain sandbox execute --session $session ls -la
rustchain sandbox write --session $session test.txt "Hello Sandbox"
```

### 11. Server Management

**Command**: `rustchain server <ACTION>` (requires `server` feature)

API server management for RustChain.

#### Subcommands

##### `rustchain server start [OPTIONS]`
Starts the API server.

**Options**:
- `--host <HOST>` - Server host (default: 127.0.0.1)
- `--port <PORT>` - Server port (default: 8080)
- `--cors` - Enable CORS

##### `rustchain server config`
Shows server configuration.

**Usage**:
```bash
rustchain server start --host 0.0.0.0 --port 3000 --cors
rustchain server config
```

### 12. Configuration Management

**Command**: `rustchain config <ACTION>`

System configuration management.

#### Subcommands

##### `rustchain config show`
Displays current configuration.

##### `rustchain config validate`
Validates configuration files.

##### `rustchain config init`
Initializes default configuration.

**Usage**:
```bash
rustchain config init
rustchain config show
rustchain config validate
```

### 13. Build Dashboard

**Command**: `rustchain build <ACTION>`

Build system monitoring and health dashboard.

#### Subcommands

##### `rustchain build dashboard`
Shows build dashboard with system health.

##### `rustchain build status`
Generates build status report.

##### `rustchain build update`
Updates dashboard with current test results.

##### `rustchain build save [OPTIONS]`
Saves dashboard to file.

**Options**:
- `-o, --output <FILE>` - Output file (default: build_dashboard.json)

**Usage**:
```bash
rustchain build dashboard
rustchain build update
rustchain build save --output status.json
```

## Enterprise Commands

### 14. Enterprise Features

**Command**: `rustchain enterprise <ACTION>` (requires `enterprise` feature)

Enterprise-grade features for production deployments.

#### Authentication Management

**Command**: `rustchain enterprise auth <ACTION>`

##### Subcommands

- `init-jwt [--secret <SECRET>]` - Initialize JWT authentication
- `setup-oauth2 <PROVIDER> --client-id <ID>` - Configure OAuth2
- `setup-rbac --config <FILE>` - Configure role-based access

#### Compliance Operations

**Command**: `rustchain enterprise compliance <ACTION>`

##### Subcommands

- `verify <MISSION> [--standard <STD>] [--all-standards]` - Verify compliance
- `list-standards` - List available standards
- `report <MISSION> [--format <FMT>]` - Generate compliance reports

#### Monitoring Features

**Command**: `rustchain enterprise monitoring <ACTION>`

##### Subcommands

- `start-metrics [--port <PORT>]` - Start metrics collection
- `dashboard` - Show performance dashboard
- `setup-alerts --config <FILE>` - Configure alerting

**Usage**:
```bash
# Authentication
rustchain enterprise auth init-jwt --secret my-secret
rustchain enterprise auth setup-oauth2 google --client-id 123456

# Compliance
rustchain enterprise compliance verify mission.yaml --all-standards
rustchain enterprise compliance report mission.yaml --format pdf

# Monitoring
rustchain enterprise monitoring start-metrics --port 9090
rustchain enterprise monitoring dashboard
```

## Global Options

All commands support these global options:

- `-h, --help` - Show help information
- `-V, --version` - Show version information
- `--config <FILE>` - Use custom configuration file
- `--log-level <LEVEL>` - Set logging level (error, warn, info, debug, trace)
- `--no-color` - Disable colored output
- `--json` - Output in JSON format (where applicable)

## Exit Codes

The CLI uses standard exit codes:

- `0` - Success
- `1` - General error
- `2` - Misuse of shell command
- `64` - Command not found
- `65` - Data format error
- `66` - Cannot open input
- `67` - User not found
- `73` - Cannot create output file
- `74` - I/O error
- `78` - Configuration error

## Environment Variables

The CLI respects these environment variables:

- `RUSTCHAIN_CONFIG` - Default configuration file path
- `RUSTCHAIN_LOG_LEVEL` - Default log level
- `RUSTCHAIN_NO_COLOR` - Disable colored output
- `OPENAI_API_KEY` - OpenAI API key
- `ANTHROPIC_API_KEY` - Anthropic API key
- `RUSTCHAIN_POLICY_FILE` - Default policy file

## Configuration Files

### Default Locations

- **Linux/macOS**: `~/.config/rustchain/config.toml`
- **Windows**: `%APPDATA%\rustchain\config.toml`
- **Custom**: Use `--config` flag or `RUSTCHAIN_CONFIG` environment variable

### Configuration Format

```toml
[general]
log_level = "info"
colored_output = true

[llm]
default_provider = "openai"
default_model = "gpt-4"
timeout_seconds = 30

[tools]
policy_file = "~/.config/rustchain/policies.yaml"
sandbox_enabled = true

[safety]
strict_mode = false
auto_validate = true

[audit]
enable_logging = true
log_file = "~/.config/rustchain/audit.log"
```

## Feature Flags

Different CLI features require specific Cargo features:

```toml
[dependencies]
rustchain = { 
    version = "0.1.0", 
    features = [
        "cli",          # Basic CLI (always enabled)
        "tools",        # Tool management commands
        "llm",          # LLM operations
        "rag",          # RAG document processing
        "sandbox",      # Sandbox execution
        "server",       # API server
        "enterprise",   # Enterprise features
        "compliance"    # Compliance verification
    ] 
}
```

## Integration Examples

### CI/CD Pipeline

```yaml
# GitHub Actions
- name: Validate Mission
  run: rustchain mission validate mission.yaml

- name: Safety Check
  run: rustchain safety validate mission.yaml --strict

- name: Execute Mission
  run: rustchain run mission.yaml
```

### Shell Scripting

```bash
#!/bin/bash
# Automated mission execution with safety checks

echo "Validating mission..."
if ! rustchain mission validate "$1"; then
    echo "Mission validation failed"
    exit 1
fi

echo "Running safety checks..."
if ! rustchain safety validate "$1" --strict; then
    echo "Safety validation failed"
    exit 1
fi

echo "Executing mission..."
rustchain run "$1"
```

### Docker Integration

```dockerfile
FROM rust:1.70
RUN cargo install rustchain --features "cli,llm,tools"
COPY mission.yaml /app/
WORKDIR /app
CMD ["rustchain", "run", "mission.yaml"]
```

## Performance Characteristics

- **Startup Time**: ~100ms (basic commands)
- **Mission Validation**: ~50ms (typical mission)
- **Tool Execution**: Varies by tool (10ms-5s)
- **LLM Operations**: 500ms-10s (depends on provider)
- **Memory Usage**: ~10MB base + operation-specific

## Best Practices

### Mission Development
1. Always validate with `rustchain mission validate` first
2. Use `--dry-run` before executing new missions
3. Test safety validation with `rustchain safety validate --strict`
4. Keep mission files in version control

### Production Deployment
1. Use configuration files instead of command-line arguments
2. Enable audit logging for compliance
3. Configure appropriate safety policies
4. Use enterprise features for production environments

### Debugging
1. Use `--log-level debug` for detailed information
2. Enable verbose mode where available
3. Check audit logs for execution history
4. Use `rustchain config validate` to check configuration

## Security Considerations

1. **Input Validation**: All inputs are validated before processing
2. **Safe Defaults**: Conservative defaults for safety-critical operations
3. **Policy Enforcement**: Mandatory policy checks for sensitive operations
4. **Audit Trails**: Comprehensive logging of all operations
5. **Privilege Separation**: Minimal required permissions for operations

---

*This specification follows the GitHub Spec Kit standards for CLI documentation and is maintained alongside the RustChain codebase.*