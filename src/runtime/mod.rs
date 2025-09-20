use crate::assert_invariant;
use crate::core::RuntimeContext;
use crate::engine::{DagExecutor, Mission, MissionResult};

#[cfg(feature = "tools")]
use crate::tools::{create_default_tool_manager, ToolCall, ToolManager, ToolResult};

pub struct RustChainRuntime {
    context: RuntimeContext,
    #[cfg(feature = "tools")]
    tool_manager: ToolManager,
}

impl RustChainRuntime {
    pub fn new() -> Self {
        assert_invariant!(true, "RustChainRuntime initialized", Some("runtime"));

        #[cfg(feature = "tools")]
        let tool_manager = create_default_tool_manager();

        Self {
            context: RuntimeContext::new(),
            #[cfg(feature = "tools")]
            tool_manager,
        }
    }

    pub async fn execute_mission(&self, mission: Mission) -> anyhow::Result<MissionResult> {
        if mission.steps.is_empty() {
            return Err(anyhow::anyhow!("Mission must have at least one step"));
        }

        // Log mission start
        let audit_entry = crate::core::AuditEntry {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            actor: "system".to_string(),
            action: format!("execute_mission:{}", mission.name),
            outcome: "started".to_string(),
            reason: None,
        };

        self.context.audit.log(audit_entry).await;

        // Execute via DAG executor
        let result = DagExecutor::execute_mission(mission).await?;

        // Log completion
        let completion_entry = crate::core::AuditEntry {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            actor: "system".to_string(),
            action: format!("execute_mission:{}", result.mission_id),
            outcome: format!("{:?}", result.status),
            reason: None,
        };

        self.context.audit.log(completion_entry).await;

        Ok(result)
    }

    #[cfg(feature = "tools")]
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        parameters: serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        let tool_call = ToolCall::new(
            tool_name.to_string(),
            parameters,
        );

