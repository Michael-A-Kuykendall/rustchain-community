//! Common utilities and types for transpilation

use crate::engine::{Mission, MissionStep, StepType};
use std::collections::HashMap;

/// Common transpilation context
#[derive(Debug, Clone)]
pub struct TranspilationContext {
    pub variables: HashMap<String, String>,
    pub step_counter: usize,
    pub mission_name: String,
    pub mission_version: String,
}

impl TranspilationContext {
    pub fn new(mission_name: String) -> Self {
        Self {
            variables: HashMap::new(),
            step_counter: 0,
            mission_name,
            mission_version: "1.0".to_string(),
        }
    }

    pub fn next_step_id(&mut self) -> String {
        self.step_counter += 1;
        format!("step_{}", self.step_counter)
    }

    pub fn add_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }
}

/// Utility functions for common transpilation tasks
pub struct TranspilerUtils;

impl TranspilerUtils {
    /// Convert a simple string template to RustChain variable format
    /// "{variable}" -> "{{variable}}"
    pub fn convert_template_variables(template: &str) -> String {
        // Convert Python f-string style {var} to RustChain {{var}}
        let mut result = template.to_string();
        
        // Find all {var} patterns and convert to {{var}}
        let re = regex::Regex::new(r"\{([^{}]+)\}").unwrap();
        result = re.replace_all(&result, "{{$1}}").to_string();
        
        result
    }

    /// Extract variable names from a template string
    pub fn extract_variables(template: &str) -> Vec<String> {
        let re = regex::Regex::new(r"\{\{([^{}]+)\}\}").unwrap();
        re.captures_iter(template)
            .map(|cap| cap[1].to_string())
            .collect()
    }

    /// Create a basic mission structure
    pub fn create_mission(name: String, description: Option<String>, steps: Vec<MissionStep>) -> Mission {
        Mission {
            version: "1.0".to_string(),
            name,
            description,
            steps,
            config: None,
        }
    }

    /// Create a basic LLM step
    pub fn create_llm_step(
        id: String,
        name: String,
        prompt: String,
        model: Option<String>,
        variables: Vec<String>,
    ) -> MissionStep {
        let mut parameters = serde_json::json!({
            "prompt": prompt
        });

        if let Some(model_name) = model {
            parameters["model"] = serde_json::Value::String(model_name);
        }

        if !variables.is_empty() {
            parameters["variables"] = serde_json::Value::Array(
                variables.into_iter().map(serde_json::Value::String).collect()
            );
        }

        MissionStep {
            id,
            name,
            step_type: StepType::Llm,
            parameters,
            depends_on: None,
            timeout_seconds: Some(60),
            continue_on_error: None,
        }
    }

    /// Create a basic tool step
    pub fn create_tool_step(
        id: String,
        name: String,
        tool_name: String,
        tool_params: serde_json::Value,
    ) -> MissionStep {
        let parameters = serde_json::json!({
            "tool": tool_name,
            "parameters": tool_params
        });

        MissionStep {
            id,
            name,
            step_type: StepType::Tool,
            parameters,
            depends_on: None,
            timeout_seconds: Some(30),
            continue_on_error: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpilation_context() {
        let mut ctx = TranspilationContext::new("test_mission".to_string());
        
        assert_eq!(ctx.mission_name, "test_mission");
        assert_eq!(ctx.step_counter, 0);
        
        let step_id = ctx.next_step_id();
        assert_eq!(step_id, "step_1");
        assert_eq!(ctx.step_counter, 1);
        
        ctx.add_variable("test_var".to_string(), "test_value".to_string());
        assert_eq!(ctx.variables.get("test_var"), Some(&"test_value".to_string()));
    }

    #[test]
    fn test_template_variable_conversion() {
        let template = "Hello {name}, welcome to {place}!";
        let result = TranspilerUtils::convert_template_variables(template);
        assert_eq!(result, "Hello {{name}}, welcome to {{place}}!");
    }

    #[test]
    fn test_extract_variables() {
        let template = "Hello {{name}}, welcome to {{place}}!";
        let variables = TranspilerUtils::extract_variables(template);
        assert_eq!(variables, vec!["name", "place"]);
    }

    #[test]
    fn test_create_llm_step() {
        let step = TranspilerUtils::create_llm_step(
            "test_step".to_string(),
            "Test LLM Step".to_string(),
            "What is {{input}}?".to_string(),
            Some("gpt-4".to_string()),
            vec!["input".to_string()],
        );

        assert_eq!(step.id, "test_step");
        assert_eq!(step.name, "Test LLM Step");
        assert!(matches!(step.step_type, StepType::Llm));
        
        let prompt = step.parameters.get("prompt").unwrap().as_str().unwrap();
        assert_eq!(prompt, "What is {{input}}?");
    }

    #[test]
    fn test_create_mission() {
        let steps = vec![
            TranspilerUtils::create_llm_step(
                "step1".to_string(),
                "Step 1".to_string(),
                "Test".to_string(),
                None,
                vec![],
            )
        ];

        let mission = TranspilerUtils::create_mission(
            "test_mission".to_string(),
            Some("A test mission".to_string()),
            steps,
        );

        assert_eq!(mission.name, "test_mission");
        assert_eq!(mission.description, Some("A test mission".to_string()));
        assert_eq!(mission.steps.len(), 1);
        assert_eq!(mission.version, "1.0");
    }
}