use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use crate::models::Message;

#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: OllamaMessage,
    #[serde(default)]
    #[allow(dead_code)]
    done: bool,
}

// Call Ollama API
pub async fn call_ollama(url: &str, model: &str, messages: &[Message]) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    // Convert messages to Ollama format
    let ollama_messages: Vec<OllamaMessage> = messages
        .iter()
        .map(|m| OllamaMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();
    
    let request = OllamaRequest {
        model: model.to_string(),
        messages: ollama_messages,
        stream: false,
    };
    
    let api_url = format!("{}/api/chat", url.trim_end_matches('/'));
    
    let response = client
        .post(&api_url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Ollama at {}: {}", api_url, e))?;
    
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Ollama API error ({}): {}", status, error_text));
    }
    
    let ollama_response: OllamaResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;
    
    Ok(ollama_response.message.content)
}

// Stream Ollama response
pub async fn stream_ollama(
    app: &AppHandle,
    event_name: &str,
    url: &str,
    model: &str,
    messages: &[Message],
    full_response: &mut String,
    periodic_save: Option<Box<dyn Fn(&str) -> Result<(), String> + Send + Sync>>,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    
    let ollama_messages: Vec<OllamaMessage> = messages
        .iter()
        .map(|m| OllamaMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();
    
    let request = serde_json::json!({
        "model": model,
        "messages": ollama_messages,
        "stream": true
    });
    
    let api_url = format!("{}/api/chat", url.trim_end_matches('/'));
    
    let response = client
        .post(&api_url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Ollama: {}", e))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Ollama API error: {}", error_text));
    }
    
    // Stream the response
    use futures_util::StreamExt;
    use std::time::{Duration, Instant};
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut last_save_time = Instant::now();
    const SAVE_INTERVAL: Duration = Duration::from_secs(2); // Save every 2 seconds
    
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| format!("Stream error: {}", e))?;
        buffer.push_str(&String::from_utf8_lossy(&chunk));
        
        // Process complete lines
        while let Some(newline_pos) = buffer.find('\n') {
            let line = buffer[..newline_pos].trim().to_string();
            buffer = buffer[newline_pos + 1..].to_string();
            
            if line.is_empty() {
                continue;
            }
            
            // Parse JSON line
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                if let Some(content) = json.get("message").and_then(|m| m.get("content")).and_then(|c| c.as_str()) {
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
                
                // Check if done
                if json.get("done").and_then(|d| d.as_bool()).unwrap_or(false) {
                    break;
                }
            }
        }
    }
    
    Ok(())
}

