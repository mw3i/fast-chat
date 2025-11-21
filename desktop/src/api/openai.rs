use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use crate::models::Message;

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAITool {
    #[serde(rename = "type")]
    tool_type: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAITool>>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

// Call OpenAI API
pub async fn call_openai(api_key: &str, web_search_enabled: &bool, messages: &[Message]) -> Result<String, String> {
    if api_key.is_empty() {
        return Err("OpenAI API key is required".to_string());
    }
    
    let client = reqwest::Client::new();
    
    // Convert messages to OpenAI format
    let openai_messages: Vec<OpenAIMessage> = messages
        .iter()
        .map(|m| OpenAIMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();
    
    // Use gpt-4o model (supports web search tool)
    let model = if *web_search_enabled {
        "gpt-4o".to_string()  // gpt-4o supports web_search_preview tool
    } else {
        "gpt-4o-mini".to_string()
    };
    
    // Include web_search_preview tool if enabled (note: tool type is web_search_preview, not web_search)
    let tools = if *web_search_enabled {
        Some(vec![OpenAITool {
            tool_type: "web_search_preview".to_string(),
        }])
    } else {
        None
    };
    
    let request = OpenAIRequest {
        model,
        messages: openai_messages,
        tools,
    };
    
    let url = "https://api.openai.com/v1/chat/completions";
    
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to OpenAI: {}", e))?;
    
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error ({}): {}", status, error_text));
    }
    
    let openai_response: OpenAIResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;
    
    let content = openai_response
        .choices
        .first()
        .and_then(|c| Some(c.message.content.clone()))
        .ok_or_else(|| "No response from OpenAI".to_string())?;
    
    Ok(content)
}

// Stream OpenAI response
pub async fn stream_openai(
    app: &AppHandle,
    event_name: &str,
    api_key: &str,
    web_search_enabled: &bool,
    messages: &[Message],
    full_response: &mut String,
    periodic_save: Option<Box<dyn Fn(&str) -> Result<(), String> + Send + Sync>>,
) -> Result<(), String> {
    if api_key.is_empty() {
        return Err("OpenAI API key is required".to_string());
    }
    
    let client = reqwest::Client::new();
    
    let openai_messages: Vec<OpenAIMessage> = messages
        .iter()
        .map(|m| OpenAIMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();
    
    // Use gpt-4o model (supports web search tool)
    let model = if *web_search_enabled {
        "gpt-4o"
    } else {
        "gpt-4o-mini"
    };
    
    // Build request with optional tools
    let mut request = serde_json::json!({
        "model": model,
        "messages": openai_messages,
        "stream": true
    });
    
    // Add web_search_preview tool if enabled (note: tool type is web_search_preview)
    if *web_search_enabled {
        request["tools"] = serde_json::json!([
            {
                "type": "web_search_preview"
            }
        ]);
    }
    
    let url = "https://api.openai.com/v1/chat/completions";
    
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to OpenAI: {}", e))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error: {}", error_text));
    }
    
    // Stream the response (SSE format)
    use futures_util::StreamExt;
    use std::time::{Duration, Instant};
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut last_save_time = Instant::now();
    const SAVE_INTERVAL: Duration = Duration::from_secs(2); // Save every 2 seconds
    
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| format!("Stream error: {}", e))?;
        buffer.push_str(&String::from_utf8_lossy(&chunk));
        
        // Process SSE lines
        while let Some(newline_pos) = buffer.find('\n') {
            let line = buffer[..newline_pos].trim().to_string();
            buffer = buffer[newline_pos + 1..].to_string();
            
            if line.starts_with("data: ") {
                let data = &line[6..];
                if data == "[DONE]" {
                    break;
                }
                
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                    if let Some(choices) = json.get("choices").and_then(|c| c.as_array()) {
                        if let Some(choice) = choices.first() {
                            if let Some(delta) = choice.get("delta") {
                                if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                                    full_response.push_str(content);
                                    app.emit(event_name, content).map_err(|e| format!("Failed to emit chunk: {}", e))?;
                                    
                                    // Periodic save (every 2 seconds)
                                    if let Some(ref save_callback) = periodic_save {
                                        if last_save_time.elapsed() >= SAVE_INTERVAL {
                                            if let Err(e) = save_callback(full_response) {
                                                eprintln!("Warning: Failed to save partial message: {}", e);
                                            }
                                            last_save_time = Instant::now();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

