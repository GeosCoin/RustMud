
use std::{path::Path, fs::File};
use chrono::Local;

pub enum Color {
    RED,  //[1;31m
    GREEN,
    YELLOW,
    DARKBLUE,
    PINK,
    BLUE,
    GRAY,
    WHITEBGRED,
}

/// Loads a file from the given path
///
/// ### Arguments
/// * `filepath` - The path to the desired file to be loaded
pub fn load_file(filepath: &str) -> File {
    let path = Path::new(filepath);
    return File::open(path).expect("Error: failed to load file");
}

pub fn show_color(content: &str, color: Color) -> String {
    match color {
        Color::RED => {
            return "[1;31m".to_owned() + content + "[0;00m";
        },
        Color::GREEN => {
            return "[1;32m".to_owned() + content + "[0;00m";
        },
        Color::YELLOW => {
            return "[1;33m".to_owned() + content + "[0;00m";
        },
        Color::DARKBLUE => {
            return "[1;34m".to_owned() + content + "[0;00m";
        },
        Color::PINK => {
            return "[1;35m".to_owned() + content + "[0;00m";
        },
        Color::BLUE => {
            return "[1;36m".to_owned() + content + "[0;00m";
        },
        Color::GRAY => {
            return "[1;38m".to_owned() + content + "[0;00m";
        },
        Color::WHITEBGRED => {
            return "[1;41m".to_owned() + content + "[0;00m";
        }
    };
}

pub fn now() -> String {
    Local::now().format("%F %T").to_string()
}
