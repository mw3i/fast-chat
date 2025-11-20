use crate::models::Message;

// Call Custom API (placeholder - implement when needed)
pub async fn call_custom(_url: &str, _messages: &[Message]) -> Result<String, String> {
    Err("Custom API integration not yet implemented".to_string())
}

