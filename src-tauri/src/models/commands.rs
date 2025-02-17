use tauri::State;

use super::{action::Action, player::Player};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn load(path: &str, state: State<'_, Player>) -> Result<String, String> {
    state
        .action_sender
        .lock()
        .map_err(|e| e.to_string())?
        .send(Action::Load(path.into()))
        .map_err(|e| e.to_string())?;
    Ok(format!("Loaded songs in:, {}", path))
}

#[tauri::command]
pub fn select(song: usize, state: State<'_, Player>) -> Result<String, String> {
    state
        .action_sender
        .lock()
        .map_err(|e| e.to_string())?
        .send(Action::Select(song))
        .map_err(|e| e.to_string())?;
    Ok(format!("Playing song:, {}", song))
}

#[tauri::command]
pub fn next(state: State<'_, Player>) -> Result<String, String> {
    state
        .action_sender
        .lock()
        .map_err(|e| e.to_string())?
        .send(Action::Next)
        .map_err(|e| e.to_string())?;
    Ok("Playing next song".to_string())
}

#[tauri::command]
pub fn back(state: State<'_, Player>) -> Result<String, String> {
    state
        .action_sender
        .lock()
        .map_err(|e| e.to_string())?
        .send(Action::Back)
        .map_err(|e| e.to_string())?;
    Ok("Playing previous song".to_string())
}

#[tauri::command]
pub fn pause(state: State<'_, Player>) -> Result<String, String> {
    state
        .action_sender
        .lock()
        .map_err(|e| e.to_string())?
        .send(Action::Pause)
        .map_err(|e| e.to_string())?;
    Ok("Player has stopped".to_string())
}

#[tauri::command]
pub fn play(state: State<'_, Player>) -> Result<String, String> {
    state
        .action_sender
        .lock()
        .map_err(|e| e.to_string())?
        .send(Action::Play)
        .map_err(|e| e.to_string())?;
    Ok("Player has stopped".to_string())
}
#[tauri::command]
pub fn ld(path: &str, state: State<'_, Player>) -> Result<String, String> {
    state
        .action_sender
        .lock()
        .map_err(|e| e.to_string())?
        .send(Action::Load(path.into()))
        .map_err(|e| e.to_string())?;
    Ok(format!("Loaded songs in:, {}", path))
}
