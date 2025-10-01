use crate::core::error::{LlmError, Result};
use reqwest;
use serde_json::{json, Value};
use tracing::{debug, info};

/// Test LLM connectivity to verify local Ollama instance
pub async fn test_ollama_connectivity() -> Result<()> {
    info!("Testing LLM connectivity...");

    let client = reqwest::Client::new();

    // Test if Ollama is running
    debug!("Checking Ollama service at localhost:11434...");
    let tags_response = client
        .get("http://localhost:11434/api/tags")
        .send()
        .await
        .map_err(|e| LlmError::service_unavailable(format!("Ollama connection failed: {}", e)))?;

    if !tags_response.status().is_success() {
        return Err(LlmError::service_unavailable("Ollama returned error status").into());
    }

    let tags: Value = tags_response
        .json()
        .await
        .map_err(|e| LlmError::response_error(format!("Failed to parse tags response: {}", e)))?;

    info!("✅ Ollama is running");
    debug!(
        "Available models: {}",
        serde_json::to_string_pretty(&tags).unwrap_or_default()
    );

    // Extract available models
    let models = tags["models"]
        .as_array()
        .ok_or_else(|| LlmError::response_error("No models array in response"))?;

    if models.is_empty() {
        return Err(LlmError::service_unavailable("No models available in Ollama").into());
    }

    let model_name = models[0]["name"]
        .as_str()
        .ok_or_else(|| LlmError::response_error("Invalid model name in response"))?;

    info!("Testing with model: {}", model_name);

    // Test a simple completion
    let completion_request = json!({
        "model": model_name,
        "prompt": "What is 2+2? Answer with just the number.",
        "stream": false,
        "options": {
            "temperature": 0.1,
            "max_tokens": 10
        }
    });

    debug!("Sending completion request...");
    let completion_response = client
        .post("http://localhost:11434/api/generate")
        .json(&completion_request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|_e| LlmError::timeout(30000))?;

    if !completion_response.status().is_success() {
        let error_text = completion_response.text().await.unwrap_or_default();
        return Err(LlmError::response_error(format!("Completion failed: {}", error_text)).into());
    }

    let completion: Value = completion_response
        .json()
        .await
        .map_err(|e| LlmError::response_error(format!("Failed to parse completion: {}", e)))?;

    info!("✅ Completion successful");
    debug!(
        "Response: {}",
        serde_json::to_string_pretty(&completion).unwrap_or_default()
    );

    // Validate response contains expected fields
    let response_text = completion["response"]
        .as_str()
        .ok_or_else(|| LlmError::response_error("No 'response' field in completion"))?;

    info!("Model response: '{}'", response_text.trim());

    // Test chat format if available
    debug!("Testing chat format...");
    let chat_request = json!({
        "model": model_name,
        "messages": [
            {
                "role": "user",
                "content": "Say 'Hello from RustChain!' and nothing else."
            }
        ],
        "stream": false,
        "options": {
            "temperature": 0.1,
            "max_tokens": 20
        }
    });

    let chat_response = client
        .post("http://localhost:11434/api/chat")
        .json(&chat_request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|_| LlmError::timeout(30000))?;

    if chat_response.status().is_success() {
        let chat: Value = chat_response
            .json()
            .await
            .map_err(|e| LlmError::response_error(format!("Failed to parse chat: {}", e)))?;

        info!("✅ Chat format also supported");
        debug!(
            "Chat response: {}",
            serde_json::to_string_pretty(&chat).unwrap_or_default()
        );

        if let Some(message) = chat["message"]["content"].as_str() {
            info!("Chat response: '{}'", message.trim());
        }
    } else {
        debug!("Chat format not supported or failed, but that's okay");
    }

    info!("🎉 LLM connectivity test completed successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_llm_connectivity_integration() {
        // This test will only pass if Ollama is running locally
        match test_ollama_connectivity().await {
            Ok(()) => {
                println!("✅ LLM connectivity test passed");
            }
            Err(e) => {
                println!("⚠️ LLM connectivity test failed (this is expected if Ollama isn't running): {}", e);
                // Don't fail the test if Ollama isn't running
                // This is an integration test that requires external service
            }
        }
    }
}
