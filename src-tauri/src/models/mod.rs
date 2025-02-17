use std::error::Error;

pub mod action;
pub mod commands;
pub mod library;
pub mod player;
pub mod que;
pub mod song;
pub mod sound;

pub type Response<T> = Result<T, Box<dyn Error>>;
