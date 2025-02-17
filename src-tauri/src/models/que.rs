use std::{ffi::OsStr, path::Path};

use super::{
    song::{self, Song},
    sound::Sound,
    Response,
};

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Queue {
    pub current: Option<usize>,
    pub queue: Vec<(usize, Sound)>,
}

impl std::fmt::Display for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.queue
                .iter()
                .map(|(i, song)| format!("{}-{}-{}", i, song.name.clone(), song.len.as_secs_f32(),))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Queue {
    fn new() -> Self {
        Default::default()
    }

    pub fn sound(&self, position: usize) -> Option<&Sound> {
        match self.queue.get(position).as_ref() {
            Some((_, sound)) => Some(sound),
            _ => None,
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, dir: P) -> Response<()> {
        let files = std::fs::read_dir(dir)?;
        if self.queue.is_empty() {
            let mut index = 0;
            for entry in files {
                let entry = entry?;
                if entry.path().is_file() {
                    let path = entry.path();
                    if path.extension().is_some() && path.extension().unwrap() == OsStr::new("mp3")
                    {
                        self.queue.push((index, Sound::new(entry.path())));
                        index += 1;
                    }
                }
            }
            return Ok(());
        } else {
            let mut len = self.queue.len() - 1;
            for entry in files {
                let entry = entry?;
                if entry.path().is_file() {
                    let path = entry.path();
                    if path.extension().is_some() && path.extension().unwrap() == OsStr::new("mp3")
                    {
                        len += 1;
                        self.queue.push((len, Sound::new(entry.path())));
                    }
                }
            }
            return Ok(());
        }
    }
}
