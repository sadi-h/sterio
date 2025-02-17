use std::{fmt::Display, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Song {
    name: String,
    len: Duration,
}

impl Song {
    pub fn new(name: String, len: Duration) -> Song {
        Self { name, len }
    }
}

impl Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.name, self.len.as_secs_f32())
    }
}
