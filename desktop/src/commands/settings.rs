use std::collections::HashMap;
use tauri::AppHandle;
use crate::storage::settings::{load_settings as load_settings_storage, save_settings as save_settings_storage};
use crate::window::shortcuts::update_shortcut;

#[tauri::command]
pub fn load_settings(app: AppHandle) -> Result<HashMap<String, serde_json::Value>, String> {
    load_settings_storage(&app)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings_map: HashMap<String, serde_json::Value>) -> Result<(), String> {
    // Get the current shortcut before trying to update
    let current_settings = load_settings_storage(&app).unwrap_or_default();
    let previous_shortcut = current_settings
        .get("keyboard-shortcut")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| crate::models::default_shortcut());
    
    let keyboard_shortcut = settings_map
        .get("keyboard-shortcut")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| crate::models::default_shortcut());
    
    // Try to update the shortcut - if it fails, revert to previous and return error
    if keyboard_shortcut != previous_shortcut {
        if let Err(e) = update_shortcut(app.clone(), keyboard_shortcut.clone()) {
            // Revert to previous shortcut
            if let Err(revert_err) = update_shortcut(app.clone(), previous_shortcut.clone()) {
                eprintln!("Warning: Failed to revert shortcut: {}", revert_err);
            }
            // Return error so frontend can show alert
            return Err(format!("Failed to set keyboard shortcut: {}. Reverted to previous shortcut.", e));
        }
    }
    
    // Save settings
    save_settings_storage(&app, settings_map)?;
    
    Ok(())
}

