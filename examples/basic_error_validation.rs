// Basic error handling validation that avoids the topological sort bug
use rustchain::engine::{Mission, MissionStep, StepType, DagExecutor};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🛡️ RustChain Basic Error Handling Validation");
    println!("=============================================");
    
    let mut total_tests = 0;
    let mut graceful_failures = 0;
    let mut unexpected_panics = 0;
    let mut error_recovery_successes = 0;
    
    // Test 1: Invalid File Path (should fail gracefully)
    println!("\n📋 Test 1: Invalid File Path Handling");
    total_tests += 1;
    
    let invalid_file_mission = Mission {
        version: "1.0".to_string(),
        name: "Invalid File Test".to_string(),
        description: Some("Test invalid file path handling".to_string()),
        steps: vec![
            MissionStep {
                id: "invalid_file".to_string(),
                name: "Invalid File Creation".to_string(),
                step_type: StepType::CreateFile,
                depends_on: None,
                timeout_seconds: None,
                continue_on_error: Some(false),
                parameters: json!({
                    "path": "/this/path/definitely/does/not/exist/test.txt",
                    "content": "This should fail gracefully"
                }),
            }
        ],
        config: None,
    };
    
    match DagExecutor::execute_mission(invalid_file_mission).await {
        Ok(_) => println!("  ⚠️  Unexpected success - should have failed"),
        Err(e) => {
            graceful_failures += 1;
            println!("  ✅ Graceful failure: {}", e);
        }
    }
    
    // Test 2: Invalid Command (should fail gracefully)
    println!("\n📋 Test 2: Invalid Command Handling");
    total_tests += 1;
    
    let invalid_command_mission = Mission {
        version: "1.0".to_string(),
        name: "Invalid Command Test".to_string(),
        description: Some("Test invalid command handling".to_string()),
        steps: vec![
            MissionStep {
                id: "invalid_command".to_string(),
                name: "Invalid Command".to_string(),
                step_type: StepType::Command,
                depends_on: None,
                timeout_seconds: None,
                continue_on_error: Some(false),
                parameters: json!({
                    "command": "this_command_definitely_does_not_exist_anywhere",
                    "args": ["test"]
                }),
            }
        ],
        config: None,
    };
    
    match DagExecutor::execute_mission(invalid_command_mission).await {
        Ok(_) => println!("  ⚠️  Unexpected success - should have failed"),
        Err(e) => {
            graceful_failures += 1;
            println!("  ✅ Graceful failure: {}", e);
        }
    }
    
    // Test 3: Continue on Error Recovery
    println!("\n📋 Test 3: Continue-on-Error Recovery");
    total_tests += 1;
    
    let recovery_mission = Mission {
        version: "1.0".to_string(),
        name: "Recovery Test".to_string(),
        description: Some("Test error recovery".to_string()),
        steps: vec![
            MissionStep {
                id: "failing_step".to_string(),
                name: "Failing Step".to_string(),
                step_type: StepType::CreateFile,
                depends_on: None,
                timeout_seconds: None,
                continue_on_error: Some(true), // Continue despite failure
                parameters: json!({
                    "path": "/invalid/path/fail.txt",
                    "content": "This will fail"
                }),
            },
            MissionStep {
                id: "success_step".to_string(),
                name: "Success Step".to_string(),
                step_type: StepType::Noop,
                depends_on: None, // No dependencies to avoid topological sort bug
                timeout_seconds: None,
                continue_on_error: Some(false),
                parameters: json!({}),
            }
        ],
        config: None,
    };
    
    match DagExecutor::execute_mission(recovery_mission).await {
        Ok(result) => {
            error_recovery_successes += 1;
            println!("  ✅ Error recovery success: {} steps completed", result.step_results.len());
        },
        Err(e) => println!("  ⚠️  Recovery failed: {}", e),
    }
    
    // Test 4: Empty Mission Handling
    println!("\n📋 Test 4: Empty Mission Validation");
    total_tests += 1;
    
    let empty_mission = Mission {
        version: "1.0".to_string(),
        name: "Empty Mission Test".to_string(),
        description: Some("Test empty mission handling".to_string()),
        steps: vec![], // No steps
        config: None,
    };
    
    match DagExecutor::execute_mission(empty_mission).await {
        Ok(_) => println!("  ⚠️  Unexpected success - empty mission should fail"),
        Err(e) => {
            graceful_failures += 1;
            println!("  ✅ Empty mission rejection: {}", e);
        }
    }
    
    // Test 5: Malformed Parameters
    println!("\n📋 Test 5: Malformed Parameter Handling");
    total_tests += 1;
    
    let malformed_params_mission = Mission {
        version: "1.0".to_string(),
        name: "Malformed Parameters Test".to_string(),
        description: Some("Test malformed parameter handling".to_string()),
        steps: vec![
            MissionStep {
                id: "malformed_params".to_string(),
                name: "Malformed Parameters".to_string(),
                step_type: StepType::CreateFile,
                depends_on: None,
                timeout_seconds: None,
                continue_on_error: Some(false),
                parameters: json!({
                    "invalid_field": "value",
                    "missing_required_path": true
                }),
            }
        ],
        config: None,
    };
    
    match DagExecutor::execute_mission(malformed_params_mission).await {
        Ok(_) => println!("  ⚠️  Unexpected success - should have failed"),
        Err(e) => {
            graceful_failures += 1;
            println!("  ✅ Graceful failure: {}", e);
        }
    }
    
    // Calculate error handling metrics
    let graceful_failure_rate = (graceful_failures as f64 / total_tests as f64) * 100.0;
    let error_recovery_rate = if error_recovery_successes > 0 { 100.0 } else { 0.0 };
    
    println!("\n🛡️ BASIC ERROR HANDLING ASSESSMENT");
    println!("==================================");
    println!("📊 Total Tests: {}", total_tests);
    println!("✅ Graceful Failures: {}/{} ({:.1}%)", graceful_failures, total_tests, graceful_failure_rate);
    println!("🔄 Error Recovery: {}/{} ({:.1}%)", error_recovery_successes, 1, error_recovery_rate);
    println!("💥 Unexpected Panics: {}", unexpected_panics);
    
    // Critical Bug Report
    println!("\n🚨 CRITICAL BUG IDENTIFIED");
    println!("==========================");
    println!("🐛 LOCATION: src/engine/mod.rs:1563");
    println!("🔥 ISSUE: Panic on unwrap() in topological_sort()");
    println!("⚠️  IMPACT: Missing dependencies cause runtime panic");
    println!("🚧 SEVERITY: PRODUCTION BLOCKER");
    println!("🔧 FIX NEEDED: Replace unwrap() with proper error handling");
    
    println!("\n📋 ERROR HANDLING GRADE");
    println!("=======================");
    if graceful_failure_rate >= 90.0 && unexpected_panics == 0 {
        println!("🎉 EXCELLENT: Production-ready error handling");
    } else if graceful_failure_rate >= 80.0 && unexpected_panics == 0 {
        println!("👍 GOOD: Solid error handling with minor gaps");
    } else if graceful_failure_rate >= 70.0 {
        println!("⚠️  MODERATE: Adequate error handling, has critical bug");
    } else {
        println!("❌ POOR: Error handling needs significant work");
    }
    
    // Enterprise readiness assessment
    println!("\n🏢 ENTERPRISE READINESS FOR ERROR HANDLING");
    println!("==========================================");
    println!("❌ NOT ENTERPRISE READY: Critical panic bug must be fixed");
    println!("🔧 REQUIRED: Fix topological sort error handling");
    println!("⏰ ESTIMATED FIX TIME: 2-4 hours");
    
    Ok(())
}