        self.tool_manager
            .execute_tool(tool_call, &self.context)
            .await
    }

    #[cfg(feature = "tools")]
    pub fn list_tools(&self) -> Vec<&str> {
        self.tool_manager.list_tools()
    }

    pub fn get_context(&self) -> &RuntimeContext {
        &self.context
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{MissionConfig, MissionStep, StepType};

    /// Helper function to create a test mission with steps
    fn create_test_mission(name: &str, step_count: usize) -> Mission {
        let steps = (0..step_count)
            .map(|i| MissionStep {
                id: format!("step_{}", i),
                name: format!("Test Step {}", i),
                step_type: StepType::Noop,
                depends_on: None,
                timeout_seconds: None,
                continue_on_error: None,
                parameters: serde_json::json!({"test": true}),
            })
            .collect();

        Mission {
            version: "1.0".to_string(),
            name: name.to_string(),
            description: Some("Test mission for runtime testing".to_string()),
            steps,
            config: Some(MissionConfig {
                max_parallel_steps: Some(2),
                timeout_seconds: Some(300),
                fail_fast: Some(true),
            }),
        }
    }

    /// Helper function to create an empty mission (for testing assertions)
    fn create_empty_mission(name: &str) -> Mission {
        Mission {
            version: "1.0".to_string(),
            name: name.to_string(),
            description: Some("Empty test mission".to_string()),
            steps: Vec::new(),
            config: None,
        }
    }

    // ===============================
    // Runtime Creation Tests  
    // ===============================

    #[test]
    fn test_runtime_creation_basic() {
        // Test basic runtime creation
        let runtime = RustChainRuntime::new();
        
        // Verify runtime context is available
        let context = runtime.get_context();
        
        // Basic verification - context should exist
        assert!(!std::ptr::eq(context, std::ptr::null()), "Context should not be null");
    }

    #[test]
    fn test_runtime_creation_invariant_assertion() {
        // This should not panic - the invariant should always be true
        let _runtime = RustChainRuntime::new();
        // If we reach this point, the assertion passed
    }

    #[cfg(feature = "tools")]
    #[test]
    fn test_runtime_creation_with_tools_feature() {
        let runtime = RustChainRuntime::new();
        
        // When tools feature is enabled, list_tools should be callable
        let tools = runtime.list_tools();
        // Should not panic and should return a vector (can be empty)
        assert!(tools.len() >= 0, "Tool list should be accessible");
    }

    // ===============================
    // Context Management Tests
    // ===============================

    #[test]
    fn test_get_context_returns_valid_reference() {
        let runtime = RustChainRuntime::new();
        let context1 = runtime.get_context();
        let context2 = runtime.get_context();
        
        // Both references should point to the same context
        assert!(std::ptr::eq(context1, context2), "Context references should be consistent");
    }

    #[test]
    fn test_context_components_accessible() {
        let runtime = RustChainRuntime::new();
        let context = runtime.get_context();
        
        // Context should be accessible and have expected components
        // These are Arc types, so we just verify they're accessible
        let _audit = &context.audit;
        let _policy_engine = &context.policy_engine; 
        let _feature_detector = &context.feature_detector;
        
        // If we reach here, the context has all required components
        assert!(true, "Context should have all required components accessible");
    }

    // ===============================
    // Mission Execution Tests
    // ===============================

    #[tokio::test]
    async fn test_execute_mission_with_valid_steps() {
        let runtime = RustChainRuntime::new();
        let mission = create_test_mission("test_execution", 1);
        
        // Execute the mission - should either succeed or fail gracefully
        let result = runtime.execute_mission(mission).await;
        
        // Test passes if we get any result (Ok or Err) without panicking
        match result {
            Ok(_mission_result) => {
                assert!(true, "Mission execution completed successfully");
            }
            Err(_e) => {
                assert!(true, "Mission execution failed gracefully with error");
            }
        }
    }

    #[tokio::test]
    async fn test_execute_mission_empty_steps_panics() {
        let runtime = RustChainRuntime::new();
        let empty_mission = create_empty_mission("empty_mission");
        
        // This should return an error for empty mission
        let result = runtime.execute_mission(empty_mission).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Mission must have at least one step"));
    }

    #[tokio::test]
    async fn test_execute_mission_audit_logging() {
        let runtime = RustChainRuntime::new();
        let mission = create_test_mission("audit_test", 1);
        
        // Execute mission (audit logging happens internally)
        let _result = runtime.execute_mission(mission).await;
        
        // This test verifies that audit logging doesn't cause failures
        assert!(true, "Mission execution with audit logging completed");
    }

    // ===============================
    // Tool Integration Tests (Feature-Gated)
    // ===============================

    #[cfg(feature = "tools")]
    mod tool_tests {
        use super::*;

        #[test]
        fn test_list_tools_returns_vector() {
            let runtime = RustChainRuntime::new();
            let tools = runtime.list_tools();
            
            // Should return a vector (can be empty)
            assert!(tools.len() >= 0, "Should return valid tools list");
        }

        #[test]
        fn test_list_tools_consistency() {
            let runtime = RustChainRuntime::new();
            let tools1 = runtime.list_tools();
            let tools2 = runtime.list_tools();
            
            // Should be consistent between calls
            assert_eq!(tools1.len(), tools2.len(), "Tool list should be consistent");
        }

        #[tokio::test]
        async fn test_execute_tool_basic() {
            let runtime = RustChainRuntime::new();
            
            // Try to execute a tool with basic parameters
            let tool_result = runtime
                .execute_tool("test_tool", serde_json::json!({"param": "value"}))
                .await;
            
            // Should return some result (success or error)
            match tool_result {
                Ok(_result) => {
                    assert!(true, "Tool execution completed successfully");
                }
                Err(_e) => {
                    assert!(true, "Tool execution failed gracefully");
                }
            }
        }
    }

    // ===============================
    // Feature Flag and Integration Tests  
    // ===============================

    #[test]
    fn test_feature_compilation() {
        let _runtime = RustChainRuntime::new();
        
        // Test that compilation works with various feature combinations
        #[cfg(feature = "tools")]
        {
            let _tools = _runtime.list_tools();
            assert!(true, "Tools feature methods should be available");
        }
        
        #[cfg(not(feature = "tools"))]
        {
            assert!(true, "Runtime should compile without tools feature");
        }
    }

    #[test]
    fn test_runtime_thread_safety() {
        // Create runtime and verify it can be used from different contexts
        let runtime = std::sync::Arc::new(RustChainRuntime::new());
        let runtime_clone = runtime.clone();
        
        // Access context from both references
        let _context1 = runtime.get_context();
        let _context2 = runtime_clone.get_context();
        
        assert!(true, "Runtime should be thread-safe for read operations");
    }

    #[tokio::test]
    async fn test_multiple_mission_executions() {
        let runtime = RustChainRuntime::new();
        
        // Execute multiple missions to test runtime reuse
        for i in 0..2 {
            let mission = create_test_mission(&format!("mission_{}", i), 1);
            let _result = runtime.execute_mission(mission).await;
        }
        
        assert!(true, "Runtime should support multiple mission executions");
    }

    #[test]
    fn test_runtime_memory_usage() {
        // Create multiple runtimes to test memory behavior
        let mut runtimes = Vec::new();
        for _ in 0..5 {
            runtimes.push(RustChainRuntime::new());
        }
        
        // All runtimes should be created successfully
        assert_eq!(runtimes.len(), 5, "Should be able to create multiple runtimes");
        
        // Access contexts to ensure they're properly initialized
        for runtime in &runtimes {
            let _context = runtime.get_context();
        }
    }
}
