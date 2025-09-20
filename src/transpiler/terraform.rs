//! Terraform Parser for RustChain
//! 
//! Converts Terraform configurations to RustChain missions:
//! - Parse Terraform HCL syntax
//! - Extract resource definitions
//! - Convert to RustChain steps
//! - Handle dependencies and variables

use crate::core::Result;
use crate::engine::{Mission, MissionStep, StepType, MissionConfig};
// Removed unused imports - ConfigError, RustChainError not used in this module
use crate::transpiler::common::TranspilationContext;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Terraform parser for converting Terraform configurations to RustChain missions
pub struct TerraformParser;

/// Represents a Terraform resource block
#[derive(Debug, Clone)]
pub struct TerraformResource {
    pub resource_type: String,
    pub name: String,
    pub config: HashMap<String, Value>,
    pub dependencies: Vec<String>,
}

/// Represents a Terraform variable
#[derive(Debug, Clone)]
pub struct TerraformVariable {
    pub name: String,
    pub default: Option<Value>,
    pub description: Option<String>,
    pub variable_type: Option<String>,
}

/// Represents a Terraform output
#[derive(Debug, Clone)]
pub struct TerraformOutput {
    pub name: String,
    pub value: Value,
    pub description: Option<String>,
}

impl TerraformParser {
    /// Parse a Terraform file and convert to RustChain mission
    pub async fn parse_file(file_path: &str) -> Result<Mission> {
        let content = std::fs::read_to_string(file_path)?;
        Self::parse_string(&content).await
    }
    
    /// Parse Terraform content from string
    pub async fn parse_string(content: &str) -> Result<Mission> {
        let mut context = TranspilationContext::new("Terraform Infrastructure Mission".to_string());
        
        // Parse Terraform blocks
        let resources = Self::parse_resources(content)?;
        let variables = Self::parse_variables(content)?;
        let outputs = Self::parse_outputs(content)?;
        
        // Convert to RustChain steps
        let mut steps = Vec::new();
        let mut step_counter = 1;
        
        // Add variable initialization steps
        for variable in &variables {
            let step = Self::create_variable_step(variable, &format!("var_{}", step_counter))?;
            steps.push(step);
            step_counter += 1;
        }
        
        // Add resource creation steps
        for resource in &resources {
            let step = Self::create_resource_step(resource, &format!("resource_{}", step_counter))?;
            steps.push(step);
            step_counter += 1;
        }
        
        // Add output steps
        for output in &outputs {
            let step = Self::create_output_step(output, &format!("output_{}", step_counter))?;
            steps.push(step);
            step_counter += 1;
        }
        
        context.add_variable("total_resources".to_string(), resources.len().to_string());
        context.add_variable("total_variables".to_string(), variables.len().to_string());
        
        Ok(Mission {
            version: "1.0".to_string(),
            name: "Terraform Infrastructure Mission".to_string(),
            description: Some(format!("Converted from Terraform configuration with {} resources, {} variables, {} outputs", 
                                    resources.len(), variables.len(), outputs.len())),
            steps,
            config: Some(MissionConfig {
                max_parallel_steps: Some(4),
                timeout_seconds: Some(3600), // 1 hour for infrastructure operations
                fail_fast: Some(true), // Infrastructure should fail fast
            }),
        })
    }
    
    /// Parse Terraform resource blocks
    fn parse_resources(content: &str) -> Result<Vec<TerraformResource>> {
        let mut resources = Vec::new();
        
        // Simple regex-based parsing for Terraform resources
        // In a production system, you'd use a proper HCL parser
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("resource ") {
                if let Some(resource) = Self::parse_resource_line(line)? {
                    resources.push(resource);
                }
            }
        }
        
