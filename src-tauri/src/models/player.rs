use std::sync::mpsc::Sender;

use tauri::ipc::Channel;

use super::{action::Action, library::Library};
pub type ActionSender = Sender<(Action, Channel<String>, tauri::AppHandle)>;

pub struct Player {
    // send actions to player thread
    pub sender: ActionSender,
}

impl Player {
    pub fn new() -> Player {
        let action_sender = Library::init();
        Self {
            sender: action_sender,
        }
    }
}
