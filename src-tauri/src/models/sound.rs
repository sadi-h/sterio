use rodio::Source;
use serde::{Deserialize, Serialize};
use std::{io::BufReader, path::PathBuf, time::Duration};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Sound {
    pub name: String,
    pub len: Duration,
    file: PathBuf,
}

impl Sound {
    pub fn new(file: PathBuf) -> Sound {
        let name = file
            .as_path()
            .file_name()
            .map(|s| s.to_str().unwrap().replace(".mp3", ""))
            .unwrap();

        let mut song = Self {
            name,
            len: Default::default(),
            file,
        };
        let duration = song.stream().unwrap().total_duration();
        match duration {
            Some(dur) => song.len = dur,
            _ => song.len = Duration::new(0, 0),
        }
        song
    }

    pub fn stream(&self) -> super::Response<rodio::Decoder<BufReader<std::fs::File>>> {
        let file = std::fs::File::open(&self.file)?;
        let source = rodio::Decoder::new(BufReader::new(file))?;
        Ok(source)
    }
}

impl std::fmt::Display for Sound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.len.as_secs_f32(), self.name)
    }
}
