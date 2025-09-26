# 📚 RustChain API Reference

**Complete API reference for RustChain Community Edition**

## 🎯 **Quick Navigation**

- [Core Runtime](#core-runtime) - Main runtime context and execution
- [Mission System](#mission-system) - Mission loading and execution
- [Agent Framework](#agent-framework) - AI agent reasoning
- [Chain Execution](#chain-execution) - Sequential workflow processing  
- [Tool System](#tool-system) - Tool registration and execution
- [LLM Integration](#llm-integration) - Language model providers
- [Memory Management](#memory-management) - Memory stores and context
- [Policy Engine](#policy-engine) - Governance and compliance
- [Safety System](#safety-system) - Validation and risk assessment
- [Audit System](#audit-system) - Logging and integrity tracking

---

## 🏗️ **Core Runtime**

### **`RuntimeContext`**

The central context for all RustChain operations.

```rust
use rustchain::core::RuntimeContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = RuntimeContext::new().await?;
    // Use context for mission execution, agent creation, etc.
    Ok(())
}
```

#### **Methods**

```rust
impl RuntimeContext {
    // Create new runtime context
    pub async fn new() -> Result<Self, RuntimeError>;
    
    // Execute mission from file
    pub async fn execute_mission(&self, path: &str) -> Result<ExecutionResult, RuntimeError>;
    
    // Execute mission object
    pub async fn execute_mission_object(&self, mission: Mission) -> Result<ExecutionResult, RuntimeError>;
    
    // Create agent with this context
    pub fn create_agent(&self, config: AgentConfig) -> Result<Agent, AgentError>;
    
    // Log audit event
    pub async fn audit_action(&self, action: &str, metadata: &str, details: &str);
    
    // Access policy engine
    pub fn policy_engine(&self) -> &PolicyEngine;
    
    // Access safety validator
    pub fn safety_validator(&self) -> &SafetyValidator;
}
```

#### **Configuration**

```rust
use rustchain::core::RuntimeConfig;

let config = RuntimeConfig {
    max_concurrent_missions: Some(5),
    timeout_seconds: Some(300),
    audit_enabled: true,
    policy_enforcement: true,
    safety_validation: true,
};

let context = RuntimeContext::with_config(config).await?;
```

---

## 🎯 **Mission System**

### **Mission Structure**

```rust
use rustchain::engine::{Mission, MissionStep, StepType};
use serde_json::json;

let mission = Mission {
    name: "Example Mission".to_string(),
    version: "1.0".to_string(),
    description: Some("Example description".to_string()),
    steps: vec![
        MissionStep {
            id: "step1".to_string(),
            name: "First Step".to_string(),
            step_type: StepType::CreateFile,
            parameters: json!({
                "path": "output.txt",
                "content": "Hello RustChain!"
            }),
            depends_on: None,
            timeout_seconds: Some(30),
        }
    ],
    config: Some(MissionConfig {
        max_parallel_steps: Some(1),
        timeout_seconds: Some(120),
        fail_fast: Some(true),
    }),
};
```

### **Step Types**

```rust
pub enum StepType {
    // File Operations
    CreateFile,     // Create new file
    EditFile,       // Modify existing file  
    DeleteFile,     // Remove file
    
    // System Operations
    Command,        // Execute shell command
    Http,           // HTTP request
    
    // AI Operations
    Llm,           // LLM completion
    Agent,         // Agent reasoning
    Chain,         // Chain execution
    
    // Data Operations
    Tool,          // Tool execution
    RagQuery,      // RAG query
    RagAdd,        // RAG document addition
    
    // Control Flow
    Noop,          // No operation (testing)
}
```

### **Mission Loading**

```rust
use rustchain::engine::MissionLoader;

// From YAML file
let mission = MissionLoader::load_from_file("mission.yaml")?;

// From JSON file  
let mission = MissionLoader::load_from_file("mission.json")?;

// From string content
let yaml_content = r#"
name: "String Mission"
version: "1.0"
steps:
  - id: "test"
    step_type: "noop"
    parameters: {}
"#;
let mission = MissionLoader::load_from_yaml_str(yaml_content)?;
```

### **Execution Results**

```rust
pub struct ExecutionResult {
    pub mission_id: String,
    pub status: ExecutionStatus,
    pub step_results: HashMap<String, StepResult>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: Option<Duration>,
    pub error: Option<String>,
}

pub struct StepResult {
    pub step_id: String,
    pub status: StepStatus,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub duration: Duration,
    pub metadata: HashMap<String, String>,
}
```

---

## 🤖 **Agent Framework**

### **Agent Creation**

```rust
use rustchain::core::{Agent, AgentConfig};

let config = AgentConfig {
    name: "MyAgent".to_string(),
    description: "Custom agent description".to_string(),
    max_iterations: 10,
    temperature: 0.7,
    tools: vec!["file_create".to_string(), "http".to_string()],
    memory_capacity: Some(1000),
};

let agent = context.create_agent(config)?;
```

### **Agent Execution**

```rust
// Execute single task
let result = agent.execute_task("Create a summary of the project").await?;

// Execute with specific objective and context
let objective = "Analyze the data and provide insights";
let context_data = json!({"data": "sample_data.csv"});
let result = agent.execute_with_context(objective, context_data).await?;

// Stream agent reasoning (for real-time monitoring)
let mut stream = agent.execute_stream("Complex multi-step task").await?;
while let Some(step) = stream.next().await {
    println!("Agent step: {:?}", step);
}
```

### **Agent Reasoning Patterns**

```rust
use rustchain::core::reasoning::{ReActAgent, ChainOfThoughtAgent};

// ReAct Pattern (Reasoning + Acting)
let react_agent = ReActAgent::new(context.clone())
    .with_tools(vec!["calculator", "search", "file_ops"])
    .with_max_iterations(15)
    .build()?;

// Chain of Thought Pattern
let cot_agent = ChainOfThoughtAgent::new(context.clone())
    .with_reasoning_depth(5)
    .with_verification(true)
    .build()?;
```

---

## 🔗 **Chain Execution**

### **Sequential Chains**

```rust
use rustchain::core::chain::{Chain, ChainStep, ChainConfig};

let chain = Chain::new("Data Processing Chain")
    .with_config(ChainConfig {
        pass_context: true,
        fail_fast: false,
        parallel_execution: false,
    })
    .add_step(ChainStep::llm("Extract key information from data"))
    .add_step(ChainStep::tool("csv_loader", json!({"file": "data.csv"})))
    .add_step(ChainStep::agent("Analyze and summarize findings"))
    .build()?;

let result = chain.execute(&context).await?;
```

### **Parallel Chains**

```rust
let parallel_chain = Chain::new("Parallel Processing")
    .with_config(ChainConfig {
        parallel_execution: true,
        max_parallel: Some(3),
        ..Default::default()
    })
    .add_parallel_steps(vec![
        ChainStep::tool("process_data_1", json!({"input": "data1.csv"})),
        ChainStep::tool("process_data_2", json!({"input": "data2.csv"})),
        ChainStep::tool("process_data_3", json!({"input": "data3.csv"})),
    ])
    .add_step(ChainStep::agent("Merge and analyze all results"))
    .build()?;
```

### **Chain Context Management**

```rust
use rustchain::core::chain::ChainContext;

let context = ChainContext::new()
    .with_variable("user_id", "12345")
    .with_variable("project_name", "MyProject")
    .with_memory(chain_memory);

// Variables are accessible in chain steps via templating
// Example: "Process data for user {{user_id}} in project {{project_name}}"
```

---

## 🛠️ **Tool System**

### **Built-in Tools**

```rust
use rustchain::tools::{ToolManager, ToolResult};

let tool_manager = create_default_tool_manager();

// Available built-in tools
let tools = tool_manager.list_tools();
// Returns: ["create_file", "http", "command", "csv_loader", "json_yaml_loader", "html_loader"]

// Execute tool
let result = tool_manager.execute_tool("create_file", json!({
    "path": "output.txt",
    "content": "Tool execution result"
})).await?;
```

### **Custom Tool Creation**

```rust
use rustchain::tools::{Tool, ToolExecutor};
use async_trait::async_trait;

pub struct CustomTool;

#[async_trait]
impl ToolExecutor for CustomTool {
    fn name(&self) -> &str {
        "custom_tool"
    }
    
    fn description(&self) -> &str {
        "Custom tool for specific operations"
    }
    
    async fn execute(&self, params: serde_json::Value) -> Result<ToolResult, ToolError> {
        // Implement your tool logic here
        let result = json!({"status": "success", "data": "processed"});
        Ok(ToolResult::Success(result))
    }
}

// Register custom tool
let mut tool_manager = create_default_tool_manager();
tool_manager.register_tool(Box::new(CustomTool))?;
```

### **Tool Registry (CLI Integration)**

```rust
use rustchain::core::tools::ToolRegistry;

let mut registry = ToolRegistry::new();
registry.register_defaults()?;

// Add custom tool to registry
registry.register("my_tool", Arc::new(MyCustomTool::new()))?;

// Get tool for execution
let tool = registry.get_tool("csv_loader")?;
let result = tool.execute(params).await?;
```

---

## 🧠 **LLM Integration**

### **Provider Configuration**

```rust
use rustchain::llm::{LlmProvider, OpenAIProvider, AnthropicProvider, OllamaProvider};

// OpenAI
let openai = OpenAIProvider::new("your-api-key")
    .with_model("gpt-4")
    .with_temperature(0.7)
    .with_max_tokens(1000);

// Anthropic Claude
let anthropic = AnthropicProvider::new("your-api-key")
    .with_model("claude-3-haiku")
    .with_temperature(0.5);

// Ollama (local)
let ollama = OllamaProvider::new("http://localhost:11434")
    .with_model("llama3.2:1b");
```

### **LLM Execution**

```rust
use rustchain::llm::{LlmRequest, LlmResponse};

let request = LlmRequest {
    prompt: "Analyze this data and provide insights".to_string(),
    system_prompt: Some("You are a data analyst".to_string()),
    temperature: Some(0.7),
    max_tokens: Some(500),
    context: Some(json!({"data": "context_data"})),
};

let response = openai.complete(request).await?;
println!("Response: {}", response.content);
```

### **Streaming Responses**

```rust
let mut stream = openai.complete_stream(request).await?;
while let Some(chunk) = stream.next().await {
    match chunk? {
        LlmChunk::Content(text) => print!("{}", text),
        LlmChunk::Done => break,
        LlmChunk::Error(err) => eprintln!("Error: {}", err),
    }
}
```

---

## 💾 **Memory Management**

### **Memory Store Types**

```rust
use rustchain::core::memory::{MemoryStore, InMemoryStore, VectorStore};

// In-memory store (fast, temporary)
let memory = InMemoryStore::new(3600); // 1 hour TTL

// Vector store (semantic search)
let vector_memory = VectorStore::new("http://localhost:6333")?; // Qdrant

// Conversation memory (chat history)
let conversation = ConversationMemory::new()
    .with_max_messages(100)
    .with_summarization(true);
```

### **Memory Operations**

```rust
// Store and retrieve data
memory.store("key", json!({"data": "value"})).await?;
let data = memory.get("key").await?;

// Semantic search in vector store
let query = "Find information about AI agents";
let results = vector_memory.search(query, 5).await?;

// Conversation management
conversation.add_message("user", "Hello, how can you help me?").await?;
conversation.add_message("assistant", "I can help with RustChain operations").await?;
let history = conversation.get_recent(10).await?;
```

### **Memory Context**

```rust
use rustchain::core::memory::MemoryContext;

let context = MemoryContext::new()
    .with_store("short_term", memory)
    .with_store("long_term", vector_memory)
    .with_conversation(conversation);

// Use in agents and chains
let agent = Agent::new("MemoryAgent")
    .with_memory_context(context)
    .build()?;
```

---

## ⚖️ **Policy Engine**

### **Policy Configuration**

```rust
use rustchain::policy::{PolicyEngine, PolicyRule, PolicyAction, PolicyCondition};

let mut policy_engine = PolicyEngine::new();

// Create custom policy rule
let rule = PolicyRule {
    name: "restrict_file_access".to_string(),
    description: "Restrict file operations to safe directories".to_string(),
    priority: 100,
    conditions: vec![
        PolicyCondition::ActionEquals("tool:execute".to_string()),
        PolicyCondition::ParameterContains("path".to_string(), "/tmp/".to_string()),
    ],
    action: PolicyAction::Allow,
};

policy_engine.add_rule(rule)?;
```

### **Policy Evaluation**

```rust
use std::collections::HashMap;

let context = HashMap::from([
    ("action".to_string(), "tool:execute".to_string()),
    ("tool".to_string(), "create_file".to_string()),
    ("path".to_string(), "/tmp/safe_file.txt".to_string()),
]);

let decision = policy_engine.evaluate("tool:execute", &context)?;
match decision {
    PolicyDecision::Allow => println!("Operation allowed"),
    PolicyDecision::Deny(reason) => println!("Operation denied: {}", reason),
    PolicyDecision::RequireApproval => println!("Manual approval required"),
}
```

### **Built-in Policies**

```rust
use rustchain::policy::create_default_policies;

let default_policies = create_default_policies();
for policy in default_policies {
    policy_engine.add_rule(policy)?;
}

// Default policies include:
// - safe_file_ops: Restrict file operations to safe paths
// - deny_dangerous_commands: Block potentially harmful commands
// - allow_test_tools: Permit testing-related tools
// - allow_document_loaders: Enable document processing tools
// - enterprise_compliance: Additional security rules
```

---

## 🛡️ **Safety System**

### **Safety Validation**

```rust
use rustchain::safety::{SafetyValidator, ValidationMode, ValidationResult};

let validator = SafetyValidator::new();

// Validate mission safety
let result = validator.validate_mission(&mission, ValidationMode::Strict)?;

match result {
    ValidationResult { is_safe: true, risk_score, issues } => {
        println!("Mission is safe (risk score: {})", risk_score);
    },
    ValidationResult { is_safe: false, risk_score, issues } => {
        println!("Mission has safety concerns (risk score: {})", risk_score);
        for issue in issues {
            println!("- {}", issue);
        }
    }
}
```

### **Safety Configuration**

```rust
use rustchain::safety::SafetyConfig;

let config = SafetyConfig {
    max_risk_score: 25,
    validation_mode: ValidationMode::Standard,
    check_file_operations: true,
    check_network_access: true,
    check_command_execution: true,
    custom_validators: vec![],
};

let validator = SafetyValidator::with_config(config);
```

### **Custom Safety Validators**

```rust
use rustchain::safety::{SafetyCheck, SafetyIssue};

pub struct CustomSafetyCheck;

impl SafetyCheck for CustomSafetyCheck {
    fn name(&self) -> &str {
        "custom_safety_check"
    }
    
    fn validate(&self, mission: &Mission) -> Result<Vec<SafetyIssue>, SafetyError> {
        let mut issues = Vec::new();
        
        // Implement custom safety logic
        for step in &mission.steps {
            if step.step_type == StepType::Command {
                if let Some(command) = step.parameters.get("command") {
                    if command.as_str() == Some("rm") {
                        issues.push(SafetyIssue {
                            severity: SafetySeverity::High,
                            message: "Dangerous rm command detected".to_string(),
                            step_id: Some(step.id.clone()),
                        });
                    }
                }
            }
        }
        
        Ok(issues)
    }
}
```

---

## 📋 **Audit System**

### **Audit Logging**

```rust
use rustchain::core::audit::{AuditSink, AuditEvent};

// Log audit events
context.audit.log_event(
    "mission_executed", 
    json!({"mission_id": "123", "status": "success"}),
    Some("user:admin")
).await?;

// Log with specific event type
context.audit.log_mission_start(&mission).await?;
context.audit.log_mission_complete(&result).await?;
context.audit.log_security_event("policy_violation", "details").await?;
```

### **Audit Trail Retrieval**

```rust
use rustchain::core::audit::AuditQuery;

// Get recent audit events
let events = context.audit.get_recent_events(100).await?;

// Query specific events
let query = AuditQuery::new()
    .with_event_type("mission_executed")
    .with_time_range(start_time, end_time)
    .with_user("admin");
    
let filtered_events = context.audit.query_events(query).await?;

// Get audit chain integrity
let chain_hash = context.audit.get_chain_hash().await;
let is_valid = context.audit.verify_chain_integrity().await?;
```

### **Compliance Reporting**

```rust
use rustchain::core::audit::ComplianceReport;

// Generate compliance report
let report = context.audit.generate_compliance_report(
    "SOC2", 
    start_date, 
    end_date
).await?;

println!("Report: {}", report.summary);
for finding in report.findings {
    println!("- {}: {}", finding.severity, finding.description);
}
```

---

## 🔧 **Error Handling**

### **Error Types**

```rust
use rustchain::core::error::{
    RustChainError, RuntimeError, MissionError, AgentError, 
    ToolError, LlmError, PolicyError, SafetyError
};

// All RustChain errors implement std::error::Error
fn handle_errors() -> Result<(), Box<dyn std::error::Error>> {
    match context.execute_mission("mission.yaml").await {
        Ok(result) => println!("Success: {:?}", result),
        Err(RuntimeError::MissionNotFound(path)) => {
            eprintln!("Mission file not found: {}", path);
        },
        Err(RuntimeError::ValidationError(msg)) => {
            eprintln!("Mission validation failed: {}", msg);
        },
        Err(err) => {
            eprintln!("Runtime error: {}", err);
        }
    }
    Ok(())
}
```

### **Result Handling Patterns**

```rust
// Option handling with RustChain
let tool = registry.get_tool("my_tool")
    .ok_or_else(|| ToolError::NotFound("my_tool".to_string()))?;

// Chain error handling
let result = mission_loader.load_from_file(path)
    .map_err(|e| RuntimeError::LoadError(e.to_string()))?;

// Async error handling
let response = llm_provider.complete(request)
    .await
    .map_err(|e| LlmError::RequestFailed(e.to_string()))?;
```

---

## 📊 **Configuration**

### **Runtime Configuration**

```rust
use rustchain::core::config::{RustChainConfig, LlmConfig, PolicyConfig};

let config = RustChainConfig {
    // Runtime settings
    max_concurrent_missions: Some(10),
    default_timeout: Some(300),
    
    // LLM settings
    llm: LlmConfig {
        default_provider: "openai".to_string(),
        openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
        anthropic_api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
        temperature: Some(0.7),
        max_tokens: Some(1000),
    },
    
    // Policy settings
    policy: PolicyConfig {
        enforcement_mode: "strict".to_string(),
        custom_rules_path: Some("policies/".to_string()),
    },
    
    // Safety settings
    safety: SafetyConfig {
        validation_mode: ValidationMode::Standard,
        max_risk_score: 50,
    },
    
    // Audit settings
    audit_enabled: true,
    audit_retention_days: Some(90),
};

let context = RuntimeContext::with_config(config).await?;
```

### **Environment Configuration**

```bash
# Environment variables
export RUSTCHAIN_LOG_LEVEL=debug
export RUSTCHAIN_MAX_CONCURRENT=5
export RUSTCHAIN_POLICY_MODE=strict
export OPENAI_API_KEY=your_key_here
export ANTHROPIC_API_KEY=your_key_here
```

---

## 🎯 **Usage Examples**

### **Complete Application Example**

```rust
use rustchain::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize runtime
    let context = RuntimeContext::new().await?;
    
    // Create and execute mission
    let mission = Mission {
        name: "Data Analysis Pipeline".to_string(),
        version: "1.0".to_string(),
        description: Some("Analyze CSV data and generate report".to_string()),
        steps: vec![
            // Load data
            MissionStep {
                id: "load_data".to_string(),
                name: "Load CSV Data".to_string(),
                step_type: StepType::Tool,
                parameters: json!({
                    "tool": "csv_loader",
                    "parameters": {
                        "file_path": "data.csv",
                        "has_headers": true
                    }
                }),
                depends_on: None,
                timeout_seconds: Some(30),
            },
            
            // Analyze with AI
            MissionStep {
                id: "analyze_data".to_string(),
                name: "AI Data Analysis".to_string(),
                step_type: StepType::Agent,
                parameters: json!({
                    "name": "DataAnalyst",
                    "objective": "Analyze the CSV data and identify key trends",
                    "tools": ["calculator", "create_file"],
                    "max_iterations": 5
                }),
                depends_on: Some(vec!["load_data".to_string()]),
                timeout_seconds: Some(120),
            },
            
            // Generate report
            MissionStep {
                id: "generate_report".to_string(),
                name: "Generate Report".to_string(),
                step_type: StepType::CreateFile,
                parameters: json!({
                    "path": "analysis_report.md",
                    "content": "# Data Analysis Report\n\n{{analyze_data_result}}"
                }),
                depends_on: Some(vec!["analyze_data".to_string()]),
                timeout_seconds: Some(30),
            },
        ],
        config: Some(MissionConfig {
            max_parallel_steps: Some(2),
            timeout_seconds: Some(300),
            fail_fast: Some(true),
        }),
    };
    
    // Execute mission
    let result = context.execute_mission_object(mission).await?;
    
    // Handle results
    match result.status {
        ExecutionStatus::Completed => {
            println!("✅ Mission completed successfully!");
            println!("Duration: {:?}", result.duration);
        },
        ExecutionStatus::Failed => {
            println!("❌ Mission failed: {}", result.error.unwrap_or_default());
        },
        _ => {
            println!("Mission status: {:?}", result.status);
        }
    }
    
    Ok(())
}
```

---

## 📖 **Further Reading**

- **[Examples](../examples/)** - Complete working examples
- **[Contributing Guide](../CONTRIBUTING.md)** - How to extend RustChain
- **[Security Policy](../SECURITY.md)** - Security considerations
- **[Deployment Guide](DEPLOYMENT.md)** - Production deployment
- **[Monitoring Guide](MONITORING.md)** - Observability and health tracking

---

*API Reference updated for RustChain Community Edition v1.0*