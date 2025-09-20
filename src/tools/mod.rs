use crate::core::RuntimeContext;
use crate::policy::{create_default_policies, EnhancedPolicyEngine, PolicyContext};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool_name: String,
    pub parameters: serde_json::Value,
    pub timeout_ms: Option<u64>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub continue_on_error: Option<bool>,
}

impl ToolCall {
    pub fn new(tool_name: String, parameters: serde_json::Value) -> Self {
        Self {
            tool_name,
            parameters,
            timeout_ms: Some(30000), // 30 second default timeout
            metadata: HashMap::new(),
            continue_on_error: Some(false),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: serde_json::Value,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

#[async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn execute(&self, call: ToolCall, context: &RuntimeContext) -> Result<ToolResult>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn schema(&self) -> serde_json::Value;
}

pub struct ToolManager {
    tools: HashMap<String, Box<dyn ToolExecutor>>,
}

impl ToolManager {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register_tool(&mut self, tool: Box<dyn ToolExecutor>) {
        let name = tool.name().to_string();
        info!("Registering tool: {}", name);
        self.tools.insert(name, tool);
    }

    pub async fn execute_tool(
        &self,
        call: ToolCall,
        context: &RuntimeContext,
    ) -> Result<ToolResult> {
        let start = std::time::Instant::now();

        context
            .audit_action(
                "tool_system",
                &format!("executing_tool:{}", call.tool_name),
                "started",
            )
            .await;

        let tool = self
            .tools
            .get(&call.tool_name)
            .ok_or_else(|| anyhow!("Tool not found: {}", call.tool_name))?;

        // Enhanced policy validation
        let policy_context = PolicyContext {
            agent_id: call.metadata
                .get("agent_id")
                .and_then(|v| v.as_str())
                .unwrap_or("system")
                .to_string(),
            ..PolicyContext::default()
        };

        // Create a temporary enhanced policy engine for validation
        let mut policy_engine = EnhancedPolicyEngine::new();
        for rule in create_default_policies() {
            if let Err(e) = policy_engine.add_rule(rule) {
                warn!("Failed to add default policy rule: {}", e);
            }
        }

        let action = format!("tool:{}", call.tool_name);
        let policy_decision = policy_engine.evaluate_action(&action, &policy_context);

        if !policy_decision.allowed {
            context
                .audit_action(
                    "tool_system",
                    &format!("tool_blocked:{}", call.tool_name),
                    &policy_decision.reason,
                )
                .await;
            return Err(anyhow!(
                "Tool execution blocked by policy: {}",
                policy_decision.reason
            ));
        }

        // Basic policy validation (fallback)
        if !context
            .policy_engine
            .validate(&format!("tool:{}", call.tool_name))
        {
            context
                .audit_action(
                    "tool_system",
                    &format!("tool_blocked:{}", call.tool_name),
                    "policy_violation",
                )
                .await;
            return Err(anyhow!(
                "Tool execution blocked by policy: {}",
                call.tool_name
            ));
        }

        let result = tool.execute(call, context).await;
        let _duration = start.elapsed().as_millis() as u64;

        match &result {
            Ok(_) => {
                context
                    .audit_action(
                        "tool_system",
                        &format!("tool_success:{}", tool.name()),
                        "completed",
                    )
                    .await;
            }
            Err(e) => {
                context
                    .audit_action(
                        "tool_system",
                        &format!("tool_error:{}", tool.name()),
                        &e.to_string(),
                    )
                    .await;
            }
        }

        result
    }

    pub fn list_tools(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }

    pub fn get_tool_schema(&self, tool_name: &str) -> Option<serde_json::Value> {
        self.tools.get(tool_name).map(|tool| tool.schema())
    }
}

// Built-in tools
pub struct FileCreateTool;

#[async_trait]
impl ToolExecutor for FileCreateTool {
    async fn execute(&self, call: ToolCall, context: &RuntimeContext) -> Result<ToolResult> {
        let start = std::time::Instant::now();

        let path = call
            .parameters
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'path' parameter"))?;

        let content = call
            .parameters
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Sandbox validation
        let sandbox_result = context.sandbox.execute(&format!("create_file:{}", path));
        if sandbox_result.is_err() {
            return Ok(ToolResult {
                success: false,
                output: serde_json::json!({"error": "Sandbox violation"}),
                error: Some("File creation blocked by sandbox".to_string()),
                execution_time_ms: start.elapsed().as_millis() as u64,
            });
        }

        match tokio::fs::write(path, content).await {
            Ok(_) => Ok(ToolResult {
                success: true,
                output: serde_json::json!({"path": path, "size": content.len()}),
                error: None,
                execution_time_ms: start.elapsed().as_millis() as u64,
            }),
            Err(e) => Ok(ToolResult {
                success: false,
                output: serde_json::json!({"error": e.to_string()}),
                error: Some(e.to_string()),
                execution_time_ms: start.elapsed().as_millis() as u64,
            }),
        }
    }

    fn name(&self) -> &str {
        "create_file"
    }

    fn description(&self) -> &str {
        "Creates a new file with the specified content"
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The file path to create"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            },
            "required": ["path"]
        })
    }
}

