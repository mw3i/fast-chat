use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, Emitter, State};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

// Helper function to toggle window visibility
pub fn toggle_window(app: &AppHandle, is_listening: &Arc<Mutex<bool>>) {
    // Don't toggle if shortcut picker is in listening mode
    if let Ok(listening) = is_listening.lock() {
        if *listening {
            return;
        }
    }
    
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap() {
            window.hide().unwrap();
        } else {
            // Center the window - this helps it appear on current space
            window.center().unwrap_or(());
            
            window.show().unwrap();
            window.set_focus().unwrap();
            
            // Emit event to focus input when window is shown
            app.emit("window-shown", ()).unwrap_or_default();
        }
    }
}

// Set shortcut picker listening mode
#[tauri::command]
pub fn set_shortcut_listening_mode(is_listening: bool, state: State<'_, Arc<Mutex<bool>>>) -> Result<(), String> {
    let mut flag = state.lock().map_err(|e| format!("Failed to lock listening flag: {}", e))?;
    *flag = is_listening;
    Ok(())
}

// Parse shortcut string (e.g., "Ctrl+Space", "Cmd+K") into Modifiers and Code
pub fn parse_shortcut(shortcut_str: &str) -> Result<(Option<Modifiers>, Code), String> {
    let parts: Vec<&str> = shortcut_str.split('+').map(|s| s.trim()).collect();
    
    if parts.is_empty() {
        return Err("Shortcut cannot be empty".to_string());
    }
    
    let mut modifiers = Modifiers::empty();
    let mut key_str = "";
    
    for part in &parts {
        let part_upper = part.to_uppercase();
        match part_upper.as_str() {
            "CTRL" | "CONTROL" => modifiers |= Modifiers::CONTROL,
            "ALT" => modifiers |= Modifiers::ALT,
            "SHIFT" => modifiers |= Modifiers::SHIFT,
            "META" | "CMD" | "COMMAND" => modifiers |= Modifiers::META,
            _ => {
                key_str = part;
            }
        }
    }
    
    if key_str.is_empty() {
        return Err("Shortcut must include a key".to_string());
    }
    
    // Map key string to Code
    let code = match key_str.to_uppercase().as_str() {
        "SPACE" => Code::Space,
        "ENTER" | "RETURN" => Code::Enter,
        "ESCAPE" | "ESC" => Code::Escape,
        "TAB" => Code::Tab,
        "BACKSPACE" => Code::Backspace,
        "DELETE" | "DEL" => Code::Delete,
        "HOME" => Code::Home,
        "END" => Code::End,
        "PAGEUP" | "PAGE_UP" => Code::PageUp,
        "PAGEDOWN" | "PAGE_DOWN" => Code::PageDown,
        "ARROWUP" | "UP" | "↑" => Code::ArrowUp,
        "ARROWDOWN" | "DOWN" | "↓" => Code::ArrowDown,
        "ARROWLEFT" | "LEFT" | "←" => Code::ArrowLeft,
        "ARROWRIGHT" | "RIGHT" | "→" => Code::ArrowRight,
        "F1" => Code::F1,
        "F2" => Code::F2,
        "F3" => Code::F3,
        "F4" => Code::F4,
        "F5" => Code::F5,
        "F6" => Code::F6,
        "F7" => Code::F7,
        "F8" => Code::F8,
        "F9" => Code::F9,
        "F10" => Code::F10,
        "F11" => Code::F11,
        "F12" => Code::F12,
        "A" => Code::KeyA,
        "B" => Code::KeyB,
        "C" => Code::KeyC,
        "D" => Code::KeyD,
        "E" => Code::KeyE,
        "F" => Code::KeyF,
        "G" => Code::KeyG,
        "H" => Code::KeyH,
        "I" => Code::KeyI,
        "J" => Code::KeyJ,
        "K" => Code::KeyK,
        "L" => Code::KeyL,
        "M" => Code::KeyM,
        "N" => Code::KeyN,
        "O" => Code::KeyO,
        "P" => Code::KeyP,
        "Q" => Code::KeyQ,
        "R" => Code::KeyR,
        "S" => Code::KeyS,
        "T" => Code::KeyT,
        "U" => Code::KeyU,
        "V" => Code::KeyV,
        "W" => Code::KeyW,
        "X" => Code::KeyX,
        "Y" => Code::KeyY,
        "Z" => Code::KeyZ,
        "0" => Code::Digit0,
        "1" => Code::Digit1,
        "2" => Code::Digit2,
        "3" => Code::Digit3,
        "4" => Code::Digit4,
        "5" => Code::Digit5,
        "6" => Code::Digit6,
        "7" => Code::Digit7,
        "8" => Code::Digit8,
        "9" => Code::Digit9,
        _ => return Err(format!("Unsupported key: {}", key_str)),
    };
    
    Ok((if modifiers.is_empty() { None } else { Some(modifiers) }, code))
}

#[tauri::command]
pub fn update_shortcut(app: AppHandle, shortcut_str: String) -> Result<(), String> {
    // Unregister all existing shortcuts first
    app.global_shortcut().unregister_all()
        .map_err(|e| format!("Failed to unregister shortcuts: {}", e))?;
    
    // Parse and register the new shortcut
    match parse_shortcut(&shortcut_str) {
        Ok((modifiers, code)) => {
            let shortcut = Shortcut::new(modifiers, code);
            app.global_shortcut().register(shortcut)
                .map_err(|e| format!("Failed to register shortcut {}: {}", shortcut_str, e))?;
            Ok(())
        }
        Err(e) => Err(format!("Invalid shortcut format: {}", e))
    }
}

