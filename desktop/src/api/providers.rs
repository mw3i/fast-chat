use tauri::AppHandle;
use serde_json;

use crate::models::Message;
use crate::storage::settings::load_settings;
use crate::api::{ollama, openai, google, lmstudio, custom};

// Call LLM - supports multiple providers
pub async fn call_llm(app: &AppHandle, messages: &[Message]) -> Result<String, String> {
    // Load settings to determine which service to use
    let settings = load_settings(app)?;
    
    let provider = settings
        .get("provider")
        .and_then(|v| v.as_str())
        .unwrap_or("ollama")
        .to_string();
    
    let provider_params_value = settings
        .get("provider-params")
        .ok_or_else(|| "Missing provider-params in settings".to_string())?;
    
    // Parse provider_params as a JSON object
    let provider_params: crate::models::ProviderParams = serde_json::from_value(provider_params_value.clone())
        .map_err(|e| format!("Failed to parse provider-params: {}", e))?;
    
    match provider.as_str() {
        "ollama" => {
            ollama::call_ollama(&provider_params.ollama.url, &provider_params.ollama.model, messages).await
        }
        "openai" => {
            openai::call_openai(&provider_params.openai.api_key, &provider_params.openai.web_search_enabled, messages).await
        }
        "google" => {
            google::call_google(&provider_params.google.api_key, messages).await
        }
        "lmstudio" => {
            lmstudio::call_lmstudio(&provider_params.lmstudio.url, &provider_params.lmstudio.model, messages).await
        }
        "custom" => {
            custom::call_custom(&provider_params.custom.url, messages).await
        }
        _ => Err(format!("Unknown provider: {}", provider)),
    }
}

pub async fn stream_llm(
    app: &AppHandle,
    event_name: &str,
    messages: &[Message],
    full_response: &mut String,
) -> Result<(), String> {
    // Load settings
    let settings = load_settings(app)?;
    let provider = settings
        .get("provider")
        .and_then(|v| v.as_str())
        .unwrap_or("ollama")
        .to_string();
    
    let provider_params_value = settings
        .get("provider-params")
        .ok_or_else(|| "Missing provider-params in settings".to_string())?;
    
    let provider_params: crate::models::ProviderParams = serde_json::from_value(provider_params_value.clone())
        .map_err(|e| format!("Failed to parse provider-params: {}", e))?;
    
    match provider.as_str() {
        "ollama" => {
            ollama::stream_ollama(
                app,
                event_name,
                &provider_params.ollama.url,
                &provider_params.ollama.model,
                messages,
                full_response,
            ).await
        }
        "openai" => {
            openai::stream_openai(
                app,
                event_name,
                &provider_params.openai.api_key,
                &provider_params.openai.web_search_enabled,
                messages,
                full_response,
            ).await
        }
        _ => {
            Err(format!("Streaming not yet implemented for provider: {}", provider))
        }
    }
}