pub struct HttpTool;

#[async_trait]
impl ToolExecutor for HttpTool {
    async fn execute(&self, call: ToolCall, _context: &RuntimeContext) -> Result<ToolResult> {
        let start = std::time::Instant::now();

        let url = call
            .parameters
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'url' parameter"))?;

        let method = call
            .parameters
            .get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("GET");

        #[cfg(feature = "llm")]
        {
            let client = reqwest::Client::new();

            let response = match method.to_uppercase().as_str() {
                "GET" => client.get(url).send().await,
                "POST" => {
                    let body = call
                        .parameters
                        .get("body")
                        .unwrap_or(&serde_json::Value::Null);
                    client.post(url).json(body).send().await
                }
                _ => return Err(anyhow!("Unsupported HTTP method: {}", method)),
            };

            match response {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    let text = resp.text().await.unwrap_or_default();

                    Ok(ToolResult {
                        success: status < 400,
                        output: serde_json::json!({
                            "status": status,
                            "body": text
                        }),
                        error: if status >= 400 {
                            Some(format!("HTTP {}", status))
                        } else {
                            None
                        },
                        execution_time_ms: start.elapsed().as_millis() as u64,
                    })
                }
                Err(e) => Ok(ToolResult {
                    success: false,
                    output: serde_json::json!({"error": e.to_string()}),
                    error: Some(e.to_string()),
                    execution_time_ms: start.elapsed().as_millis() as u64,
                }),
            }
        }

        #[cfg(not(feature = "llm"))]
        {
            // Mock HTTP response when reqwest is not available
            Ok(ToolResult {
                success: true,
                output: serde_json::json!({
                    "status": 200,
                    "body": format!("Mock HTTP {} response for: {}", method, url)
                }),
                error: None,
                execution_time_ms: start.elapsed().as_millis() as u64,
            })
        }
    }

    fn name(&self) -> &str {
        "http"
    }

    fn description(&self) -> &str {
        "Makes HTTP requests"
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to request"
                },
                "method": {
                    "type": "string",
                    "enum": ["GET", "POST", "PUT", "DELETE"],
                    "description": "HTTP method to use"
                },
                "body": {
                    "description": "Request body for POST/PUT requests"
                }
            },
            "required": ["url"]
        })
    }
}

pub struct CommandTool;

#[async_trait]
impl ToolExecutor for CommandTool {
    async fn execute(&self, call: ToolCall, context: &RuntimeContext) -> Result<ToolResult> {
        let start = std::time::Instant::now();

        let command = call
            .parameters
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'command' parameter"))?;

        let args = call
            .parameters
            .get("args")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        // Sandbox validation
        let sandbox_check = context.sandbox.execute(&format!("command:{}", command));
        if sandbox_check.is_err() {
            return Ok(ToolResult {
                success: false,
                output: serde_json::json!({"error": "Command blocked by sandbox"}),
                error: Some("Command execution blocked".to_string()),
                execution_time_ms: start.elapsed().as_millis() as u64,
            });
        }

        let output = tokio::process::Command::new(command)
            .args(args)
            .output()
            .await;

        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);

