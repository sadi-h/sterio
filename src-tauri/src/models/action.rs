use std::{path::PathBuf, str::FromStr, time::Duration};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Action {
    Stop,
    Quit,
    Play,
    Next,
    Back,
    Pause,
    Position,
    Empty,
    Select(usize),
    Load(PathBuf),
    Seek(Duration),
    Speed(Option<f32>),
    Volume(Option<f32>),
}

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.trim() {
            "" => Err("No action provided")?,
            "quit" => Ok(Self::Quit),
            "next" => Ok(Self::Next),
            "back" => Ok(Self::Back),
            "stop" => Ok(Self::Stop),
            "play" => Ok(Self::Play),
            "empty" => Ok(Self::Empty),
            "pause" => Ok(Self::Pause),
            "position" => Ok(Self::Position),
            "volume" => Ok(Self::Volume(None)),
            "speed" => Ok(Self::Speed(None)),
            n => match n.split(' ').collect::<Vec<_>>()[..] {
                ["load", directory]
                    if PathBuf::from(directory)
                        .try_exists()
                        .map_err(|e| format!("Location provided does not exit: {}", e))? =>
                {
                    Ok(Self::Load(PathBuf::from(directory)))
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
