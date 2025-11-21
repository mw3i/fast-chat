use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use chrono::{DateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{Conversation, ConversationMetadata, Message};

pub fn get_conversations_dir(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("Failed to get app data directory")
        .join("conversations")
}

// Validate conversation ID to prevent path traversal attacks
pub fn validate_conversation_id(id: &str) -> Result<(), String> {
    // Only allow alphanumeric characters, hyphens, and underscores
    // This prevents path traversal (../) and other malicious patterns
    if id.is_empty() {
        return Err("Conversation ID cannot be empty".to_string());
    }
    
    if id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        Ok(())
    } else {
        Err(format!("Invalid conversation ID: contains invalid characters"))
    }
}

pub fn save_conversation(app: &AppHandle, conversation: &Conversation) -> Result<(), String> {
    let conversations_dir = get_conversations_dir(app);
    
    // Ensure the conversations directory exists
    fs::create_dir_all(&conversations_dir)
        .map_err(|e| format!("Failed to create conversations directory: {}", e))?;
    
    // Create file path: conversations/{id}.json
    let file_path = conversations_dir.join(format!("{}.json", conversation.id));
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(conversation)
        .map_err(|e| format!("Failed to serialize conversation: {}", e))?;
    
    // Write to file
    fs::write(&file_path, json)
        .map_err(|e| format!("Failed to write conversation file: {}", e))?;
    
    Ok(())
}

pub fn load_conversation(app: &AppHandle, conversation_id: &str) -> Result<Conversation, String> {
    // Validate conversation ID to prevent path traversal
    validate_conversation_id(conversation_id)?;
    
    let conversations_dir = get_conversations_dir(app);
    let file_path = conversations_dir.join(format!("{}.json", conversation_id));
    
    if !file_path.exists() {
        return Err(format!("Conversation {} not found", conversation_id));
    }
    
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read conversation file: {}", e))?;
    
    let conversation: Conversation = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse conversation JSON: {}", e))?;
    
    Ok(conversation)
}

pub fn list_conversations(app: &AppHandle) -> Result<Vec<Conversation>, String> {
    let conversations_dir = get_conversations_dir(app);
    
    // Ensure directory exists
    if !conversations_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut conversations = Vec::new();
    
    // Read all JSON files in the conversations directory
    let entries = fs::read_dir(&conversations_dir)
        .map_err(|e| format!("Failed to read conversations directory: {}", e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        // Only process .json files
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    if let Ok(conversation) = serde_json::from_str::<Conversation>(&content) {
                        conversations.push(conversation);
                    }
                }
                Err(_) => {
                    // Skip files that can't be read
                    continue;
                }
            }
        }
    }
    
    // Sort by updated_at (most recent first)
    conversations.sort_by(|a, b| {
        b.updated_at.cmp(&a.updated_at)
    });
    
    Ok(conversations)
}

pub fn delete_conversation(app: &AppHandle, conversation_id: &str) -> Result<(), String> {
    // Validate conversation ID to prevent path traversal
    validate_conversation_id(conversation_id)?;
    
    let conversations_dir = get_conversations_dir(app);
    let file_path = conversations_dir.join(format!("{}.json", conversation_id));
    
    if !file_path.exists() {
        return Err(format!("Conversation {} not found", conversation_id));
    }
    
    fs::remove_file(&file_path)
        .map_err(|e| format!("Failed to delete conversation file: {}", e))?;
    
    Ok(())
}

pub fn delete_all_conversations(app: &AppHandle) -> Result<(), String> {
    let conversations_dir = get_conversations_dir(app);
    
    if !conversations_dir.exists() {
        return Ok(()); // Nothing to delete
    }
    
    // Read all JSON files in the conversations directory and delete them
    let entries = fs::read_dir(&conversations_dir)
        .map_err(|e| format!("Failed to read conversations directory: {}", e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        // Only delete .json files
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            fs::remove_file(&path)
                .map_err(|e| format!("Failed to delete conversation file {}: {}", path.display(), e))?;
        }
    }
    
    Ok(())
}

pub fn get_conversation_history(app: &AppHandle) -> Result<Vec<ConversationMetadata>, String> {
    let conversations = list_conversations(app)?;
    
    let metadata: Vec<ConversationMetadata> = conversations
        .into_iter()
        .map(|conv| {
            // Get last message content (or empty string if no messages)
            let last_message = conv.messages
                .last()
                .map(|m| {
                    // Truncate to first 100 chars for preview
                    let content = &m.content;
                    if content.len() > 100 {
                        format!("{}...", &content[..100])
                    } else {
                        content.clone()
                    }
                })
                .unwrap_or_default();
            
            ConversationMetadata {
                id: conv.id,
                title: conv.title,
                last_message,
                timestamp: conv.updated_at,
                message_count: conv.messages.len(),
            }
        })
        .collect();
    
    Ok(metadata)
}

pub fn get_iso_timestamp() -> String {
    // Generate ISO 8601 timestamp in format: YYYY-MM-DDTHH:MM:SSZ
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn generate_conversation_id() -> String {
    // Generate a unique ID using timestamp with nanoseconds for uniqueness
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    let nanos = now.as_nanos();
    format!("conv-{}", nanos)
}

pub fn generate_title_from_message(message: &str) -> String {
    // Create title from first message (truncate to 50 chars)
    let trimmed = message.trim();
    if trimmed.len() > 50 {
        format!("{}...", &trimmed[..50])
    } else {
        trimmed.to_string()
    }
}

pub fn create_conversation(app: &AppHandle, user_message: &str) -> Result<String, String> {
    let conversations_dir = get_conversations_dir(app);
    
    // Ensure the conversations directory exists
    fs::create_dir_all(&conversations_dir)
        .map_err(|e| format!("Failed to create conversations directory: {}", e))?;
    
    // Generate conversation ID
    let conversation_id = generate_conversation_id();
    let iso_timestamp = get_iso_timestamp();
    
    // Generate title from first message
    let title = generate_title_from_message(user_message);
    
    // Create the first message
    let first_message = Message {
        role: "user".to_string(),
        content: user_message.to_string(),
        timestamp: iso_timestamp.clone(),
        complete: true,
    };
    
    // Create the conversation
    let conversation = Conversation {
        id: conversation_id.clone(),
        title,
        created_at: iso_timestamp.clone(),
        updated_at: iso_timestamp,
        messages: vec![first_message],
    };
    
    // Save the conversation
    save_conversation(app, &conversation)?;
    
    Ok(conversation_id)
}