                Ok(ToolResult {
                    success: result.status.success(),
                    output: serde_json::json!({
                        "stdout": stdout,
                        "stderr": stderr,
                        "exit_code": result.status.code()
                    }),
                    error: if !result.status.success() {
                        Some(stderr.to_string())
                    } else {
                        None
                    },
                    execution_time_ms: start.elapsed().as_millis() as u64,
                })
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: serde_json::json!({"error": e.to_string()}),
                error: Some(e.to_string()),
                execution_time_ms: start.elapsed().as_millis() as u64,
            }),
        }
    }

    fn name(&self) -> &str {
        "command"
    }

    fn description(&self) -> &str {
        "Executes system commands"
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The command to execute"
                },
                "args": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Command arguments"
                }
            },
            "required": ["command"]
        })
    }
}

/// Document loader adapter tools that bridge from ToolRegistry (core::tools) to ToolManager (tools::mod)
use crate::core::tools::{Tool, ToolResult as CoreToolResult};
use crate::core::document_loaders::{CsvDocumentLoader, JsonYamlDocumentLoader, HtmlDocumentLoader};

pub struct CsvLoaderTool;

#[async_trait]
impl ToolExecutor for CsvLoaderTool {
    async fn execute(&self, call: ToolCall, _context: &RuntimeContext) -> Result<ToolResult> {
        let start = std::time::Instant::now();
        
        // Create the core CSV loader
        let loader = CsvDocumentLoader::new();
        
        // Convert ToolCall parameters to JSON string input for core tool interface
        let input = serde_json::to_string(&call.parameters)
            .map_err(|e| anyhow!("Failed to serialize parameters: {}", e))?;
        
        // Execute using core tool interface
        match loader.invoke(&input).await {
            Ok(core_result) => {
                let success = match core_result {
                    CoreToolResult::Success(_) => true,
                    CoreToolResult::StructuredJson(_) => true,
                    CoreToolResult::Error(_) => false,
                };
                
                let (output, error) = match core_result {
                    CoreToolResult::Success(msg) => (serde_json::json!({"content": msg}), None),
                    CoreToolResult::StructuredJson(data) => (data, None),
                    CoreToolResult::Error(err) => (serde_json::json!({"error": err}), Some(err)),
                };
                
                Ok(ToolResult {
                    success,
                    output,
                    error,
                    execution_time_ms: start.elapsed().as_millis() as u64,
                })
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: serde_json::json!({"error": e.to_string()}),
                error: Some(e.to_string()),
                execution_time_ms: start.elapsed().as_millis() as u64,
            })
        }
    }

    fn name(&self) -> &str {
        "csv_loader"
    }

    fn description(&self) -> &str {
        "Loads and parses CSV files with customizable delimiters and header options"
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "Path to the CSV file to load"
                },
                "delimiter": {
                    "type": "string", 
                    "description": "Field delimiter character (default: ',')",
                    "default": ","
                },
                "has_headers": {
                    "type": "boolean",
                    "description": "Whether the CSV file has headers (default: true)",
                    "default": true
                }
            },
            "required": ["file_path"]
        })
    }
}

pub struct JsonYamlLoaderTool;

