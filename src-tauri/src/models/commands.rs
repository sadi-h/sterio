use std::{path::PathBuf, sync::Mutex, time::Duration};

use tauri::{ipc::Channel, AppHandle, PhysicalPosition, State, WebviewUrl, WebviewWindowBuilder};

use super::{action::Action, player::Player};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn load(
    dirs: Vec<PathBuf>,
    app: AppHandle,
    state: State<'_, Mutex<Player>>,
    dispatcher: Channel<String>,
) -> Result<String, String> {
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
pub fn music_window(app: AppHandle) -> Result<String, String> /* Result<String, String> */ {
    let window = WebviewWindowBuilder::new(&app, "music", WebviewUrl::App("/music".into()))
        .inner_size(390f64, 500f64)
        .build()
        .map_err(|e| e.to_string())?;
    window
        .set_position(PhysicalPosition { x: 2565, y: 410 })
        .map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())?;
    window.set_title("").map_err(|e| e.to_string())?;
    //TODO: will use to remove default heading and impl custom heading
    // window.set_decorations(false).map_err(|e| e.to_string())?;

    // INFO: uncomment for devtools during dev
    /* #[cfg(debug_assertions)] //only include in debug mode
    {
        window.open_devtools();
        window.close_devtools();
    }; */
    Ok("success".to_string())
}

#[tauri::command]
pub fn video_window(app: AppHandle) -> Result<String, String> /* Result<String, String> */ {
    let window = WebviewWindowBuilder::new(&app, "video", WebviewUrl::App("/video".into()))
        .inner_size(390f64, 500f64)
        .build()
        .map_err(|e| e.to_string())?;
    window
        .set_position(PhysicalPosition { x: 2565, y: 410 })
        .map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())?;
    window.set_title("").map_err(|e| e.to_string())?;

    // INFO: uncomment for devtools during dev

    /* #[cfg(debug_assertions)] //only include in debug mode
    {
        window.open_devtools();
        window.close_devtools();
    }; */
    Ok("success".to_string())
}
