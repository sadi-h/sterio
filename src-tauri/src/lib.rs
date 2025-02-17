mod models;
use models::commands;
use tauri::Manager;

use models::player::Player;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let player = Player::new();
            app.manage(player);
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