#[async_trait]
impl ToolExecutor for JsonYamlLoaderTool {
    async fn execute(&self, call: ToolCall, _context: &RuntimeContext) -> Result<ToolResult> {
        let start = std::time::Instant::now();
        
        let loader = JsonYamlDocumentLoader::new();
        
        let input = serde_json::to_string(&call.parameters)
            .map_err(|e| anyhow!("Failed to serialize parameters: {}", e))?;
        
        match loader.invoke(&input).await {
            Ok(core_result) => {
                let success = match core_result {
                    CoreToolResult::Success(_) => true,
                    CoreToolResult::StructuredJson(_) => true,
                    CoreToolResult::Error(_) => false,
                };
                
                let (output, error) = match core_result {
                    CoreToolResult::Success(msg) => (serde_json::json!({"content": msg}), None),
                    CoreToolResult::StructuredJson(data) => (data, None),
                    CoreToolResult::Error(err) => (serde_json::json!({"error": err}), Some(err)),
                };
                
                Ok(ToolResult {
                    success,
                    output,
                    error,
                    execution_time_ms: start.elapsed().as_millis() as u64,
                })
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: serde_json::json!({"error": e.to_string()}),
                error: Some(e.to_string()),
                execution_time_ms: start.elapsed().as_millis() as u64,
            })
        }
    }

    fn name(&self) -> &str {
        "json_yaml_loader"
    }

    fn description(&self) -> &str {
        "Loads and parses JSON and YAML files with auto-format detection"
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "Path to the JSON or YAML file to load"
                },
                "format": {
                    "type": "string",
                    "enum": ["json", "yaml"],
                    "description": "Force specific format (auto-detected from extension if not provided)"
                }
            },
            "required": ["file_path"]
        })
    }
}

pub struct HtmlLoaderTool;

#[async_trait]
impl ToolExecutor for HtmlLoaderTool {
    async fn execute(&self, call: ToolCall, _context: &RuntimeContext) -> Result<ToolResult> {
        let start = std::time::Instant::now();
        
        let loader = HtmlDocumentLoader::new();
        
        let input = serde_json::to_string(&call.parameters)
            .map_err(|e| anyhow!("Failed to serialize parameters: {}", e))?;
        
        match loader.invoke(&input).await {
            Ok(core_result) => {
                let success = match core_result {
                    CoreToolResult::Success(_) => true,
                    CoreToolResult::StructuredJson(_) => true,
                    CoreToolResult::Error(_) => false,
                };
                
                let (output, error) = match core_result {
                    CoreToolResult::Success(msg) => (serde_json::json!({"content": msg}), None),
                    CoreToolResult::StructuredJson(data) => (data, None),
                    CoreToolResult::Error(err) => (serde_json::json!({"error": err}), Some(err)),
                };
                
                Ok(ToolResult {
                    success,
                    output,
                    error,
                    execution_time_ms: start.elapsed().as_millis() as u64,
                })
            }
            Err(e) => Ok(ToolResult {
                success: false,
                output: serde_json::json!({"error": e.to_string()}),
                error: Some(e.to_string()),
                execution_time_ms: start.elapsed().as_millis() as u64,
            })
        }
    }

    fn name(&self) -> &str {
        "html_loader"
    }

    fn description(&self) -> &str {
        "Loads and extracts content from HTML files with multiple extraction modes"
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "Path to the HTML file to load"
                },
                "extract_mode": {
                    "type": "string",
                    "enum": ["text", "structure", "links", "metadata", "all"],
                    "description": "Extraction mode: text, structure, links, metadata, or all",
                    "default": "all"
                }
            },
            "required": ["file_path"]
        })
    }
}

