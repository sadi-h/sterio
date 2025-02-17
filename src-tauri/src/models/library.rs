use std::{sync::mpsc, time::Duration};

use super::{action::Action, que::Queue, song::Song, Response};

pub struct Library {
    player: rodio::Sink,
    //drop stream to end sound
    stream: Option<rodio::OutputStream>,
    // current song playing
    //TODO: impl queue as option to allow playing random songs not in a queue. allowing re-use of sink
    queue: Queue,
}

impl std::fmt::Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.queue)
    }
}

impl Library {
    pub fn init() -> (mpsc::Sender<Action>, mpsc::Receiver<String>) {
        //action sender and receivers
        let (sender, receiver) = mpsc::channel::<Action>();
        // player thread senders and receivers -- communication ouside the thread
        let (tx, rx) = mpsc::channel::<String>();

        let _ = std::thread::spawn({
            let tx = tx.clone();
            move || {
                let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
                let player = rodio::Sink::try_new(&stream_handle).unwrap();

                let mut library = Library {
                    player,
                    stream: Some(stream),
                    queue: Default::default(),
                };
                while let Ok(action) = receiver.recv() {
                    if library.stream.is_none() {
                        return;
                    };

                    match action {
                        Action::Stop => {
                            library.player.stop();
                        }
                        Action::Quit => {
                            library.player.stop();
                            std::mem::drop(library.stream.take());
                            tx.send("quit".to_string()).unwrap();
                        }
                        Action::Play => library.player.play(),
                        Action::Next => match library.forward() {
                            Ok(_) => {
                                println!("\nPlaying: {:?}", library.current())
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        },
                        Action::Back => match library.back() {
                            Ok(_) => println!("\nPlaying: {:?} ", library.current()),
                            Err(e) => eprintln!("Error: {}", e),
                        },
                        Action::Pause => library.player.pause(),
                        Action::Position => {
                            let pos = library.player.get_pos();
                            tx.send(format!("{:?}", pos)).unwrap();
                        }
                        Action::Empty => {
                            let pos = library.player.empty();
                            tx.send(format!("{:?}", pos)).unwrap();
                        }
                        Action::Select(position) => match library.select(position) {
                            Ok(_) => {
                                if library.current().is_some() {
                                    tx.send(library.current().unwrap().to_string()).unwrap();
                                };
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        },
                        Action::Load(path) => match library.queue.load(path) {
                            Ok(_queue) => {
                                tx.send(library.to_string()).unwrap();
                                println!("{}", library);
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        },
                        Action::Seek(position) => match library.seek(position) {
                            Ok(_) => {}
                            Err(e) => eprintln!("Error: {}", e),
                        },
                        Action::Speed(speed) => {
                            let val = library.speed(speed);
                            tx.send(format!("{:?}", val)).unwrap();
                        }
                        Action::Volume(vol) => {
                            let val = library.volume(vol);
                            tx.send(format!("{:?}", val)).unwrap();
                        }
                    }
                }
            }
        });
        (sender, rx)
    }

    pub fn current(&self) -> Option<Song> {
        // unwrap is safe, because of check above
        let current = self.queue.current?;
        self.queue
            .queue
            .get(current)
            .map(|(_, sound)| Song::new(sound.name.clone(), sound.len))
    }
    pub fn songs(&self) -> String {
        self.queue.to_string()
    }
    pub fn forward(&mut self) -> Response<()> {
        if self.queue.queue.is_empty() {
            Err("Queue is empty")?;
        }
        let len = self.queue.queue.len();
        match self.queue.current {
            Some(position) => {
                if position == len {
                    self.select(0)?;
                    Ok(())
                } else {
                    self.select(position + 1)?;
                    Ok(())
                }
            }
            _ => Err("Nothing is playing")?,
        }
    }

    pub fn back(&mut self) -> Response<()> {
        if self.queue.queue.is_empty() {
            Err("Queue is empty")?;
        }
        let len = self.queue.queue.len();
        match self.queue.current {
            Some(position) => {
                if position == 0 {
                    self.select(len)?;
                    Ok(())
                } else {
                    self.select(position - 1)?;
                    Ok(())
                }
            }
            _ => Err("Nothing is playing")?,
        }
    }

    fn select(&mut self, position: usize) -> Response<()> {
        match self.queue.sound(position) {
            Some(sound) => {
                self.player.stop();
                self.player.append(sound.stream()?);
                self.queue.current.replace(position);
            }
            _ => return Err("No songs loaded in queue")?,
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
