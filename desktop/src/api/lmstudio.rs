use serde::{Deserialize, Serialize};
use crate::models::Message;

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

// Call LM Studio API (similar to Ollama)
pub async fn call_lmstudio(url: &str, model: &str, messages: &[Message]) -> Result<String, String> {
    // LM Studio uses OpenAI-compatible API
    let client = reqwest::Client::new();
    
    let openai_messages: Vec<OpenAIMessage> = messages
        .iter()
        .map(|m| OpenAIMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();
    
    let request = OpenAIRequest {
        model: model.to_string(),
        messages: openai_messages,
        tools: None, // LM Studio doesn't support web search
    };
    
    let api_url = format!("{}/v1/chat/completions", url.trim_end_matches('/'));
    
    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to LM Studio at {}: {}", api_url, e))?;
    
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("LM Studio API error ({}): {}", status, error_text));
    }
    
    let lmstudio_response: OpenAIResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse LM Studio response: {}", e))?;
    
    let content = lmstudio_response
        .choices
        .first()
        .and_then(|c| Some(c.message.content.clone()))
        .ok_or_else(|| "No response from LM Studio".to_string())?;
    
    Ok(content)
}