/// Initialize default tool registry with document loaders
pub fn create_default_tool_manager() -> ToolManager {
    let mut manager = ToolManager::new();

    // Register built-in tools
    manager.register_tool(Box::new(FileCreateTool));
    manager.register_tool(Box::new(HttpTool));
    manager.register_tool(Box::new(CommandTool));
    
    // Register document loader bridge tools
    manager.register_tool(Box::new(CsvLoaderTool));
    manager.register_tool(Box::new(JsonYamlLoaderTool));
    manager.register_tool(Box::new(HtmlLoaderTool));

    manager
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::RuntimeContext;
    use tempfile::TempDir;

    struct MockTool {
        name: String,
        should_succeed: bool,
        execution_time: u64,
    }

    impl MockTool {
        fn new(name: String, should_succeed: bool) -> Self {
            Self {
                name,
                should_succeed,
                execution_time: 100,
            }
        }

        fn with_execution_time(mut self, time_ms: u64) -> Self {
            self.execution_time = time_ms;
            self
        }
    }

    #[async_trait]
    impl ToolExecutor for MockTool {
        async fn execute(&self, call: ToolCall, _context: &RuntimeContext) -> Result<ToolResult> {
            tokio::time::sleep(tokio::time::Duration::from_millis(self.execution_time)).await;

            if self.should_succeed {
                Ok(ToolResult {
                    success: true,
                    output: serde_json::json!({
                        "tool": self.name,
                        "parameters": call.parameters
                    }),
                    error: None,
                    execution_time_ms: self.execution_time,
                })
            } else {
                Ok(ToolResult {
                    success: false,
                    output: serde_json::json!({"error": "mock tool failure"}),
                    error: Some("Mock tool failure".to_string()),
                    execution_time_ms: self.execution_time,
                })
            }
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            "Mock tool for testing"
        }

        fn schema(&self) -> serde_json::Value {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "test_param": {
                        "type": "string",
                        "description": "Test parameter"
                    }
                }
            })
        }
    }

    fn create_test_runtime_context() -> RuntimeContext {
        // Create a basic runtime context for testing
        RuntimeContext::new()
    }

    #[tokio::test]
    async fn test_tool_manager_creation() {
        let manager = ToolManager::new();
        assert_eq!(manager.list_tools().len(), 0);
    }

    #[tokio::test]
    async fn test_tool_registration() {
        let mut manager = ToolManager::new();
        let tool = MockTool::new("test_tool".to_string(), true);

        manager.register_tool(Box::new(tool));

        let tools = manager.list_tools();
        assert_eq!(tools.len(), 1);
        assert!(tools.contains(&"test_tool"));
    }

    #[tokio::test]
    async fn test_tool_schema_retrieval() {
        let mut manager = ToolManager::new();
        let tool = MockTool::new("test_tool".to_string(), true);

        manager.register_tool(Box::new(tool));

        let schema = manager.get_tool_schema("test_tool");
        assert!(schema.is_some());

        let schema_value = schema.unwrap();
        assert_eq!(schema_value["type"], "object");
        assert!(schema_value["properties"]["test_param"].is_object());
    }

    #[tokio::test]
    async fn test_tool_schema_not_found() {
        let manager = ToolManager::new();
        let schema = manager.get_tool_schema("nonexistent");
        assert!(schema.is_none());
    }

    #[tokio::test]
    async fn test_tool_execution_success() {
        let mut manager = ToolManager::new();
        let tool = MockTool::new("test_tool".to_string(), true);
        manager.register_tool(Box::new(tool));

        let context = create_test_runtime_context();

        let call = ToolCall::new(
            "test_tool".to_string(),
            serde_json::json!({"test_param": "test_value"}),
        );

        let result = manager.execute_tool(call, &context).await;
        assert!(result.is_ok());

        let tool_result = result.unwrap();
        assert!(tool_result.success);
        assert_eq!(tool_result.output["tool"], "test_tool");
        assert_eq!(tool_result.output["parameters"]["test_param"], "test_value");
        assert!(tool_result.error.is_none());
    }

    #[tokio::test]
    async fn test_tool_execution_failure() {
        let mut manager = ToolManager::new();
        let tool = MockTool::new("failing_tool".to_string(), false);
        manager.register_tool(Box::new(tool));

        let context = create_test_runtime_context();

        let call = ToolCall::new(
            "failing_tool".to_string(),
            serde_json::json!({}),
        );

        let result = manager.execute_tool(call, &context).await;
        assert!(result.is_ok());

        let tool_result = result.unwrap();
        assert!(!tool_result.success);
        assert!(tool_result.error.is_some());
        assert_eq!(tool_result.error.unwrap(), "Mock tool failure");
    }

    #[tokio::test]
    async fn test_tool_not_found() {
        let manager = ToolManager::new();
        let context = create_test_runtime_context();

        let call = ToolCall::new(
            "nonexistent_tool".to_string(),
            serde_json::json!({}),
        );

        let result = manager.execute_tool(call, &context).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Tool not found"));
    }

    #[tokio::test]
    async fn test_tool_execution_with_parameters() {
        let mut manager = ToolManager::new();
        let tool = MockTool::new("param_tool".to_string(), true);
        manager.register_tool(Box::new(tool));

        let context = create_test_runtime_context();

        let call = ToolCall::new(
            "param_tool".to_string(),
            serde_json::json!({
                "param1": "value1",
                "param2": 42,
                "param3": true
            }),
        );

        let result = manager.execute_tool(call, &context).await;
        assert!(result.is_ok());

        let tool_result = result.unwrap();
        assert!(tool_result.success);
        assert_eq!(tool_result.output["parameters"]["param1"], "value1");
        assert_eq!(tool_result.output["parameters"]["param2"], 42);
        assert_eq!(tool_result.output["parameters"]["param3"], true);
    }

    #[tokio::test]
    async fn test_file_create_tool_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");

        let tool = FileCreateTool;
        let context = create_test_runtime_context();

        let call = ToolCall::new(
            "create_file".to_string(),
            serde_json::json!({
                "path": file_path.to_str().unwrap(),
                "content": "Hello, World!"
            }),
        );

        let result = tool.execute(call, &context).await;
        assert!(result.is_ok());

        let tool_result = result.unwrap();
        assert!(tool_result.success);
        assert_eq!(tool_result.output["path"], file_path.to_str().unwrap());
        assert_eq!(tool_result.output["size"], 13);
        assert!(tool_result.error.is_none());

        // Verify file was actually created
        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[tokio::test]
    async fn test_file_create_tool_missing_path() {
        let tool = FileCreateTool;
        let context = create_test_runtime_context();

        let call = ToolCall::new(
            "create_file".to_string(),
            serde_json::json!({
                "content": "Hello, World!"
            }),
        );

        let result = tool.execute(call, &context).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing 'path' parameter"));
    }

    #[tokio::test]
    async fn test_file_create_tool_no_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("empty_file.txt");

        let tool = FileCreateTool;
        let context = create_test_runtime_context();

        let call = ToolCall::new(
            "create_file".to_string(),
            serde_json::json!({
                "path": file_path.to_str().unwrap()
                // No content specified - should default to empty
            }),
        );

        let result = tool.execute(call, &context).await;
        assert!(result.is_ok());

        let tool_result = result.unwrap();
        assert!(tool_result.success);
        assert_eq!(tool_result.output["size"], 0);

        // Verify empty file was created
        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "");
    }

    #[tokio::test]
    async fn test_file_create_tool_schema() {
        let tool = FileCreateTool;

        assert_eq!(tool.name(), "create_file");
        assert_eq!(
            tool.description(),
            "Creates a new file with the specified content"
        );

        let schema = tool.schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["path"].is_object());
        assert!(schema["properties"]["content"].is_object());
        assert_eq!(schema["required"], serde_json::json!(["path"]));
    }

    #[tokio::test]
    async fn test_http_tool_schema() {
        let tool = HttpTool;

        assert_eq!(tool.name(), "http");
        assert_eq!(tool.description(), "Makes HTTP requests");

        let schema = tool.schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["url"].is_object());
        assert!(schema["properties"]["method"].is_object());
        assert!(schema["properties"]["body"].is_object());
        assert_eq!(schema["required"], serde_json::json!(["url"]));
    }

    #[tokio::test]
    async fn test_command_tool_success() {
        let tool = CommandTool;
        let context = create_test_runtime_context();

        // Use a simple command that should work on most systems
        let call = ToolCall::new(
            "command".to_string(),
            serde_json::json!({
                "command": if cfg!(windows) { "echo" } else { "echo" },
                "args": ["hello", "world"]
            }),
        );

        let result = tool.execute(call, &context).await;
        assert!(result.is_ok());

        let tool_result = result.unwrap();
        assert!(tool_result.success);
        assert!(tool_result.output["stdout"]
            .as_str()
            .unwrap()
            .contains("hello"));
        assert!(tool_result.error.is_none());
        assert_eq!(tool_result.output["exit_code"], 0);
    }

    #[tokio::test]
    async fn test_command_tool_missing_command() {
        let tool = CommandTool;
        let context = create_test_runtime_context();

        let call = ToolCall::new(
            "command".to_string(),
            serde_json::json!({
                "args": ["test"]
            }),
        );

        let result = tool.execute(call, &context).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing 'command' parameter"));
    }

    #[tokio::test]
    async fn test_command_tool_with_no_args() {
        let tool = CommandTool;
        let context = create_test_runtime_context();

        let call = ToolCall::new(
            "command".to_string(),
            serde_json::json!({
                "command": if cfg!(windows) { "echo" } else { "echo" }
                // No args provided - should work with empty args
            }),
        );

        let result = tool.execute(call, &context).await;
        assert!(result.is_ok());

        let tool_result = result.unwrap();
        assert!(tool_result.success);
        assert_eq!(tool_result.output["exit_code"], 0);
    }

    #[tokio::test]
    async fn test_command_tool_schema() {
        let tool = CommandTool;

        assert_eq!(tool.name(), "command");
        assert_eq!(tool.description(), "Executes system commands");

        let schema = tool.schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["command"].is_object());
        assert!(schema["properties"]["args"].is_object());
        assert_eq!(schema["required"], serde_json::json!(["command"]));
    }

    #[test]
    fn test_create_default_tool_manager() {
        let manager = create_default_tool_manager();
        let tools = manager.list_tools();

        // Should have 6 tools: 3 built-in + 3 document loader bridges
        assert_eq!(tools.len(), 6);
        
        // Built-in tools
        assert!(tools.contains(&"create_file"));
        assert!(tools.contains(&"http"));
        assert!(tools.contains(&"command"));
        
        // Document loader bridge tools
        assert!(tools.contains(&"csv_loader"));
        assert!(tools.contains(&"json_yaml_loader"));
        assert!(tools.contains(&"html_loader"));
    }

    #[test]
    fn test_tool_call_serialization() {
        let call = ToolCall::new(
            "test_tool".to_string(),
            serde_json::json!({"key": "value"}),
        );

        let serialized = serde_json::to_string(&call).unwrap();
        let deserialized: ToolCall = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.tool_name, "test_tool");
        assert_eq!(deserialized.parameters["key"], "value");
        assert_eq!(deserialized.timeout_ms, Some(30000));
    }

    #[test]
    fn test_tool_result_serialization() {
        let result = ToolResult {
            success: true,
            output: serde_json::json!({"result": "success"}),
            error: None,
            execution_time_ms: 150,
        };

        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: ToolResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.success, true);
        assert_eq!(deserialized.output["result"], "success");
        assert!(deserialized.error.is_none());
        assert_eq!(deserialized.execution_time_ms, 150);
    }

    #[tokio::test]
    async fn test_tool_execution_timing() {
        let mut manager = ToolManager::new();
        let tool = MockTool::new("timing_tool".to_string(), true).with_execution_time(200);
        manager.register_tool(Box::new(tool));

        let context = create_test_runtime_context();

        let call = ToolCall::new(
            "timing_tool".to_string(),
            serde_json::json!({}),
        );

        let start = std::time::Instant::now();
        let result = manager.execute_tool(call, &context).await;
        let elapsed = start.elapsed().as_millis() as u64;

        assert!(result.is_ok());
        let tool_result = result.unwrap();
        assert!(tool_result.success);
        assert!(tool_result.execution_time_ms >= 200);
        assert!(elapsed >= 200); // Verify actual execution time
    }

    #[tokio::test]
    async fn test_multiple_tool_registration() {
        let mut manager = ToolManager::new();

        manager.register_tool(Box::new(MockTool::new("tool1".to_string(), true)));
        manager.register_tool(Box::new(MockTool::new("tool2".to_string(), true)));
        manager.register_tool(Box::new(MockTool::new("tool3".to_string(), true)));

        let tools = manager.list_tools();
        assert_eq!(tools.len(), 3);
        assert!(tools.contains(&"tool1"));
        assert!(tools.contains(&"tool2"));
        assert!(tools.contains(&"tool3"));
    }
}
