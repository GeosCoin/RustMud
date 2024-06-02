
use std::{fs::File, net::SocketAddr, path::Path};
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

// Use Example: 
// use serde::{Serialize, Deserialize};
// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// struct Exit {
//     name: String,
//     room: u32,
// }
// let mut w_rooms: Vec<Exit> = Vec::new();
// let rooms_file = utils::load_file("users.json");
// let mut rooms: Vec<Exit> = serde_json::from_reader(rooms_file).expect("Error: failed to read json file");
// let exist = rooms.contains(&Exit{
//     name:"lxz".to_string(),room:3234
// });
// println!("{}", exist);
// rooms[0].room = 3234;
// w_rooms = rooms;
// let w_file = utils::create_file("users.json"); 
// serde_json::to_writer(w_file, &w_rooms).unwrap();

//read only
pub fn load_file(filepath: &str) -> File {
    let path = Path::new(filepath);
    return File::open(path).expect("Error: failed to load file");
}

//write only
pub fn create_file(filepath: &str) -> File {
    let path = Path::new(filepath);
    return File::create(path).expect("Error: failed to create file");
}

//append
pub fn append_file(filepath: &str) -> File {
    let path = Path::new(filepath);
    return File::options()
        .read(true)
        .write(true)
        .open(path)
        .expect("Error: failed to append file");
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
