use crate::models::Message;

// Call Google API (placeholder - implement when needed)
pub async fn call_google(_api_key: &str, _messages: &[Message]) -> Result<String, String> {
    Err("Google API integration not yet implemented".to_string())
}

