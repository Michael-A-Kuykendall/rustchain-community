# RustChain: Complete AI Agent Framework Guide

**For AI Assistants**: This is your comprehensive guide to using RustChain effectively across any project. Master these concepts to provide expert-level assistance with RustChain operations.

---

## Core Architecture Understanding

### What RustChain Is
RustChain is a **mission-based AI orchestration framework** built in Rust that executes complex workflows through YAML mission definitions. Think of it as "infrastructure as code" for AI operations.

### Key Concepts
- **Mission**: A YAML file defining a complete workflow with multiple steps
- **Step**: Individual operations (LLM calls, file operations, HTTP requests, tool executions)
- **DAG Execution**: Steps execute in dependency order with parallel processing where possible
- **Context**: Shared state between steps, supports variable substitution
- **Policy Engine**: Security and compliance validation for enterprise deployments

---

## Mission Structure Deep Dive

### Basic Mission Template
```yaml
name: "Example Mission"
description: "Comprehensive workflow example"
version: "1.0"
config:
  fail_fast: false          # Continue on step failures
  max_parallel: 4          # Concurrent step limit
  timeout_seconds: 3600    # Mission timeout
steps:
  - id: "unique_step_id"
    name: "Human-readable step name"
    step_type: "llm"        # Step type determines behavior
    depends_on: []          # Dependencies (other step IDs)
    continue_on_error: false
    timeout_seconds: 300
    parameters:
      # Step-specific configuration
```

### Step Types and Usage

#### 1. LLM Steps - AI Text Generation
```yaml
step_type: "llm"
parameters:
  provider: "ollama"        # ollama, openai, anthropic, shimmy
  model: "llama3.2:latest"  # Provider-specific model
  prompt: "Analyze this data: {{previous_step_output}}"
  max_tokens: 2000
  temperature: 0.7
  system_prompt: "You are a data analyst"
```

#### 2. Agent Steps - Complex AI Reasoning
```yaml
step_type: "agent"
parameters:
  agent_type: "reasoning"   # reasoning, creative, analytical
  task: "Create a comprehensive report on {{data_source}}"
  tools: ["file_ops", "web_search", "calculator"]
  max_iterations: 10
  memory_enabled: true
```

#### 3. Tool Steps - System Operations
```yaml
step_type: "tool"
parameters:
  tool_name: "file_operations"
  action: "create_file"
  path: "/tmp/output.txt"
  content: "{{processed_data}}"
  mode: "write"             # write, append, read
```

#### 4. HTTP Steps - API Integration
```yaml
step_type: "http"
parameters:
  method: "POST"
  url: "https://api.example.com/data"
  headers:
    Content-Type: "application/json"
    Authorization: "Bearer {{api_token}}"
  body: |
    {
      "data": "{{input_data}}",
      "timestamp": "{{current_time}}"
    }
  timeout_seconds: 30
```

#### 5. Chain Steps - Complex Workflows
```yaml
step_type: "chain"
parameters:
  chain_type: "sequential"  # sequential, parallel, conditional
  steps:
    - llm_call: "analyze"
    - data_transform: "normalize"
    - output_format: "json"
```

#### 6. CreateFile Steps - File Generation
```yaml
step_type: "create_file"
parameters:
  path: "./reports/{{date}}/analysis.md"
  content: |
    # Analysis Report
    Generated: {{timestamp}}
    Data: {{analysis_results}}
  template: true            # Enable variable substitution
```

---

## Advanced Execution Patterns

### Dependency Management
```yaml
steps:
  - id: "data_fetch"
    step_type: "http"
    # No dependencies - runs first
    
  - id: "data_process"
    step_type: "llm" 
    depends_on: ["data_fetch"]  # Waits for data_fetch
    
  - id: "report_generate"
    step_type: "agent"
    depends_on: ["data_process"]
    
  - id: "notification"
    step_type: "http"
    depends_on: ["report_generate"]
    continue_on_error: true   # Send notification even if report fails
```

### Variable Substitution System
```yaml
# Context variables available in all steps:
# {{step_id.output}} - Output from specific step
# {{global.variable_name}} - Global context variables
# {{env.VARIABLE}} - Environment variables
# {{timestamp}} - Current timestamp
# {{date}} - Current date
# {{uuid}} - Generated UUID

parameters:
  prompt: "Process this data: {{data_fetch.output}} at {{timestamp}}"
  file_path: "./logs/{{date}}/{{uuid}}.log"
```

