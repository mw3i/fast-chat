use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use serde_json;

use crate::models::{Settings, default_shortcut};
use crate::encryption::{load_api_keys_from_encrypted, save_api_keys_to_encrypted, store_api_key_in_encrypted};

pub fn get_config_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("Failed to get app data directory")
        .join("config.json")
}

pub fn load_settings(app: &AppHandle) -> Result<HashMap<String, serde_json::Value>, String> {
    let config_path = get_config_path(app);
    
    // Load settings from file or use defaults
    let mut settings_json: serde_json::Value = if !config_path.exists() {
        // Return default settings if config doesn't exist
        serde_json::to_value(Settings::default())
            .map_err(|e| format!("Failed to serialize default settings: {}", e))?
    } else {
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                serde_json::from_str(&content)
                    .map_err(|e| format!("Failed to parse config.json: {}", e))?
            }
            Err(e) => return Err(format!("Failed to read config.json: {}", e)),
        }
    };
    
    // Migrate existing API keys from JSON to encrypted storage (one-time migration)
    // If we find non-empty API keys in JSON, move them to encrypted storage
    if let Some(obj) = settings_json.as_object_mut() {
        if let Some(provider_params) = obj.get_mut("provider-params") {
            migrate_api_keys_to_encrypted(app, provider_params)?;
        }
    }
    
    // Load API keys from encrypted storage into JSON
    load_api_keys_from_encrypted(app, &mut settings_json, "")?;
    
    // Convert to HashMap for frontend
    let map: HashMap<String, serde_json::Value> = serde_json::from_value(settings_json)
        .map_err(|e| format!("Failed to convert settings to HashMap: {}", e))?;
    
    Ok(map)
}

// Helper function to migrate API keys from JSON to encrypted storage (one-time migration)
fn migrate_api_keys_to_encrypted(app: &AppHandle, value: &mut serde_json::Value) -> Result<(), String> {
    migrate_api_keys_to_encrypted_recursive(app, value, "provider-params")
}

fn migrate_api_keys_to_encrypted_recursive(app: &AppHandle, value: &mut serde_json::Value, path_prefix: &str) -> Result<(), String> {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map.iter_mut() {
                let current_path = if path_prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path_prefix, key)
                };
                
                if key.ends_with("_key") {
                    if let serde_json::Value::String(s) = val {
                        // If we have a non-empty API key in JSON, migrate it to encrypted storage
                        if !s.is_empty() && s != "__NOT_STORED_HERE__" {
                            store_api_key_in_encrypted(app, &current_path, s)?;
                            // Set marker in JSON
                            *val = serde_json::Value::String("__NOT_STORED_HERE__".to_string());
                        }
                    }
                } else {
                    // Recursively process nested objects
                    migrate_api_keys_to_encrypted_recursive(app, val, &current_path)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn save_settings(app: &AppHandle, settings_map: HashMap<String, serde_json::Value>) -> Result<(), String> {
    let config_path = get_config_path(app);
    
    // Convert HashMap to JSON Value for processing
    let mut settings_json = serde_json::to_value(&settings_map)
        .map_err(|e| format!("Failed to convert settings to JSON: {}", e))?;
    
    // Save API keys to encrypted storage and set marker in JSON
    save_api_keys_to_encrypted(app, &mut settings_json, "")?;
    
    // Now convert to Settings struct for validation and proper serialization
    let provider = settings_map
        .get("provider")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "ollama".to_string());
    
    let provider_params_value = settings_json
        .get("provider-params")
        .cloned()
        .unwrap_or_else(|| serde_json::to_value(crate::models::ProviderParams::default()).unwrap());
    
    let provider_params: crate::models::ProviderParams = serde_json::from_value(provider_params_value)
        .map_err(|e| format!("Failed to parse provider-params: {}", e))?;
    
    let conversation_history = settings_map
        .get("conversation-history")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    
    let keyboard_shortcut = settings_map
        .get("keyboard-shortcut")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| default_shortcut());
    
    let setup_completed = settings_map
        .get("setup-completed")
        .and_then(|v| v.as_bool())
        .unwrap_or_else(|| crate::models::default_setup_completed());
    
    let settings = Settings {
        provider,
        provider_params,
        conversation_history,
        keyboard_shortcut,
        setup_completed,
        window_x: None,
        window_y: None,
    };
    
    // Ensure the directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    // Serialize to JSON (this will use the serde rename attributes)
    // API keys will be "__NOT_STORED_HERE__" markers in the JSON
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    // Write to file
    fs::write(&config_path, json)
        .map_err(|e| format!("Failed to write config.json: {}", e))?;
    
    Ok(())
}

