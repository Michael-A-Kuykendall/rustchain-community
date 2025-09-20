// Simple test program to validate step type functionality
use rustchain::engine::{Mission, MissionStep, StepType, DagExecutor};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🔧 RustChain Step Type Validation");
    println!("===================================");
    
    // Test 1: Basic Noop Step
    println!("\n📋 Test 1: Basic Noop Mission");
    let noop_mission = Mission {
        version: "1.0".to_string(),
        name: "Noop Test Mission".to_string(),
        description: Some("Test basic Noop step execution".to_string()),
        steps: vec![
            MissionStep {
                id: "noop_1".to_string(),
                name: "Test Noop Step".to_string(),
                step_type: StepType::Noop,
                depends_on: None,
                timeout_seconds: None,
                continue_on_error: Some(false),
                parameters: json!({}),
            }
        ],
        config: None,
    };
    
    match DagExecutor::execute_mission(noop_mission).await {
        Ok(result) => println!("  ✅ Noop mission succeeded: {} steps", result.step_results.len()),
        Err(e) => println!("  ❌ Noop mission failed: {}", e),
    }
    
    // Test 2: Multi-Step Mission with Dependencies
    println!("\n📋 Test 2: Multi-Step Mission with Dependencies");
    let multi_step_mission = Mission {
        version: "1.0".to_string(),
        name: "Multi-Step Test Mission".to_string(),
        description: Some("Test multiple dependent steps".to_string()),
        steps: vec![
            MissionStep {
                id: "step_1".to_string(),
                name: "First Step".to_string(),
                step_type: StepType::Noop,
                depends_on: None,
                timeout_seconds: None,
                continue_on_error: Some(false),
                parameters: json!({}),
            },
            MissionStep {
                id: "step_2".to_string(),
                name: "Second Step".to_string(),
                step_type: StepType::Noop,
                depends_on: Some(vec!["step_1".to_string()]),
                timeout_seconds: None,
                continue_on_error: Some(false),
                parameters: json!({}),
            }
        ],
        config: None,
    };
    
    match DagExecutor::execute_mission(multi_step_mission).await {
        Ok(result) => println!("  ✅ Multi-step mission succeeded: {} steps", result.step_results.len()),
        Err(e) => println!("  ❌ Multi-step mission failed: {}", e),
    }
    
    // Test 3: Step Type Availability Survey
    println!("\n📋 Test 3: Step Type Availability Survey");
    let test_step_types = vec![
        (StepType::Noop, "Noop"),
        (StepType::Command, "Command"),
        (StepType::CreateFile, "CreateFile"),
        (StepType::Http, "HTTP"),
        (StepType::Tool, "Tool"),
        (StepType::Chain, "Chain"),
        (StepType::Agent, "Agent"),
        (StepType::Llm, "LLM"),
        (StepType::SqlQuery, "SQL Query"),
        (StepType::RedisGet, "Redis Get"),
        (StepType::GenerateEmbedding, "Generate Embedding"),
    ];
    
    let mut implemented_count = 0;
    let mut not_implemented_count = 0;
    
    for (step_type, type_name) in test_step_types {
        let test_mission = Mission {
            version: "1.0".to_string(),
            name: format!("Test {} Mission", type_name),
            description: Some(format!("Test {} step type", type_name)),
            steps: vec![
                MissionStep {
                    id: format!("test_{}", type_name.replace(" ", "_").to_lowercase()),
                    name: format!("Test {}", type_name),
                    step_type: step_type.clone(),
                    depends_on: None,
                    timeout_seconds: Some(5), // 5 second timeout for tests
                    continue_on_error: Some(true),
                    parameters: json!({}),
                }
            ],
            config: None,
        };
        
        match DagExecutor::execute_mission(test_mission).await {
            Ok(_) => {
                implemented_count += 1;
                println!("  ✅ {} - IMPLEMENTED", type_name);
            },
            Err(e) => {
                not_implemented_count += 1;
                let error_msg = e.to_string();
                if error_msg.contains("not implemented") || error_msg.contains("unsupported") {
                    println!("  🚧 {} - NOT IMPLEMENTED", type_name);
                } else {
                    println!("  ⚠️  {} - ERROR: {}", type_name, e);
                }
            }
        }
    }
    
    println!("\n📊 STEP TYPE IMPLEMENTATION SUMMARY");
    println!("====================================");
    println!("✅ Implemented: {} step types", implemented_count);
    println!("🚧 Not Implemented: {} step types", not_implemented_count);
    println!("📊 Total Tested: {} step types", implemented_count + not_implemented_count);
    
    let implementation_rate = (implemented_count as f64 / (implemented_count + not_implemented_count) as f64) * 100.0;
    println!("📈 Implementation Rate: {:.1}%", implementation_rate);
    
    if implementation_rate >= 50.0 {
        println!("\n🎉 ASSESSMENT: RustChain has substantial step type implementation");
    } else if implementation_rate >= 25.0 {
        println!("\n⚠️  ASSESSMENT: RustChain has partial step type implementation");
    } else {
        println!("\n❌ ASSESSMENT: RustChain has minimal step type implementation");
    }
    
    Ok(())
}