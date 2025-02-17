use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    // thread,
};

use super::{action::Action, library::Library};

pub struct Player {
    // send actions to player thread
    pub action_sender: Arc<Mutex<Sender<Action>>>,
    // receive responses from player
    pub stat_receiver: Arc<Mutex<Receiver<String>>>,
    // handle to player thread
    // handle: thread::JoinHandle<()>,
}

impl Player {
    pub fn new() -> Player {
        let (action_sender, stat_receiver /*handle */) = Library::init();
        Self {
            action_sender: Arc::new(Mutex::new(action_sender)),
            stat_receiver: Arc::new(Mutex::new(stat_receiver)),
            // handle,
        }
    }
}
