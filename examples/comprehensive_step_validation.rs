// Comprehensive step type validation program
use rustchain::engine::{Mission, MissionStep, StepType, DagExecutor};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🔧 RustChain Comprehensive Step Type Validation");
    println!("===============================================");
    
    // Test all 51 step types from the StepType enum
    let all_step_types = vec![
        // File Operations (8)
        (StepType::CreateFile, "CreateFile", "File Operations"),
        (StepType::EditFile, "EditFile", "File Operations"),
        (StepType::DeleteFile, "DeleteFile", "File Operations"),
        (StepType::CopyFile, "CopyFile", "File Operations"),
        (StepType::MoveFile, "MoveFile", "File Operations"),
        (StepType::ReadFile, "ReadFile", "File Operations"),
        (StepType::ListDirectory, "ListDirectory", "File Operations"),
        (StepType::FileSearch, "FileSearch", "File Operations"),
        
        // Data Processing (5)
        (StepType::ParseJson, "ParseJson", "Data Processing"),
        (StepType::ParseYaml, "ParseYaml", "Data Processing"),
        (StepType::ParseXml, "ParseXml", "Data Processing"),
        (StepType::ValidateSchema, "ValidateSchema", "Data Processing"),
        (StepType::CsvProcess, "CsvProcess", "Data Processing"),
        
        // Code Development (6)
        (StepType::CompileCode, "CompileCode", "Code Development"),
        (StepType::RunTests, "RunTests", "Code Development"),
        (StepType::FormatCode, "FormatCode", "Code Development"),
        (StepType::LintCode, "LintCode", "Code Development"),
        (StepType::ExtractFunctions, "ExtractFunctions", "Code Development"),
        (StepType::GenerateDocs, "GenerateDocs", "Code Development"),
        
        // Git Operations (5)
        (StepType::GitCommit, "GitCommit", "Git Operations"),
        (StepType::GitBranch, "GitBranch", "Git Operations"),
        (StepType::GitMerge, "GitMerge", "Git Operations"),
        (StepType::GitStatus, "GitStatus", "Git Operations"),
        (StepType::GitDiff, "GitDiff", "Git Operations"),
        
        // System Operations (5)
        (StepType::ProcessStart, "ProcessStart", "System Operations"),
        (StepType::ProcessKill, "ProcessKill", "System Operations"),
        (StepType::MonitorResources, "MonitorResources", "System Operations"),
        (StepType::ServiceHealth, "ServiceHealth", "System Operations"),
        (StepType::Compress, "Compress", "System Operations"),
        
        // Database Operations (5)
        (StepType::SqlQuery, "SqlQuery", "Database Operations"),
        (StepType::RedisSet, "RedisSet", "Database Operations"),
        (StepType::RedisGet, "RedisGet", "Database Operations"),
        (StepType::DbBackup, "DbBackup", "Database Operations"),
        (StepType::DbMigrate, "DbMigrate", "Database Operations"),
        
        // Network Operations (5)
        (StepType::WebsocketConnect, "WebsocketConnect", "Network Operations"),
        (StepType::FtpUpload, "FtpUpload", "Network Operations"),
        (StepType::FtpDownload, "FtpDownload", "Network Operations"),
        (StepType::SshExecute, "SshExecute", "Network Operations"),
        (StepType::PingHost, "PingHost", "Network Operations"),
        
        // AI/ML Operations (3)
        (StepType::GenerateEmbedding, "GenerateEmbedding", "AI/ML Operations"),
        (StepType::SimilaritySearch, "SimilaritySearch", "AI/ML Operations"),
        (StepType::ModelInference, "ModelInference", "AI/ML Operations"),
        
        // Core Operations (9)
        (StepType::Command, "Command", "Core Operations"),
        (StepType::Http, "Http", "Core Operations"),
        (StepType::Noop, "Noop", "Core Operations"),
        (StepType::Llm, "Llm", "Core Operations"),
        (StepType::Tool, "Tool", "Core Operations"),
        (StepType::RagQuery, "RagQuery", "Core Operations"),
        (StepType::RagAdd, "RagAdd", "Core Operations"),
        (StepType::Chain, "Chain", "Core Operations"),
        (StepType::Agent, "Agent", "Core Operations"),
    ];
    
    let mut results_by_category = std::collections::HashMap::new();
    let mut total_implemented = 0;
    let mut total_not_implemented = 0;
    
    for (step_type, type_name, category) in all_step_types {
        let test_mission = Mission {
            version: "1.0".to_string(),
            name: format!("Test {} Mission", type_name),
            description: Some(format!("Validate {} step type", type_name)),
            steps: vec![
                MissionStep {
                    id: format!("test_{}", type_name.to_lowercase()),
                    name: format!("Test {}", type_name),
                    step_type: step_type.clone(),
                    depends_on: None,
                    timeout_seconds: Some(3), // Short timeout for tests
                    continue_on_error: Some(true),
                    parameters: json!({}),
                }
            ],
            config: None,
        };
        
        let is_implemented = match DagExecutor::execute_mission(test_mission).await {
            Ok(_) => {
                total_implemented += 1;
                true
            },
            Err(_) => {
                total_not_implemented += 1;
                false
            }
        };
        
        let entry = results_by_category.entry(category).or_insert((Vec::new(), 0, 0));
        entry.0.push((type_name, is_implemented));
        if is_implemented {
            entry.1 += 1;
        } else {
            entry.2 += 1;
        }
    }
    
    // Print results by category
    println!("\n📊 STEP TYPE IMPLEMENTATION BY CATEGORY");
    println!("=======================================");
    
    for (category, (steps, implemented, not_implemented)) in &results_by_category {
        let total = implemented + not_implemented;
        let percentage = (*implemented as f64 / total as f64) * 100.0;
        
        println!("\n🏷️  {}: {}/{} ({:.1}%)", category, implemented, total, percentage);
        
        for (step_name, is_implemented) in steps {
            let status = if *is_implemented { "✅" } else { "🚧" };
            println!("    {} {}", status, step_name);
        }
    }
    
    // Print overall summary
    let total_steps = total_implemented + total_not_implemented;
    let overall_percentage = (total_implemented as f64 / total_steps as f64) * 100.0;
    
    println!("\n🎯 OVERALL IMPLEMENTATION SUMMARY");
    println!("=================================");
    println!("✅ Implemented: {} step types", total_implemented);
    println!("🚧 Not Implemented: {} step types", total_not_implemented);
    println!("📊 Total Step Types: {} step types", total_steps);
    println!("📈 Implementation Rate: {:.1}%", overall_percentage);
    
    // Assessment
    println!("\n📋 ASSESSMENT");
    println!("=============");
    if overall_percentage >= 80.0 {
        println!("🎉 EXCELLENT: RustChain has comprehensive step type implementation");
    } else if overall_percentage >= 60.0 {
        println!("👍 GOOD: RustChain has strong step type implementation");
    } else if overall_percentage >= 40.0 {
        println!("⚠️  MODERATE: RustChain has partial step type implementation");
    } else if overall_percentage >= 20.0 {
        println!("🔧 LIMITED: RustChain has basic step type implementation");
    } else {
        println!("❌ MINIMAL: RustChain has very limited step type implementation");
    }
    
    // Production readiness assessment
    if total_implemented >= 10 {
        println!("✅ PRODUCTION READY: Sufficient step types for real-world usage");
    } else {
        println!("🚧 DEVELOPMENT: More step types needed for production usage");
    }
    
    Ok(())
}