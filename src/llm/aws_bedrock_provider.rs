// AWS Bedrock LLM Provider Implementation
use super::{ChatMessage, LLMProvider, LLMRequest, LLMResponse, MessageRole, ModelInfo};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info};

#[derive(Debug, Clone)]
pub struct AWSBedrockProvider {
    client: reqwest::Client,
    aws_access_key_id: String,
    #[allow(dead_code)]
    aws_secret_access_key: String,
    aws_session_token: Option<String>,
    region: String,
    model: String,
}

impl AWSBedrockProvider {
    pub fn new(
        aws_access_key_id: String,
        aws_secret_access_key: String,
        region: String,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            aws_access_key_id,
            aws_secret_access_key,
            aws_session_token: None,
            region,
            model: "anthropic.claude-3-sonnet-20240229-v1:0".to_string(),
        }
    }

    pub fn with_session_token(mut self, session_token: String) -> Self {
        self.aws_session_token = Some(session_token);
        self
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn with_region(mut self, region: String) -> Self {
        self.region = region;
        self
    }

    fn convert_messages_to_bedrock(&self, messages: &[ChatMessage]) -> (Option<String>, Vec<BedrockMessage>) {
        let mut system_message = None;
        let mut bedrock_messages = Vec::new();
        
        for message in messages {
            match message.role {
                MessageRole::System => {
                    system_message = Some(message.content.clone());
                }
                MessageRole::User => {
                    bedrock_messages.push(BedrockMessage {
                        role: "user".to_string(),
                        content: vec![BedrockContent {
                            content_type: "text".to_string(),
                            text: Some(message.content.clone()),
                        }],
                    });
                }
                MessageRole::Assistant => {
                    bedrock_messages.push(BedrockMessage {
                        role: "assistant".to_string(),
                        content: vec![BedrockContent {
                            content_type: "text".to_string(),
                            text: Some(message.content.clone()),
                        }],
                    });
                }
                MessageRole::Tool => {
                    // Convert tool responses to user messages in Bedrock
                    bedrock_messages.push(BedrockMessage {
                        role: "user".to_string(),
                        content: vec![BedrockContent {
                            content_type: "text".to_string(),
                            text: Some(format!("Tool response: {}", message.content)),
                        }],
                    });
                }
            }
        }
        
        (system_message, bedrock_messages)
    }

    fn get_bedrock_endpoint(&self, model: &str) -> String {
        format!(
            "https://bedrock-runtime.{}.amazonaws.com/model/{}/invoke",
            self.region, model
        )
    }

    async fn sign_request(&self, _method: &str, _url: &str, _payload: &str) -> Result<HashMap<String, String>> {
        // This is a simplified AWS signature implementation
        // In production, you would use the aws-sdk-rust or aws-sigv4 crate
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("AWS4-HMAC-SHA256 Credential={}/...", self.aws_access_key_id));
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("X-Amz-Target".to_string(), "AmazonBedrockFrontendService.InvokeModel".to_string());
        
        if let Some(session_token) = &self.aws_session_token {
            headers.insert("X-Amz-Security-Token".to_string(), session_token.clone());
        }
        
        Ok(headers)
    }
}

#[async_trait]
impl LLMProvider for AWSBedrockProvider {
    async fn complete(&self, request: LLMRequest) -> Result<LLMResponse> {
        debug!("AWS Bedrock completion request: {:?}", request.model);
        
        let model = request.model.as_ref().unwrap_or(&self.model);
        let url = self.get_bedrock_endpoint(model);
        
        let (system_message, messages) = self.convert_messages_to_bedrock(&request.messages);
        
        let bedrock_request = BedrockRequest {
            anthropic_version: "bedrock-2023-05-31".to_string(),
            max_tokens: request.max_tokens.unwrap_or(2048),
            messages,
            system: system_message,
            temperature: request.temperature,
            top_p: None,
            top_k: None,
            stop_sequences: None,
        };

        let payload = serde_json::to_string(&bedrock_request)?;
        let headers = self.sign_request("POST", &url, &payload).await?;
        
        debug!("Sending request to Bedrock: {}", url);
        
        let mut req = self
            .client
            .post(&url)
            .body(payload);
            
        for (key, value) in headers {
            req = req.header(&key, &value);
        }
        
        let response = req.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Bedrock API error: {} - {}", status, error_text);
            return Err(anyhow!("Bedrock API error: {} - {}", status, error_text));
        }

        let bedrock_response: BedrockResponse = response.json().await?;
        
