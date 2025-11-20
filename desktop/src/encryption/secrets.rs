use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

pub fn get_secrets_path(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("Failed to get app data directory")
        .join("secrets.encrypted")
}

// Derive encryption key from system information
fn derive_encryption_key(app: &AppHandle) -> Result<Key<Aes256Gcm>, String> {
    // Use app data directory path as a stable system identifier
    let app_data_dir = app.path()
        .app_data_dir()
        .expect("Failed to get app data directory");
    
    // Create a stable key from the app data directory path
    // This ensures the same key is derived on the same machine
    let key_material = format!("ai-launcher-secret-key:{}", app_data_dir.display());
    
    // Hash to get 32 bytes for AES-256
    let mut hasher = Sha256::new();
    hasher.update(key_material.as_bytes());
    let hash = hasher.finalize();
    
    // Convert to Key
    let key_bytes: [u8; 32] = hash.into();
    Ok(*Key::<Aes256Gcm>::from_slice(&key_bytes))
}

// Encrypt data
fn encrypt_data(data: &str, key: &Key<Aes256Gcm>) -> Result<String, String> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    let ciphertext = cipher.encrypt(&nonce, data.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    // Combine nonce and ciphertext, then base64 encode
    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);
    
    Ok(general_purpose::STANDARD.encode(&combined))
}

// Decrypt data
fn decrypt_data(encrypted_data: &str, key: &Key<Aes256Gcm>) -> Result<String, String> {
    let combined = general_purpose::STANDARD.decode(encrypted_data)
        .map_err(|e| format!("Base64 decode failed: {}", e))?;
    
    if combined.len() < 12 {
        return Err("Invalid encrypted data".to_string());
    }
    
    // Extract nonce (first 12 bytes) and ciphertext (rest)
    let nonce = Nonce::from_slice(&combined[..12]);
    let ciphertext = &combined[12..];
    
    let cipher = Aes256Gcm::new(key);
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;
    
    String::from_utf8(plaintext)
        .map_err(|e| format!("UTF-8 decode failed: {}", e))
}

// Load secrets from encrypted file
pub fn load_secrets(app: &AppHandle) -> Result<HashMap<String, String>, String> {
    let secrets_path = get_secrets_path(app);
    
    if !secrets_path.exists() {
        return Ok(HashMap::new());
    }
    
    let encrypted_data = fs::read_to_string(&secrets_path)
        .map_err(|e| format!("Failed to read secrets file: {}", e))?;
    
    let key = derive_encryption_key(app)?;
    let decrypted_data = decrypt_data(&encrypted_data, &key)?;
    
    let secrets: HashMap<String, String> = serde_json::from_str(&decrypted_data)
        .map_err(|e| format!("Failed to parse secrets JSON: {}", e))?;
    
    Ok(secrets)
}

// Save secrets to encrypted file
pub fn save_secrets(app: &AppHandle, secrets: &HashMap<String, String>) -> Result<(), String> {
    let secrets_path = get_secrets_path(app);
    
    // Ensure directory exists
    if let Some(parent) = secrets_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create secrets directory: {}", e))?;
    }
    
    let json_data = serde_json::to_string(secrets)
        .map_err(|e| format!("Failed to serialize secrets: {}", e))?;
    
    let key = derive_encryption_key(app)?;
    let encrypted_data = encrypt_data(&json_data, &key)?;
    
    fs::write(&secrets_path, encrypted_data)
        .map_err(|e| format!("Failed to write secrets file: {}", e))?;
    
    Ok(())
}

// Get API key from encrypted storage
pub fn get_api_key_from_encrypted(app: &AppHandle, path: &str) -> Result<String, String> {
    let secrets = load_secrets(app)?;
    Ok(secrets.get(path).cloned().unwrap_or_default())
}

// Store API key in encrypted storage
pub fn store_api_key_in_encrypted(app: &AppHandle, path: &str, value: &str) -> Result<(), String> {
    if value.is_empty() {
        // Remove from secrets if empty
        let mut secrets = load_secrets(app)?;
        secrets.remove(path);
        save_secrets(app, &secrets)?;
        return Ok(());
    }
    
    let mut secrets = load_secrets(app)?;
    secrets.insert(path.to_string(), value.to_string());
    save_secrets(app, &secrets)?;
    
    Ok(())
}

// Recursively process JSON to load API keys from encrypted storage
pub fn load_api_keys_from_encrypted(app: &AppHandle, value: &mut serde_json::Value, path_prefix: &str) -> Result<(), String> {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map.iter_mut() {
                let current_path = if path_prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path_prefix, key)
                };
                
                // Check if this is an API key field (ends with _key)
                if key.ends_with("_key") {
                    if let serde_json::Value::String(s) = val {
                        // If marker, try to load from encrypted storage
                        if s == "__NOT_STORED_HERE__" {
                            let encrypted_value = get_api_key_from_encrypted(app, &current_path)?;
                            *val = serde_json::Value::String(encrypted_value);
                        }
                    }
                } else {
                    // Recursively process nested objects
                    load_api_keys_from_encrypted(app, val, &current_path)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

// Recursively process JSON to save API keys to encrypted storage
pub fn save_api_keys_to_encrypted(app: &AppHandle, value: &mut serde_json::Value, path_prefix: &str) -> Result<(), String> {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map.iter_mut() {
                let current_path = if path_prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path_prefix, key)
                };
                
                // Check if this is an API key field (ends with _key)
                if key.ends_with("_key") {
                    if let serde_json::Value::String(s) = val {
                        if !s.is_empty() && s != "__NOT_STORED_HERE__" {
                            // Store in encrypted storage
                            store_api_key_in_encrypted(app, &current_path, s)?;
                            // Set marker in JSON
                            *val = serde_json::Value::String("__NOT_STORED_HERE__".to_string());
                        }
                    }
                } else {
                    // Recursively process nested objects
                    save_api_keys_to_encrypted(app, val, &current_path)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

