use crate::core::{Mission, MissionStep, Result, RustChainError, ToolError};
use std::path::Path;
use std::process::Command;

pub struct MissionExecutor;

impl MissionExecutor {
    pub fn new() -> Self {
        Self
    }
    
    // Helper function to get parameter as string
    fn get_param(&self, step: &MissionStep, key: &str) -> Option<String> {
        if let Some(params) = &step.parameters {
            params.get(key)?.as_str().map(String::from)
        } else {
            None
        }
    }
    
    // Helper function to get file path from parameters or legacy field  
    fn get_file_path(&self, step: &MissionStep) -> Option<String> {
        self.get_param(step, "path").or_else(|| step.file_path.clone())
    }
    
    // Helper function to get content from parameters or legacy field
    fn get_content(&self, step: &MissionStep) -> Option<String> {
        self.get_param(step, "content").or_else(|| step.content.clone())
    }
    
    // Helper function to get command from parameters or legacy field
    fn get_command(&self, step: &MissionStep) -> Option<String> {
        self.get_param(step, "command").or_else(|| step.command.clone())
    }
    
    pub async fn execute_mission(&self, mission: Mission) -> Result<()> {
        println!("🚀 Executing mission: {}", mission.name);
        
        for (i, step) in mission.steps.iter().enumerate() {
            println!("📋 Step {}/{}: {}", i + 1, mission.steps.len(), step.id);
            self.execute_step(step).await?;
        }
        
        println!("✅ Mission completed: {}", mission.name);
        Ok(())
    }
    
    async fn execute_step(&self, step: &MissionStep) -> Result<()> {
        match step.step_type.as_str() {
            "create" | "create_file" => self.execute_create_step(step),
            "edit" => self.execute_edit_step(step),
            "command" => self.execute_command_step(step),
            "test" => self.execute_test_step(step),
            _ => Err(RustChainError::Tool(ToolError::NotFound { 
                tool_name: format!("step_type_{}", step.step_type) 
            })),
        }
    }
    
    fn execute_create_step(&self, step: &MissionStep) -> Result<()> {
        if let (Some(file_path), Some(content)) = (self.get_file_path(step), self.get_content(step)) {
            let path = Path::new(&file_path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(path, &content)?;
            println!("📝 Created: {}", file_path);
        } else {
            return Err(RustChainError::Tool(ToolError::InvalidParameters { 
                tool_name: "create_file".to_string(),
                details: "Missing file_path or content parameters".to_string()
            }));
        }
        Ok(())
    }
    
    fn execute_edit_step(&self, step: &MissionStep) -> Result<()> {
        if let (Some(file_path), Some(content)) = (self.get_file_path(step), self.get_content(step)) {
            let existing = std::fs::read_to_string(&file_path).unwrap_or_default();
            let new_content = format!("{}\n{}", existing.trim(), content.trim());
            std::fs::write(&file_path, new_content)?;
            println!("✏️ Edited: {}", file_path);
        } else {
            return Err(RustChainError::Tool(ToolError::InvalidParameters { 
                tool_name: "edit_file".to_string(),
                details: "Missing file_path or content parameters".to_string()
            }));
        }
        Ok(())
    }
    
    fn execute_command_step(&self, step: &MissionStep) -> Result<()> {
        if let Some(command) = self.get_command(step) {
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()?;
            
            if output.status.success() {
                println!("🔧 Command succeeded: {}", command);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(RustChainError::Tool(ToolError::ExecutionFailed { 
                    tool_name: "command".to_string(),
                    reason: format!("Command failed: {}", stderr)
                }));
            }
        } else {
            return Err(RustChainError::Tool(ToolError::InvalidParameters { 
                tool_name: "command".to_string(),
                details: "Missing command parameter".to_string()
            }));
        }
        Ok(())
    }
    
    fn execute_test_step(&self, step: &MissionStep) -> Result<()> {
        let default_lang = "rust".to_string();
        let language = step.language.as_ref().unwrap_or(&default_lang);
        
        match language.as_str() {
            "rust" => {
                let output = Command::new("cargo")
                    .args(&["test"])
                    .output()?;
                
                if output.status.success() {
                    println!("✅ Tests passed");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!("⚠️ Test output: {}", stderr);
                }
            }
            _ => println!("🧪 Test step for {} (not implemented)", language),
        }
        
        Ok(())
    }
}