        let content = bedrock_response
            .content
            .first()
            .and_then(|c| c.text.as_ref())
            .cloned()
            .unwrap_or_default();

        let usage = super::TokenUsage {
            prompt_tokens: bedrock_response.usage.input_tokens,
            completion_tokens: bedrock_response.usage.output_tokens,
            total_tokens: bedrock_response.usage.input_tokens + bedrock_response.usage.output_tokens,
        };

        let finish_reason = match bedrock_response.stop_reason.as_deref().unwrap_or("stop") {
            "stop_sequence" | "end_turn" | "stop" => super::FinishReason::Stop,
            "max_tokens" => super::FinishReason::Length,
            "tool_use" => super::FinishReason::ToolCalls,
            "content_filtered" => super::FinishReason::ContentFilter,
            _ => super::FinishReason::Error,
        };

        info!("Bedrock completion successful, {} input tokens, {} output tokens", 
              bedrock_response.usage.input_tokens, bedrock_response.usage.output_tokens);

        Ok(LLMResponse {
            content,
            role: MessageRole::Assistant,
            model: model.clone(),
            usage,
            tool_calls: None, // Bedrock tool calls implementation requires AWS SDK - planned for future release
            finish_reason,
            metadata: HashMap::new(),
        })
    }

    async fn stream(
        &self,
        _request: LLMRequest,
    ) -> Result<Box<dyn futures::Stream<Item = Result<LLMResponse>> + Send + Unpin>> {
        // Streaming support for Bedrock requires AWS SDK and event stream handling
        // This is planned for future release when proper AWS SDK integration is implemented
        Err(anyhow!("Streaming not yet implemented for AWS Bedrock"))
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "anthropic.claude-3-sonnet-20240229-v1:0".to_string(),
                name: "Claude 3 Sonnet".to_string(),
                provider: "aws-bedrock".to_string(),
                context_length: 200_000,
                max_output_tokens: 4096,
                supports_tools: true,
                supports_streaming: false, // Streaming planned for future AWS SDK release
                cost_per_input_token: Some(0.000003), // $3 per 1M tokens
                cost_per_output_token: Some(0.000015), // $15 per 1M tokens
            },
            ModelInfo {
                id: "anthropic.claude-3-haiku-20240307-v1:0".to_string(),
                name: "Claude 3 Haiku".to_string(),
                provider: "aws-bedrock".to_string(),
                context_length: 200_000,
                max_output_tokens: 4096,
                supports_tools: true,
                supports_streaming: false,
                cost_per_input_token: Some(0.00000025), // $0.25 per 1M tokens
                cost_per_output_token: Some(0.00000125), // $1.25 per 1M tokens
            },
            ModelInfo {
                id: "amazon.titan-text-express-v1".to_string(),
                name: "Titan Text G1 - Express".to_string(),
                provider: "aws-bedrock".to_string(),
                context_length: 8000,
                max_output_tokens: 8000,
                supports_tools: false,
                supports_streaming: false,
                cost_per_input_token: Some(0.0000008), // $0.8 per 1M tokens
                cost_per_output_token: Some(0.0000016), // $1.6 per 1M tokens
            },
            ModelInfo {
                id: "ai21.j2-ultra-v1".to_string(),
                name: "Jurassic-2 Ultra".to_string(),
                provider: "aws-bedrock".to_string(),
                context_length: 8192,
                max_output_tokens: 8192,
                supports_tools: false,
                supports_streaming: false,
                cost_per_input_token: Some(0.0000188), // $18.8 per 1M tokens
                cost_per_output_token: Some(0.0000188), // $18.8 per 1M tokens
            },
            ModelInfo {
                id: "cohere.command-text-v14".to_string(),
                name: "Command Text".to_string(),
                provider: "aws-bedrock".to_string(),
                context_length: 4096,
                max_output_tokens: 4096,
                supports_tools: false,
                supports_streaming: false,
                cost_per_input_token: Some(0.0000015), // $1.5 per 1M tokens
                cost_per_output_token: Some(0.000002), // $2 per 1M tokens
            },
            ModelInfo {
                id: "meta.llama2-70b-chat-v1".to_string(),
                name: "Llama 2 70B Chat".to_string(),
                provider: "aws-bedrock".to_string(),
                context_length: 4096,
                max_output_tokens: 2048,
                supports_tools: false,
                supports_streaming: false,
                cost_per_input_token: Some(0.00000195), // $1.95 per 1M tokens
                cost_per_output_token: Some(0.00000256), // $2.56 per 1M tokens
            },
        ])
    }

    fn provider_name(&self) -> &str {
        "aws-bedrock"
    }

    fn supports_streaming(&self) -> bool {
        false // Streaming planned for future AWS SDK release
    }

    fn supports_tools(&self) -> bool {
        true
    }
}

