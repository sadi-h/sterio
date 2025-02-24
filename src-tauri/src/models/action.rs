use std::{path::PathBuf, str::FromStr, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Stop,
    State,
    Songs,
    Play,
    Next,
    Back,
    Empty,
    Pause,
    Current,
    Cover,
    Position,
    Select(usize),
    Load(Vec<PathBuf>),
    Seek(Duration),
    Speed(Option<f32>),
    Volume(Option<f32>),
}

impl Action {
    fn paths_exist(input: &str) -> bool {
        let dirs = input.split(' ').collect::<Vec<_>>();
        dirs.iter()
            .all(|dir| PathBuf::from(dir).try_exists().is_ok())
    }
    fn string_to_paths(input: &str) -> Vec<PathBuf> {
        input
            .split(' ')
            .collect::<Vec<_>>()
            .iter()
            .map(PathBuf::from)
            .collect()
    }
}

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.trim() {
            "" => Err("No action provided")?,
            "next" => Ok(Self::Next),
            "back" => Ok(Self::Back),
            "stop" => Ok(Self::Stop),
            "play" => Ok(Self::Play),
            "state" => Ok(Self::State),
            "empty" => Ok(Self::Empty),
            "pause" => Ok(Self::Pause),
            "position" => Ok(Self::Position),
            "songs" => Ok(Self::Songs),
            "current" => Ok(Self::Current),
            "Cover" => Ok(Self::Cover),
            "volume" => Ok(Self::Volume(None)),
            "speed" => Ok(Self::Speed(None)),
            n => match n.split(' ').collect::<Vec<_>>()[..] {
                ["load", directory] if Self::paths_exist(directory) => {
                    Ok(Self::Load(Self::string_to_paths(directory)))
                }

                ["select", position] => {
                    let pos = position
                        .parse::<usize>()
                        .map_err(|e| format!("Invalid position provided: {}", e))?;
                    Ok(Self::Select(pos))
                }
                ["volume", volume] => {
                    let val = volume
                        .parse::<f32>()
                        .map_err(|e| format!("Invalid Volume value: {}", e))?;
                    Ok(Self::Volume(Some(val)))
                }
                ["speed", speed] => {
                    let val = speed
                        .parse::<f32>()
                        .map_err(|e| format!("Invalid speed value: {}", e))?;
                    Ok(Self::Volume(Some(val)))
                }
                ["seek", position] => {
                    let pos = position
                        .parse::<f32>()
                        .map_err(|e| format!("Invalid seek position: {}", e))?;
                    if pos.is_sign_positive() {
                        Ok(Self::Seek(Duration::from_secs_f32(pos)))
                    } else {
                        Err("Invalid seek position. Provide positive values")?
                    }
                }
                [] => Err("Empty input arguments")?,
                [_] => Err("Invalid Action")?,
                [_, _] => Err("Invalid Action/Value")?,
                [_, _, ..] => Err("Unsupported number of arguments")?,
            },
        }
    }
}
