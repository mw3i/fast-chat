use tauri::AppHandle;
use crate::models::Conversation;
use crate::storage::conversations::{
    save_conversation as save_conversation_storage,
    load_conversation as load_conversation_storage,
    list_conversations as list_conversations_storage,
    delete_conversation as delete_conversation_storage,
    delete_all_conversations as delete_all_conversations_storage,
    get_conversation_history as get_conversation_history_storage,
    create_conversation as create_conversation_storage,
};

#[tauri::command]
pub fn save_conversation(app: AppHandle, conversation: Conversation) -> Result<(), String> {
    save_conversation_storage(&app, &conversation)
}

#[tauri::command]
pub fn load_conversation(app: AppHandle, conversation_id: String) -> Result<Conversation, String> {
    load_conversation_storage(&app, &conversation_id)
}

#[tauri::command]
pub fn list_conversations(app: AppHandle) -> Result<Vec<Conversation>, String> {
    list_conversations_storage(&app)
}

#[tauri::command]
pub fn delete_conversation(app: AppHandle, conversation_id: String) -> Result<(), String> {
    delete_conversation_storage(&app, &conversation_id)
}

#[tauri::command]
pub fn delete_all_conversations(app: AppHandle) -> Result<(), String> {
    delete_all_conversations_storage(&app)
}

#[tauri::command]
pub fn get_conversation_history(app: AppHandle) -> Result<Vec<crate::models::ConversationMetadata>, String> {
    get_conversation_history_storage(&app)
}

#[tauri::command]
pub fn create_conversation(app: AppHandle, user_message: String) -> Result<String, String> {
    create_conversation_storage(&app, &user_message)
}

