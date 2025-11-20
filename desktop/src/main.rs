mod models;
mod encryption;
mod storage;
mod api;
mod commands;
mod window;

use std::sync::{Arc, Mutex};
use tauri::{Manager, RunEvent, WindowEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutEvent, ShortcutState};

use window::shortcuts::{toggle_window, parse_shortcut, set_shortcut_listening_mode, update_shortcut};
use storage::settings::load_settings as load_settings_storage;
use commands::{
    load_settings,
    save_settings,
    save_conversation,
    load_conversation,
    list_conversations,
    delete_conversation,
    delete_all_conversations,
    get_conversation_history,
    create_conversation,
    send_message,
    send_message_stream,
};

fn main() {
    // Shared state to track if shortcut picker is in listening mode
    let is_listening = Arc::new(Mutex::new(false));
    let is_listening_clone = Arc::clone(&is_listening);
    let is_listening_for_run = Arc::clone(&is_listening);
    
    tauri::Builder::default()
        .manage(is_listening)
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app: &tauri::AppHandle, _shortcut: &Shortcut, event: ShortcutEvent| {
                    if event.state() == ShortcutState::Pressed {
                        toggle_window(app, &is_listening_clone);
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,
            save_conversation,
            load_conversation,
            list_conversations,
            delete_conversation,
            delete_all_conversations,
            get_conversation_history,
            create_conversation,
            send_message,
            send_message_stream,
            update_shortcut,
            set_shortcut_listening_mode
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let app_handle = app.handle();
            
            // Start with window hidden
            window.hide().unwrap();
            
            // Load settings and register keyboard shortcut
            let settings_result = load_settings_storage(&app_handle);
            let shortcut_str = if let Ok(settings) = settings_result {
                settings.get("keyboard-shortcut")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "Ctrl+Space".to_string())
            } else {
                "Ctrl+Space".to_string()
            };
            
            // Parse and register the shortcut
            if let Ok((modifiers, code)) = parse_shortcut(&shortcut_str) {
                let shortcut = Shortcut::new(modifiers, code);
                if let Err(e) = app_handle.global_shortcut().register(shortcut) {
                    eprintln!("Failed to register shortcut {}: {}", shortcut_str, e);
                    // Fallback to default
                    let default_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);
                    app_handle.global_shortcut().register(default_shortcut).unwrap();
                }
            } else {
                // Fallback to default if parsing fails
                eprintln!("Failed to parse shortcut: {}", shortcut_str);
                let default_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);
                app_handle.global_shortcut().register(default_shortcut).unwrap();
            }
            
            // Handle window close event - hide instead of closing on macOS (and Windows)
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    // Prevent the window from closing
                    api.prevent_close();
                    // Hide the window instead
                    window_clone.hide().unwrap_or(());
                }
            });
            
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(move |app_handle, event| {
            match event {
                RunEvent::Reopen { .. } => {
                    // Handle dock icon click - toggle window visibility
                    toggle_window(&app_handle, &is_listening_for_run);
                }
                _ => {}
            }
        });
}
