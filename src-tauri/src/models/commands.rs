use std::{path::PathBuf, sync::Mutex, time::Duration};

use tauri::{ipc::Channel, AppHandle, State, WebviewUrl, WebviewWindowBuilder};

use super::{action::Action, player::Player};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn load(
    dirs: Vec<PathBuf>,
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    println!("Load function called");
    let player = { state.lock().unwrap() };
    player
        .sender
        .send((Action::Load(dirs), dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}

#[tauri::command]
pub fn songs(
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    let player = { state.lock().map_err(|e| e.to_string())? };
    player
        .sender
        .send((Action::Songs, dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}

#[tauri::command]
pub fn select(
    id: usize,
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    let player = { state.lock().unwrap() };
    player
        .sender
        .send((Action::Select(id), dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}

#[tauri::command]
pub fn next(
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    let player = { state.lock().unwrap() };
    player
        .sender
        .send((Action::Next, dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}

#[tauri::command]
pub fn back(
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    let player = { state.lock().unwrap() };
    player
        .sender
        .send((Action::Back, dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}

#[tauri::command]
pub fn pause(
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    let player = { state.lock().unwrap() };
    player
        .sender
        .send((Action::Pause, dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}

#[tauri::command]
pub fn play(
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    let player = { state.lock().unwrap() };
    player
        .sender
        .send((Action::Play, dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}
#[tauri::command]
pub fn state(
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    let player = { state.lock().map_err(|e| e.to_string())? };
    player
        .sender
        .send((Action::State, dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}

#[tauri::command]
pub fn seek(
    dur: u64,
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    let player = { state.lock().map_err(|e| e.to_string())? };
    player
        .sender
        .send((Action::Seek(Duration::from_secs(dur)), dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}

#[tauri::command]
pub fn position(
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    let player = { state.lock().map_err(|e| e.to_string())? };
    player
        .sender
        .send((Action::Position, dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}

#[tauri::command]
pub fn cover(
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
    let player = { state.lock().map_err(|e| e.to_string())? };
    player
        .sender
        .send((Action::Cover, dispatcher, app))
        .map_err(|e| e.to_string())?;
    Ok("success".to_string())
}

#[tauri::command]
pub fn new_window(app: AppHandle) -> Result<String, String> /* Result<String, String> */ {
    let window = WebviewWindowBuilder::new(&app, "Library", WebviewUrl::App("/library".into()))
        .inner_size(500f64, 500f64)
        .build()
        .map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())?;
    window.set_title("").map_err(|e| e.to_string())?;
    #[cfg(debug_assertions)] //only include in debug mode
    {
        window.open_devtools();
        window.close_devtools();
    };
    Ok("success".to_string())
}