// Bedrock API request/response structures
#[derive(Debug, Serialize)]
struct BedrockRequest {
    anthropic_version: String,
    max_tokens: u32,
    messages: Vec<BedrockMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BedrockMessage {
    role: String,
    content: Vec<BedrockContent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BedrockContent {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BedrockResponse {
    content: Vec<BedrockContent>,
    #[serde(rename = "stop_reason")]
    stop_reason: Option<String>,
    usage: BedrockUsage,
}

#[derive(Debug, Deserialize)]
struct BedrockUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bedrock_provider_creation() {
        let provider = AWSBedrockProvider::new(
            "test-access-key".to_string(),
            "test-secret-key".to_string(),
            "us-east-1".to_string(),
        );
        assert_eq!(provider.provider_name(), "aws-bedrock");
        assert_eq!(provider.region, "us-east-1");
        assert_eq!(provider.model, "anthropic.claude-3-sonnet-20240229-v1:0");
    }

    #[test]
    fn test_bedrock_provider_with_custom_model() {
        let provider = AWSBedrockProvider::new(
            "test-access-key".to_string(),
            "test-secret-key".to_string(),
            "us-west-2".to_string(),
        ).with_model("anthropic.claude-3-haiku-20240307-v1:0".to_string());
        
        assert_eq!(provider.model, "anthropic.claude-3-haiku-20240307-v1:0");
        assert_eq!(provider.region, "us-west-2");
    }

    #[test]
    fn test_bedrock_provider_with_session_token() {
        let provider = AWSBedrockProvider::new(
            "test-access-key".to_string(),
            "test-secret-key".to_string(),
            "us-east-1".to_string(),
        ).with_session_token("test-session-token".to_string());
        
        assert_eq!(provider.aws_session_token, Some("test-session-token".to_string()));
    }

    #[test]
    fn test_message_conversion() {
        let provider = AWSBedrockProvider::new(
            "test-access-key".to_string(),
            "test-secret-key".to_string(),
            "us-east-1".to_string(),
        );
        
        let messages = vec![
            ChatMessage {
                role: MessageRole::System,
                content: "You are a helpful assistant".to_string(),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatMessage {
                role: MessageRole::User,
                content: "Hello!".to_string(),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ];
        
        let (system, bedrock_messages) = provider.convert_messages_to_bedrock(&messages);
        assert_eq!(system, Some("You are a helpful assistant".to_string()));
        assert_eq!(bedrock_messages.len(), 1);
        assert_eq!(bedrock_messages[0].role, "user");
        assert_eq!(bedrock_messages[0].content[0].text, Some("Hello!".to_string()));
    }

    #[test]
    fn test_bedrock_endpoint_generation() {
        let provider = AWSBedrockProvider::new(
            "test-access-key".to_string(),
            "test-secret-key".to_string(),
            "us-east-1".to_string(),
        );
        
        let endpoint = provider.get_bedrock_endpoint("anthropic.claude-3-sonnet-20240229-v1:0");
        assert_eq!(
            endpoint,
            "https://bedrock-runtime.us-east-1.amazonaws.com/model/anthropic.claude-3-sonnet-20240229-v1:0/invoke"
        );
    }

    #[tokio::test]
    async fn test_list_models() {
        let provider = AWSBedrockProvider::new(
            "test-access-key".to_string(),
            "test-secret-key".to_string(),
            "us-east-1".to_string(),
        );
        
        let models = provider.list_models().await.unwrap();
        assert_eq!(models.len(), 6);
        assert!(models.iter().any(|m| m.id == "anthropic.claude-3-sonnet-20240229-v1:0"));
        assert!(models.iter().any(|m| m.id == "anthropic.claude-3-haiku-20240307-v1:0"));
        assert!(models.iter().any(|m| m.id == "amazon.titan-text-express-v1"));
        assert!(models.iter().any(|m| m.id == "ai21.j2-ultra-v1"));
        assert!(models.iter().any(|m| m.id == "cohere.command-text-v14"));
        assert!(models.iter().any(|m| m.id == "meta.llama2-70b-chat-v1"));
    }
}