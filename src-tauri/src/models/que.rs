use std::{ffi::OsStr, path::Path};

use serde::{Deserialize, Serialize};

use super::{song::Track, Response};

#[derive(Default, Deserialize, Serialize)]
pub struct TrackQueue {
    pub current: Option<usize>,
    pub queue: Vec<(usize, Track)>,
}

impl TrackQueue {
    pub fn track(&self, position: usize) -> Option<&Track> {
        self.queue.get(position).map(|(_, sound)| sound)
    }
    pub fn load<P: AsRef<Path>>(&mut self, dirs: Vec<P>) -> Response<()> {
        //TODO: impl using thread. One thread to query filesytem and another to build base64 string
        //for album cover [thread::scope]
        let mut id = match self.queue.is_empty() {
            true => 0,
            _ => self.queue.len() - 1,
        };
        match id {
            0 => {
                for dir in dirs {
                    for entry in std::fs::read_dir(dir)? {
                        let entry = entry?;
                        if entry.path().is_file() {
                            let path = entry.path();
                            if path.extension().is_some()
                                && path.extension().unwrap() == OsStr::new("mp3")
                            {
                                self.queue.push((id, Track::new(entry.path())?));
                                id += 1;
                            }
                        }
                    }
                }
                Ok(())
            }
            mut len => {
                for dir in dirs {
                    for entry in std::fs::read_dir(dir)? {
                        let entry = entry?;
                        if entry.path().is_file() {
                            let path = entry.path();
                            if path.extension().is_some()
                                && path.extension().unwrap() == OsStr::new("mp3")
                            {
                                len += 1;
                                self.queue.push((len, Track::new(entry.path())?));
                            }
                        }
                    }
                }
                Ok(())
            }
        }
    }
}
impl std::fmt::Display for TrackQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.queue).unwrap())
    }
}
