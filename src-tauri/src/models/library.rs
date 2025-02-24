use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::{sync::mpsc, time::Duration};
use tauri::{ipc::Channel, AppHandle, Emitter};

use super::{
    action::Action,
    que::TrackQueue,
    song::{Cover, Track},
    Response,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Current {
    pub song: Track,
    pub paused: bool,
}

impl Current {
    fn new(song: Track, paused: bool) -> Self {
        Self { song, paused }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub current: Option<Current>,
    pub next: Option<Track>,
    pub prev: Option<Track>,
}

impl State {
    fn new(current: Option<Current>, next: Option<Track>, prev: Option<Track>) -> State {
        Self {
            current,
            next,
            prev,
        }
    }
}

pub struct Library {
    player: rodio::Sink,
    //drop stream to end sound
    stream: Option<rodio::OutputStream>,
    // current song playing
    //TODO: impl queue as option to allow playing random songs not in a queue. allowing re-use of sink

    // queue: Queue,
    queue: TrackQueue,
}

impl std::fmt::Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.queue)
    }
}

impl Library {
    pub fn init() -> mpsc::Sender<(Action, Channel<String>, AppHandle)> {
        //action sender and receivers
        let (sender, receiver) = mpsc::channel::<(Action, Channel<String>, AppHandle)>();

        let _ = std::thread::spawn({
            move || {
                let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
                let player = rodio::Sink::try_new(&stream_handle).unwrap();

                let mut library = Library {
                    player,
                    stream: Some(stream),
                    queue: Default::default(),
                };
                while let Ok((action, dispatcher, emitter)) = receiver.recv() {
                    if library.stream.is_none() {
                        return;
                    };

                    match action {
                        Action::Stop => {
                            library.player.stop();
                            emitter.emit("stop", library.current()).unwrap();
                        }
                        Action::Play => {
                            library.player.play();
                            emitter.emit("play", false).unwrap();
                        }
                        Action::State => {
                            let state = library.state();
                            dispatcher.send(json::to_string(&state).unwrap()).unwrap();
                        }
                        Action::Next => {
                            if library.forward().is_ok() {
                                emitter.emit("next", library.current()).unwrap();
                            }
                        }
                        Action::Back => {
                            if library.back().is_ok() {
                                emitter.emit("back", library.current()).unwrap();
                            }
                        }
                        Action::Pause => {
                            library.player.pause();
                            emitter.emit("pause", true).unwrap();
                        }
                        Action::Current => match library.current() {
                            Some(curr) => dispatcher.send(json::to_string(&curr).unwrap()).unwrap(),
                            _ => dispatcher.send("{current: None}".to_string()).unwrap(),
                        },
                        Action::Cover => match library.cover() {
                            Some(cover) => {
                                dispatcher.send(cover.data).unwrap();
                            }
                            _ => dispatcher
                                .send(json::to_string(&None::<String>).unwrap())
                                .unwrap(),
                        },
                        Action::Position => {
                            let pos = library.player.get_pos();
                            let _ = emitter.emit("position", pos);
                        }
                        Action::Empty => {
                            // return true when sink has no more sounds. can be used to impl auto play next/ repeat
                            let empty = library.player.empty();
                            let _ = emitter.emit("empty", empty);
                        }
                        Action::Songs => dispatcher
                            .send(json::to_string(&library.songs()).unwrap())
                            .unwrap(),
                        Action::Select(position) => {
                            if library.select(position).is_ok() {
                                emitter.emit("select", library.current()).unwrap();
                            }
                        }
                        Action::Load(dirs) => match library.queue.load(dirs) {
                            Ok(_) => {
                                dispatcher
                                    .send(json::to_string(&library.songs()).unwrap())
                                    .unwrap();
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        },
                        Action::Seek(position) => match library.seek(position) {
                            Ok(_) => {
                                emitter.emit("seek", position).unwrap();
                                dispatcher
                                    .send(json::to_string(&position).unwrap())
                                    .unwrap();
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        },
                        Action::Speed(speed) => {
                            let val = library.speed(speed);
                            emitter.emit("speed", val).unwrap();
                        }
                        Action::Volume(vol) => {
                            let val = library.volume(vol);
                            emitter.emit("volume", val).unwrap();
                        }
                    }
                }
            }
        });
        sender
    }

    pub fn current(&self) -> Option<Current> {
        self.queue.current.and_then(|p| {
            self.queue.queue.get(p).map(|(_, sound)| {
                let paused = self.player.is_paused();
                Current::new(sound.clone(), paused)
            })
        })
    }

    pub fn cover(&self) -> Option<Cover> {
        self.current().and_then(|current| {
            id3::Tag::read_from_path(&current.song.source)
                .unwrap_or_default()
                .pictures()
                .nth(0)
                .cloned()
                .map(|p| Cover {
                    mime_type: p.mime_type,
                    data: STANDARD.encode(p.data),
                })
        })
    }

    pub fn state(&self) -> State {
        let len = if self.queue.queue.is_empty() {
            0
        } else {
            self.queue.queue.len() - 1
        };
        let current = self
            .queue
            .current
            .and_then(|s| self.queue.queue.get(s))
            .map(|(_, track)| {
                let paused = self.player.is_paused();
                Current::new(track.clone(), paused)
            });
        let next = self
            .queue
            .current
            .and_then(|s| self.queue.queue.get(if s == len { 0 } else { s + 1 }))
            .map(|(_, sound)| sound.clone());
        let prev = self
            .queue
            .current
            .and_then(|s| self.queue.queue.get(if s == 0 { len } else { s - 1 }))
            .map(|(_, sound)| sound.clone());

        State::new(current, next, prev)
    }

    pub fn songs(&self) -> Vec<(usize, Track)> {
        self.queue
            .queue
            .iter()
            .map(|(i, sound)| (*i, sound.clone()))
            .collect()
    }

    pub fn forward(&mut self) -> Response<()> {
        if self.queue.queue.is_empty() {
            Err("No songs loaded for playback")?;
        }
        let len = self.queue.queue.len() - 1;
        match self.queue.current {
            Some(p) => match p == len {
                true => {
                    self.select(0)?;
                    Ok(())
                }
                _ => {
                    self.select(p + 1)?;
                    Ok(())
                }
            },
            _ => Err("No song selected for playback")?,
        }
    }

    pub fn back(&mut self) -> Response<()> {
        if self.queue.queue.is_empty() {
            Err("No songs loaded for playback")?;
        }
        let len = self.queue.queue.len() - 1;
        match self.queue.current {
            Some(p) => match p == 0 {
                true => {
                    self.select(len)?;
                    Ok(())
                }
                _ => {
                    self.select(p - 1)?;
                    Ok(())
                }
            },
            _ => Err("No song selected for playback")?,
        }
    }

    fn select(&mut self, position: usize) -> Response<()> {
        match self.queue.track(position) {
            Some(sound) => {
                self.player.stop();
                self.player.append(sound.stream()?);
                self.queue.current.replace(position);
            }
            _ => return Err("Invalid Song Range")?,
        }
        Ok(())
    }
    pub fn seek(&self, position: Duration) -> Response<()> {
        self.player.try_seek(position)?;
        Ok(())
    }

    pub fn volume(&self, value: Option<f32>) -> f32 {
        match value {
            Some(val) => {
                self.player.set_volume(val);
                val
            }
            None => self.player.volume(),
        }
    }
    pub fn speed(&self, value: Option<f32>) -> f32 {
        match value {
            Some(val) => {
                self.player.set_speed(val);
                val
            }
            None => self.player.speed(),
        }
    }
}