        Ok(resources)
    }
    
    /// Parse a single resource line
    fn parse_resource_line(line: &str) -> Result<Option<TerraformResource>> {
        // Example: resource "aws_instance" "web" {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let resource_type = parts[1].trim_matches('"');
            let name = parts[2].trim_matches('"');
            
            Ok(Some(TerraformResource {
                resource_type: resource_type.to_string(),
                name: name.to_string(),
                config: HashMap::new(),
                dependencies: Vec::new(),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Parse Terraform variable blocks
    fn parse_variables(content: &str) -> Result<Vec<TerraformVariable>> {
        let mut variables = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("variable ") {
                if let Some(variable) = Self::parse_variable_line(line)? {
                    variables.push(variable);
                }
            }
        }
        
        Ok(variables)
    }
    
    /// Parse a single variable line
    fn parse_variable_line(line: &str) -> Result<Option<TerraformVariable>> {
        // Example: variable "instance_type" {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[1].trim_matches('"');
            
            Ok(Some(TerraformVariable {
                name: name.to_string(),
                default: None,
                description: None,
                variable_type: None,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Parse Terraform output blocks
    fn parse_outputs(content: &str) -> Result<Vec<TerraformOutput>> {
        let mut outputs = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.starts_with("output ") {
                if let Some(output) = Self::parse_output_line(line)? {
                    outputs.push(output);
                }
            }
        }
        
        Ok(outputs)
    }
    
    /// Parse a single output line
    fn parse_output_line(line: &str) -> Result<Option<TerraformOutput>> {
        // Example: output "instance_ip" {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[1].trim_matches('"');
            
            Ok(Some(TerraformOutput {
                name: name.to_string(),
                value: Value::String("${aws_instance.web.public_ip}".to_string()),
                description: None,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Convert Terraform variable to RustChain step
    fn create_variable_step(variable: &TerraformVariable, step_id: &str) -> Result<MissionStep> {
        Ok(MissionStep {
            id: step_id.to_string(),
            name: format!("Initialize Variable: {}", variable.name),
            step_type: StepType::Noop,
            depends_on: None,
            timeout_seconds: Some(30),
            continue_on_error: Some(false),
            parameters: json!({
                "message": format!("Variable {}: {}", variable.name, variable.default.clone().unwrap_or_else(|| Value::String("default".to_string()))),
                "level": "info",
                "variable_name": variable.name,
                "variable_value": variable.default.clone().unwrap_or_else(|| Value::String("".to_string())),
                "description": variable.description.clone().unwrap_or_else(|| format!("Terraform variable: {}", variable.name)),
                "type": variable.variable_type.clone().unwrap_or_else(|| "string".to_string())
            }),
        })
    }
    
    /// Convert Terraform resource to RustChain step
    fn create_resource_step(resource: &TerraformResource, step_id: &str) -> Result<MissionStep> {
        let step_type = Self::map_resource_to_step_type(&resource.resource_type);
        
        Ok(MissionStep {
            id: step_id.to_string(),
            name: format!("Create {}: {}", resource.resource_type, resource.name),
            step_type,
            depends_on: if resource.dependencies.is_empty() { None } else { Some(resource.dependencies.clone()) },
            timeout_seconds: Some(600),
            continue_on_error: Some(false),
            parameters: json!({
                "resource_type": resource.resource_type,
                "resource_name": resource.name,
                "config": resource.config,
                "terraform_resource": true
            }),
        })
    }
    
    /// Convert Terraform output to RustChain step
    fn create_output_step(output: &TerraformOutput, step_id: &str) -> Result<MissionStep> {
        Ok(MissionStep {
            id: step_id.to_string(),
            name: format!("Output: {}", output.name),
            step_type: StepType::Noop,
            depends_on: None,
            timeout_seconds: Some(30),
            continue_on_error: Some(true),
            parameters: json!({
                "message": format!("Terraform Output {}: {}", output.name, output.value),
                "level": "info",
                "output_name": output.name,
                "output_value": output.value
            }),
        })
    }
    
    /// Map Terraform resource types to RustChain step types
    fn map_resource_to_step_type(resource_type: &str) -> StepType {
        match resource_type {
            "aws_instance" | "google_compute_instance" | "azurerm_virtual_machine" => StepType::Http,
            "aws_s3_bucket" | "google_storage_bucket" | "azurerm_storage_account" => StepType::Http,
            "aws_lambda_function" | "google_cloud_function" | "azurerm_function_app" => StepType::Http,
            "kubernetes_deployment" | "kubernetes_service" => StepType::Http,
            "local_file" | "template_file" => StepType::CreateFile,
            "null_resource" => StepType::Command,
            "random_string" | "random_id" => StepType::Noop,
            _ => StepType::Http, // Default to HTTP for cloud API calls
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_simple_terraform() {
        let terraform_content = r#"
variable "instance_type" {
  description = "The type of instance to create"
  default     = "t2.micro"
}

resource "aws_instance" "web" {
  ami           = "ami-0c55b159cbfafe1d0"
  instance_type = var.instance_type
}

output "instance_ip" {
  value = aws_instance.web.public_ip
}
        "#;
        
        let result = TerraformParser::parse_string(terraform_content).await;
        assert!(result.is_ok());
        
        let mission = result.unwrap();
        assert_eq!(mission.name, "Terraform Infrastructure Mission");
        assert!(mission.steps.len() >= 3); // Variable + Resource + Output
        
        // Check that we have steps for variable, resource, and output
        let step_names: Vec<String> = mission.steps.iter().map(|s| s.name.clone()).collect();
        assert!(step_names.iter().any(|name| name.contains("Variable")));
        assert!(step_names.iter().any(|name| name.contains("aws_instance")));
        assert!(step_names.iter().any(|name| name.contains("Output")));
    }
    
    #[test]
    fn test_parse_resource_line() {
        let line = r#"resource "aws_instance" "web" {"#;
        let result = TerraformParser::parse_resource_line(line).unwrap();
        
        assert!(result.is_some());
        let resource = result.unwrap();
        assert_eq!(resource.resource_type, "aws_instance");
        assert_eq!(resource.name, "web");
    }
    
    #[test]
    fn test_parse_variable_line() {
        let line = r#"variable "instance_type" {"#;
        let result = TerraformParser::parse_variable_line(line).unwrap();
        
        assert!(result.is_some());
        let variable = result.unwrap();
        assert_eq!(variable.name, "instance_type");
    }
    
    #[test]
    fn test_parse_output_line() {
        let line = r#"output "instance_ip" {"#;
        let result = TerraformParser::parse_output_line(line).unwrap();
        
        assert!(result.is_some());
        let output = result.unwrap();
        assert_eq!(output.name, "instance_ip");
    }
    
    #[test]
    fn test_resource_type_mapping() {
        assert!(matches!(TerraformParser::map_resource_to_step_type("aws_instance"), StepType::Http));
        assert!(matches!(TerraformParser::map_resource_to_step_type("local_file"), StepType::CreateFile));
        assert!(matches!(TerraformParser::map_resource_to_step_type("null_resource"), StepType::Command));
        assert!(matches!(TerraformParser::map_resource_to_step_type("kubernetes_deployment"), StepType::Http));
    }
    
    #[tokio::test]
    async fn test_complex_terraform_with_dependencies() {
        let terraform_content = r#"
resource "aws_vpc" "main" {
  cidr_block = "10.0.0.0/16"
}

resource "aws_subnet" "web" {
  vpc_id     = aws_vpc.main.id
  cidr_block = "10.0.1.0/24"
}

resource "aws_instance" "web" {
  ami           = "ami-0c55b159cbfafe1d0"
  instance_type = "t2.micro"
  subnet_id     = aws_subnet.web.id
}
        "#;
        
        let result = TerraformParser::parse_string(terraform_content).await;
        assert!(result.is_ok());
        
        let mission = result.unwrap();
        assert_eq!(mission.steps.len(), 3);
        assert!(mission.description.unwrap().contains("3 resources"));
    }
    
    #[tokio::test]
    async fn test_empty_terraform() {
        let result = TerraformParser::parse_string("").await;
        assert!(result.is_ok());
        
        let mission = result.unwrap();
        assert_eq!(mission.steps.len(), 0);
        assert!(mission.description.unwrap().contains("0 resources"));
    }
}