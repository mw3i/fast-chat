use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String, // "system", "user", or "assistant"
    pub content: String,
    pub timestamp: String, // ISO 8601 timestamp
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: String, // ISO 8601 timestamp
    pub updated_at: String, // ISO 8601 timestamp
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationMetadata {
    pub id: String,
    pub title: String,
    #[serde(rename = "lastMessage")]
    pub last_message: String,
    pub timestamp: String, // ISO 8601 timestamp (using updated_at)
    #[serde(rename = "messageCount")]
    pub message_count: usize,
}