### Error Handling Strategies
```yaml
config:
  fail_fast: false          # Continue mission on step failures
  
steps:
  - id: "critical_step"
    continue_on_error: false # Mission fails if this step fails
    
  - id: "optional_step" 
    continue_on_error: true  # Mission continues if this step fails
    
  - id: "cleanup_step"
    depends_on: ["critical_step", "optional_step"]
    # Runs after both, regardless of optional_step success
```

---

## CLI Command Mastery

### Essential Commands

#### Mission Execution
```bash
# Execute mission with verbose output
rustchain run mission.yaml --verbose

# Validate mission syntax without execution
rustchain mission validate mission.yaml

# List available example missions
rustchain mission list

# Check mission dependencies
rustchain mission deps mission.yaml
```

#### Interactive Mode
```bash
# Start conversational AI mode
rustchain interactive

# Interactive with specific provider
rustchain interactive --provider ollama --model llama3.2
```

#### Policy and Safety
```bash
# Validate mission against security policies
rustchain policy validate mission.yaml

# Run safety checks
rustchain safety check mission.yaml

# Audit mission execution
rustchain audit mission mission.yaml
```

#### Transpilation (Universal Workflow Conversion)
```bash
# Convert mission to different platforms
rustchain transpile mission.yaml --output airflow
rustchain transpile mission.yaml --output github-actions
rustchain transpile mission.yaml --output kubernetes
rustchain transpile mission.yaml --output docker-compose
```

#### Configuration Management
```bash
# Initialize RustChain configuration
rustchain config init

# Set default LLM provider
rustchain config set llm.default_provider ollama

# Check current configuration
rustchain config show
```

### Advanced CLI Usage

#### Feature Detection
```bash
# Check available features in current build
rustchain features summary

# Test specific feature availability
rustchain features test llm
rustchain features test transpiler
```

#### Benchmarking and Performance
```bash
# Run performance benchmarks
rustchain benchmark performance

# Compare with other frameworks
rustchain benchmark competitive

# Generate performance report
rustchain benchmark report --output performance.md
```

---

## Enterprise Features Deep Dive

### Security and Compliance
```yaml
# Mission with enterprise security
name: "Secure Data Processing"
config:
  security_policy: "enterprise"
  audit_level: "comprehensive"
  encryption: true
  
steps:
  - id: "secure_fetch"
    step_type: "http"
    parameters:
      url: "{{env.SECURE_API_URL}}"
      headers:
        Authorization: "Bearer {{env.API_TOKEN}}"
      tls_verify: true
      cert_file: "/path/to/cert.pem"
```

### Audit Trails
```yaml
config:
  audit_trail: true
  audit_destination: "/var/log/rustchain/audit.log"
  compliance_standards: ["SOX", "GDPR", "HIPAA"]
```

### Policy Engine Integration
```yaml
# Policy validation configuration
policies:
  data_governance:
    - no_external_apis: true
    - encrypt_at_rest: true
    - log_all_operations: true
  
  resource_limits:
    - max_memory_mb: 1024
    - max_execution_time: 3600
    - max_file_size_mb: 100
```

---

## Integration Patterns

### LLM Provider Configuration

#### Ollama (Local)
```yaml
llm_config:
  provider: "ollama"
  base_url: "http://localhost:11434"
  models:
    - "llama3.2:latest"
    - "codellama:13b"
  default_model: "llama3.2:latest"
```

#### OpenAI
```yaml
llm_config:
  provider: "openai"
  api_key: "{{env.OPENAI_API_KEY}}"
  models: ["gpt-4", "gpt-3.5-turbo"]
  default_model: "gpt-4"
```

#### Anthropic Claude
```yaml
llm_config:
  provider: "anthropic"
  api_key: "{{env.ANTHROPIC_API_KEY}}"
  models: ["claude-3-sonnet", "claude-3-haiku"]
  default_model: "claude-3-sonnet"
```

