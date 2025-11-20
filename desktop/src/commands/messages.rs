use tauri::{AppHandle, Emitter};
use crate::models::{Conversation, Message};
use crate::storage::conversations::{
    load_conversation as load_conversation_storage,
    save_conversation as save_conversation_storage,
    get_iso_timestamp,
};
use crate::api::providers::{call_llm, stream_llm};

#[tauri::command]
pub async fn send_message(app: AppHandle, conversation_id: String, user_message: String) -> Result<Conversation, String> {
    // Load the conversation
    let mut conversation = load_conversation_storage(&app, &conversation_id)?;
    
    // Check if the last message is already the user message we're trying to send
    // This happens when creating a new conversation (message already added in create_conversation)
    let message_already_added = conversation.messages
        .last()
        .map(|m| m.role == "user" && m.content == user_message)
        .unwrap_or(false);
    
    // Only add user message if it's not already the last message
    if !message_already_added {
        let user_timestamp = get_iso_timestamp();
        let user_msg = Message {
            role: "user".to_string(),
            content: user_message,
            timestamp: user_timestamp,
        };
        conversation.messages.push(user_msg);
    }
    
    // Prepare messages for LLM (extract role and content, exclude timestamp)
    let llm_messages: Vec<Message> = conversation.messages
        .iter()
        .map(|m| Message {
            role: m.role.clone(),
            content: m.content.clone(),
            timestamp: String::new(), // Not needed for LLM call
        })
        .collect();
    
    // Call LLM
    let llm_response = call_llm(&app, &llm_messages).await?;
    
    // Add assistant message
    let assistant_timestamp = get_iso_timestamp();
    let assistant_msg = Message {
        role: "assistant".to_string(),
        content: llm_response,
        timestamp: assistant_timestamp,
    };
    conversation.messages.push(assistant_msg);
    
    // Update updated_at timestamp
    conversation.updated_at = get_iso_timestamp();
    
    // Save the updated conversation
    save_conversation_storage(&app, &conversation)?;
    
    Ok(conversation)
}

#[tauri::command]
pub async fn send_message_stream(
    app: AppHandle,
    conversation_id: String,
    user_message: String,
) -> Result<(), String> {
    // Load the conversation
    let mut conversation = load_conversation_storage(&app, &conversation_id)?;
    
    // Check if the last message is already the user message we're trying to send
    let message_already_added = conversation.messages
        .last()
        .map(|m| m.role == "user" && m.content == user_message)
        .unwrap_or(false);
    
    // Only add user message if it's not already the last message
    if !message_already_added {
        let user_timestamp = get_iso_timestamp();
        let user_msg = Message {
            role: "user".to_string(),
            content: user_message,
            timestamp: user_timestamp,
        };
        conversation.messages.push(user_msg);
    }
    
    // Prepare messages for LLM
    let llm_messages: Vec<Message> = conversation.messages
        .iter()
        .map(|m| Message {
            role: m.role.clone(),
            content: m.content.clone(),
            timestamp: String::new(),
        })
        .collect();
    
    // Stream LLM response
    let mut full_response = String::new();
    let event_name = format!("stream-chunk-{}", conversation_id);
    
    stream_llm(&app, &event_name, &llm_messages, &mut full_response).await?;
    
    // Send completion event
    app.emit(&event_name, "DONE").map_err(|e| format!("Failed to emit completion: {}", e))?;
    
    // Add assistant message
    let assistant_timestamp = get_iso_timestamp();
    let assistant_msg = Message {
        role: "assistant".to_string(),
        content: full_response,
        timestamp: assistant_timestamp,
    };
    conversation.messages.push(assistant_msg);
    
    // Update updated_at timestamp
    conversation.updated_at = get_iso_timestamp();
    
    // Save the updated conversation
    save_conversation_storage(&app, &conversation)?;
    
    Ok(())
}

