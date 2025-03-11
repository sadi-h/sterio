use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

use id3::TagLike;
use rodio::{Decoder, Source};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cover {
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub cover: Option<Cover>,
    pub tracks: Option<u32>,
    pub year: Option<i32>,
}

impl Album {
    pub fn new(
        title: Option<String>,
        artist: Option<String>,
        cover: Option<Cover>,
        tracks: Option<u32>,
        year: Option<i32>,
    ) -> Album {
        Self {
            title,
            artist,
            cover,
            tracks,
            year,
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Track {
    artist: Option<String>,
    title: Option<String>,
    len: Option<Duration>,
    album: Option<Album>,
    genre: Option<String>,
    track: Option<u32>,
    pub source: PathBuf,
}

impl Track {
    pub fn new(source: PathBuf) -> Result<Track, String> {
        Track::try_from(source)
    }
    pub fn stream(&self) -> Result<Decoder<BufReader<File>>, String> {
        let file = std::fs::File::open(&self.source).map_err(|e| e.to_string())?;
        let source = rodio::Decoder::new(BufReader::new(file)).map_err(|e| e.to_string())?;
        Ok(source)
    }
}

impl TryFrom<PathBuf> for Track {
    //TODO: error handling and remove unwrap [anyhow]
    type Error = String;
    // add code here
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(&value).map_err(|e| e.to_string())?;
        let source = rodio::Decoder::new(BufReader::new(file)).map_err(|e| e.to_string())?;
        let tags = id3::Tag::read_from_path(&value).unwrap_or_default();

        // --- album info
        let album_title = tags.album().map(String::from);
        let album_artist = tags.album_artist().map(String::from);
        let album_num_tracks = tags.total_tracks();
        let album_picture = tags.pictures().nth(0).cloned().map(|p| Cover {
            mime_type: p.mime_type,
            data: STANDARD.encode(p.data),
        });

        // --- track info
        let artist = tags.artist().map(String::from);
        let title = tags.title().map(String::from).or_else(|| {
            value
                .clone()
                .as_path()
                .file_name()
                .map(|s| s.to_str().unwrap().replace(".mp3", ""))
        });
        let len = source.total_duration();
        let album_year = if let Some(year) = tags.year() {
            Some(year)
        } else if let Some(date) = tags.date_released() {
            Some(date.year)
        } else {
            tags.date_recorded().map(|date| date.year)
        };

        let genre = tags.genre().map(String::from);
        let track = tags.track();
        let album = Album::new(
            album_title,
            album_artist,
            album_picture,
            album_num_tracks,
            album_year,
        );

        Ok(Self {
            artist,
            title,
            len,
            album: Some(album),
            genre,
            track,
            source: value,
        })
    }
}