### Multi-Provider Workflows
```yaml
steps:
  - id: "creative_task"
    step_type: "llm"
    parameters:
      provider: "openai"
      model: "gpt-4"
      prompt: "Create innovative solution for {{problem}}"
      
  - id: "technical_analysis"
    step_type: "llm"
    depends_on: ["creative_task"]
    parameters:
      provider: "anthropic"
      model: "claude-3-sonnet"
      prompt: "Analyze technical feasibility: {{creative_task.output}}"
      
  - id: "local_processing"
    step_type: "llm"
    depends_on: ["technical_analysis"]
    parameters:
      provider: "ollama"
      model: "llama3.2:latest"
      prompt: "Summarize findings: {{technical_analysis.output}}"
```

---

## Advanced Patterns and Best Practices

### Dynamic Mission Generation
```yaml
# Mission that generates other missions
name: "Mission Generator"
steps:
  - id: "analyze_requirements"
    step_type: "llm"
    parameters:
      prompt: "Convert these requirements to RustChain mission: {{requirements}}"
      
  - id: "generate_mission"
    step_type: "create_file"
    depends_on: ["analyze_requirements"]
    parameters:
      path: "./generated/mission_{{uuid}}.yaml"
      content: "{{analyze_requirements.output}}"
      
  - id: "execute_generated"
    step_type: "mission"
    depends_on: ["generate_mission"]
    parameters:
      mission_file: "./generated/mission_{{uuid}}.yaml"
```

### Conditional Execution
```yaml
steps:
  - id: "check_condition"
    step_type: "llm"
    parameters:
      prompt: "Should we proceed with processing? Respond only 'yes' or 'no': {{data}}"
      
  - id: "conditional_processing"
    step_type: "conditional"
    depends_on: ["check_condition"]
    parameters:
      condition: "{{check_condition.output}} == 'yes'"
      if_true:
        step_type: "agent"
        parameters:
          task: "Process the data thoroughly"
      if_false:
        step_type: "create_file"
        parameters:
          path: "./skipped.log"
          content: "Processing skipped due to condition"
```

### Parallel Processing Patterns
```yaml
steps:
  - id: "data_source"
    step_type: "http"
    # Single data source
    
  # These three steps run in parallel
  - id: "analysis_a"
    step_type: "llm"
    depends_on: ["data_source"]
    parameters:
      prompt: "Analyze sentiment: {{data_source.output}}"
      
  - id: "analysis_b"
    step_type: "llm"
    depends_on: ["data_source"]
    parameters:
      prompt: "Extract entities: {{data_source.output}}"
      
  - id: "analysis_c"
    step_type: "llm"
    depends_on: ["data_source"]
    parameters:
      prompt: "Summarize content: {{data_source.output}}"
      
  # This step waits for all analyses
  - id: "combine_results"
    step_type: "llm"
    depends_on: ["analysis_a", "analysis_b", "analysis_c"]
    parameters:
      prompt: |
        Combine these analyses:
        Sentiment: {{analysis_a.output}}
        Entities: {{analysis_b.output}}
        Summary: {{analysis_c.output}}
```

---

## Troubleshooting and Debugging

### Common Issues and Solutions

#### Mission Validation Errors
```bash
# Check mission syntax
rustchain mission validate mission.yaml

# Common issues:
# - Invalid YAML syntax
# - Missing required parameters
# - Circular dependencies
# - Invalid step types
```

#### Execution Failures
```bash
# Run with debug output
RUST_LOG=debug rustchain run mission.yaml

# Check specific step failures
rustchain audit logs --step-id problematic_step
```

#### Provider Connection Issues
```bash
# Test LLM provider connectivity
rustchain config test llm

# Check Ollama status
curl http://localhost:11434/api/tags

# Verify API keys (for external providers)
rustchain config validate
```

### Performance Optimization

#### Mission Design
- Use `max_parallel` to control concurrency
- Set appropriate `timeout_seconds` for each step
- Use `continue_on_error` strategically
- Minimize unnecessary dependencies

#### Resource Management
```yaml
config:
  max_parallel: 8           # Adjust based on system capacity
  memory_limit_mb: 2048    # Prevent memory exhaustion
  disk_cache_enabled: true # Cache LLM responses
```

---

## Real-World Examples

