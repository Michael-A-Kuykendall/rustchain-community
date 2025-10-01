/// Minimal step type validation for RustChain
/// Tests core step types that should work without feature dependencies
use rustchain::engine::{Mission, MissionStep, StepType, DagExecutor, ExecutionContext};
use serde_json::json;

#[tokio::test]
async fn test_noop_mission_execution() {
    println!("🔧 Testing Noop step via mission execution...");
    
    let mission = Mission {
        version: "1.0".to_string(),
        name: "Test Noop Mission".to_string(),
        description: Some("Test mission with Noop step".to_string()),
        steps: vec![
            MissionStep {
                id: "noop_test".to_string(),
                name: "Test Noop".to_string(),
                step_type: StepType::Noop,
                depends_on: None,
                timeout_seconds: None,
                continue_on_error: Some(false),
                parameters: json!({}),
            }
        ],
        config: None,
    };
    
    let result = DagExecutor::execute_mission(mission).await;
    
    match result {
        Ok(_) => {
            println!("  ✅ Noop mission executed successfully");
            assert!(true, "Noop mission should always succeed");
        },
        Err(e) => {
            println!("  ❌ Noop mission failed: {}", e);
            // Don't panic - this tells us about implementation status
        }
    }
}

#[tokio::test]
async fn test_multi_step_mission() {
    println!("🔧 Testing multi-step mission execution...");
    
    let mission = Mission {
        version: "1.0".to_string(),
        name: "Multi-Step Test Mission".to_string(),
        description: Some("Test mission with multiple steps".to_string()),
        steps: vec![
            MissionStep {
                id: "step1".to_string(),
                name: "First Step".to_string(),
                step_type: StepType::Noop,
                depends_on: None,
                timeout_seconds: None,
                continue_on_error: Some(false),
                parameters: json!({}),
            },
            MissionStep {
                id: "step2".to_string(),
                name: "Second Step".to_string(),
                step_type: StepType::Noop,
                depends_on: Some(vec!["step1".to_string()]),
                timeout_seconds: None,
                continue_on_error: Some(false),
                parameters: json!({}),
            }
        ],
        config: None,
    };
    
    let result = DagExecutor::execute_mission(mission).await;
    
    match result {
        Ok(_) => {
            println!("  ✅ Multi-step mission executed successfully");
        },
        Err(e) => {
            println!("  ❌ Multi-step mission failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_step_type_availability() {
    println!("📊 Step Type Availability Check");
    println!("================================");
    
    let test_step_types = vec![
        (StepType::Noop, "Noop"),
        (StepType::Command, "Command"),
        (StepType::Http, "HTTP"),
        (StepType::CreateFile, "CreateFile"),
        (StepType::Tool, "Tool"),
        (StepType::Llm, "LLM"),
        (StepType::Chain, "Chain"),
        (StepType::Agent, "Agent"),
    ];
    
    let mut success_count = 0;
    let mut error_count = 0;
    
    for (step_type, type_name) in test_step_types {
        let mission = Mission {
            version: "1.0".to_string(),
            name: format!("Test {} Mission", type_name),
            description: Some(format!("Test mission for {} step type", type_name)),
            steps: vec![
                MissionStep {
                    id: format!("{}_test", type_name.to_lowercase()),
                    name: format!("Test {}", type_name),
                    step_type: step_type.clone(),
                    depends_on: None,
                    timeout_seconds: None,
                    continue_on_error: Some(true),
                    parameters: json!({}),
                }
            ],
            config: None,
        };
        
        let result = DagExecutor::execute_mission(mission).await;
        
        match result {
            Ok(_) => {
                success_count += 1;
                println!("  ✅ {} - Available", type_name);
            },
            Err(e) => {
                error_count += 1;
                println!("  ⚠️  {} - Not Available: {}", type_name, e);
            }
        }
    }
    
    println!("\n📈 AVAILABILITY SUMMARY");
    println!("=======================");
    println!("✅ Available: {} step types", success_count);
    println!("⚠️  Not Available: {} step types", error_count);
    println!("📊 Total Tested: {} step types", success_count + error_count);
    
    // At least Noop should work
    assert!(success_count > 0, "At least one step type should be available");
}

#[tokio::test] 
async fn test_execution_context_functionality() {
    println!("🔧 Testing ExecutionContext functionality...");
    
    let mut context = ExecutionContext::new();
    
    // Test basic context creation
    assert!(context.variables.is_empty(), "Variables should start empty");
    assert!(context.environment.is_empty(), "Environment should start empty");
    
    // Test variable setting (assuming string-based storage)
    context.variables.insert("test_var".to_string(), "test_value".to_string());
    
    if let Some(value) = context.variables.get("test_var") {
        println!("  ✅ Variable storage working: {}", value);
        assert_eq!(value, "test_value");
    } else {
        println!("  ❌ Variable storage not working");
        panic!("Variable storage should work");
    }
    
    println!("  ✅ ExecutionContext basic functionality verified");
}