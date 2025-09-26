use rustchain::core::{RuntimeContext, Tool};
use rustchain::engine::{DagExecutor, Mission, MissionStep, StepType};
use serde_json::json;

// Simple mock tool for testing
struct MockTool {
    name: String,
}

impl MockTool {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Tool for MockTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn invoke(&self, args: serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
        Ok(json!({
            "tool": self.name,
            "input": args,
            "output": "mock_result"
        }))
    }
}

#[tokio::test]
async fn test_runtime_context_creation() {
    let ctx = RuntimeContext::new();
    let config = ctx.config.read().await;
    assert_eq!(config.mission_timeout_seconds, 300);
    assert_eq!(config.max_parallel_steps, 4);
    assert!(config.audit_enabled);
}

#[tokio::test]
async fn test_audit_logging() {
    let ctx = RuntimeContext::new();
    ctx.audit_action("test_agent", "test_action", "success")
        .await;

    // Verify audit entry was logged by checking the chain hash is not genesis
    let hash = ctx.audit.get_chain_hash().await;
    assert_ne!(hash, "genesis");
}

#[tokio::test]
async fn test_simple_mission_execution() {
    let mission = Mission {
        version: "1.0".to_string(),
        name: "test_mission".to_string(),
        description: Some("Test mission".to_string()),
        steps: vec![MissionStep {
            id: "step1".to_string(),
            name: "Test Step".to_string(),
            step_type: StepType::CreateFile,
            parameters: json!({
                "path": "test_file.txt",
                "content": "Hello, World!"
            }),
            depends_on: None,
            timeout_seconds: Some(30),
            continue_on_error: None,
        }],
        config: None,
    };

    let result = DagExecutor::execute_mission(mission).await;
    assert!(result.is_ok(), "Mission execution should succeed");

    let execution_result = result.unwrap();
    assert_eq!(execution_result.step_results.len(), 1);
}

#[tokio::test]
async fn test_tool_registry() {
    let ctx = RuntimeContext::new();
    let mock_tool = Box::new(MockTool::new("echo"));

    {
        let mut registry = ctx.tool_registry.write().await;
        registry.register("echo".to_string(), mock_tool);
    }

    let registry = ctx.tool_registry.read().await;
    let tool = registry.get("echo");
    assert!(tool.is_some());

    let result = tool.unwrap().invoke(json!({"message": "hello"}));
    assert!(result.is_ok());
}
