mod models;
use models::commands;
use std::sync::Mutex;
use tauri::{Manager, PhysicalPosition};

use models::player::Player;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let player = Mutex::new(Player::new());
            app.manage(player);

            #[cfg(debug_assertions)] //only include in debug mode
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            };

            let main_window = app.get_webview_window("main").unwrap();
            main_window
                .set_position(PhysicalPosition { x: 2565, y: 0 })
                .map_err(|e| e.to_string())?;
            main_window
                .set_resizable(false)
                .map_err(|e| e.to_string())?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::play,
            commands::pause,
            commands::next,
            commands::back,
            commands::load,
            commands::select,
            commands::state,
            commands::new_window,
            commands::position,
            commands::seek,
            commands::songs,
            commands::cover,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