### Complete Data Processing Pipeline
```yaml
name: "Enterprise Data Analysis Pipeline"
description: "Fetch, process, analyze, and report on business data"
version: "1.0"

config:
  fail_fast: false
  max_parallel: 4
  audit_trail: true

steps:
  - id: "fetch_sales_data"
    name: "Retrieve Sales Data"
    step_type: "http"
    parameters:
      method: "GET"
      url: "{{env.SALES_API_URL}}/data"
      headers:
        Authorization: "Bearer {{env.API_TOKEN}}"
    
  - id: "fetch_customer_data"
    name: "Retrieve Customer Data"
    step_type: "http"
    parameters:
      method: "GET"
      url: "{{env.CUSTOMER_API_URL}}/data"
      headers:
        Authorization: "Bearer {{env.API_TOKEN}}"
    
  - id: "data_validation"
    name: "Validate Data Quality"
    step_type: "llm"
    depends_on: ["fetch_sales_data", "fetch_customer_data"]
    parameters:
      provider: "ollama"
      model: "llama3.2:latest"
      prompt: |
        Validate this business data for completeness and consistency:
        Sales Data: {{fetch_sales_data.output}}
        Customer Data: {{fetch_customer_data.output}}
        
        Report any issues found.
    
  - id: "trend_analysis"
    name: "Analyze Sales Trends"
    step_type: "agent"
    depends_on: ["data_validation"]
    parameters:
      agent_type: "analytical"
      task: "Identify sales trends and patterns from the validated data"
      tools: ["calculator", "statistical_analysis"]
    
  - id: "customer_segmentation"
    name: "Segment Customer Base"
    step_type: "llm"
    depends_on: ["data_validation"]
    parameters:
      provider: "anthropic"
      model: "claude-3-sonnet"
      prompt: "Segment customers based on behavior and demographics: {{fetch_customer_data.output}}"
    
  - id: "generate_insights"
    name: "Generate Business Insights"
    step_type: "llm"
    depends_on: ["trend_analysis", "customer_segmentation"]
    parameters:
      provider: "openai"
      model: "gpt-4"
      prompt: |
        Generate actionable business insights by combining:
        Trends: {{trend_analysis.output}}
        Segments: {{customer_segmentation.output}}
    
  - id: "create_report"
    name: "Generate Executive Report"
    step_type: "create_file"
    depends_on: ["generate_insights"]
    parameters:
      path: "./reports/executive_report_{{date}}.md"
      content: |
        # Executive Business Report
        Generated: {{timestamp}}
        
        ## Key Insights
        {{generate_insights.output}}
        
        ## Data Validation Results
        {{data_validation.output}}
        
        ## Recommendations
        Based on the analysis, we recommend the following actions...
    
  - id: "send_notification"
    name: "Notify Stakeholders"
    step_type: "http"
    depends_on: ["create_report"]
    continue_on_error: true
    parameters:
      method: "POST"
      url: "{{env.SLACK_WEBHOOK_URL}}"
      body: |
        {
          "text": "Executive report generated: reports/executive_report_{{date}}.md"
        }
```

---

## Advanced AI Assistant Usage Tips

### For AI Assistants Using This Guide

1. **Mission Design Principles**
   - Start with clear objectives
   - Break complex tasks into discrete steps
   - Use appropriate step types for each operation
   - Design for failure scenarios

2. **Context Awareness**
   - Always check available providers and models
   - Verify environment variables are set
   - Consider resource constraints
   - Plan for error scenarios

3. **Performance Considerations**
   - Use parallel execution where possible
   - Set realistic timeouts
   - Cache results when appropriate
   - Monitor resource usage

4. **Security Best Practices**
   - Never hardcode sensitive data
   - Use environment variables for secrets
   - Enable audit trails for compliance
   - Validate all external inputs

5. **Debugging Approach**
   - Start with mission validation
   - Use verbose logging for issues
   - Test individual steps in isolation
   - Check provider connectivity first

### Quick Reference Commands
```bash
# Essential operations
rustchain run mission.yaml                    # Execute mission
rustchain mission validate mission.yaml       # Validate mission
rustchain interactive                          # Start AI chat
rustchain config show                         # Show configuration
rustchain features summary                    # Check available features

# Debugging
RUST_LOG=debug rustchain run mission.yaml    # Debug execution
rustchain mission deps mission.yaml          # Check dependencies
rustchain config test                         # Test configuration

# Advanced
rustchain transpile mission.yaml --output airflow  # Convert to Airflow
rustchain benchmark performance              # Run benchmarks
rustchain policy validate mission.yaml       # Security validation
```

---

This guide provides comprehensive coverage of RustChain's capabilities. Use it as your authoritative reference for creating sophisticated AI workflows and troubleshooting issues across any project.