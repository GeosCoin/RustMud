#![allow(warnings)]

use std::{fs::File, io::{self, BufRead}, net::SocketAddr, path::Path, sync::atomic::{AtomicUsize, Ordering}};
use chrono::Local;

// 起：  [1;31m   尾：   [0;00m
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

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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

pub fn now_hm() -> String {
    Local::now().format("%H:%M").to_string()
}

pub fn now_mdhm() -> String {
    Local::now().format("%m-%d %H:%M").to_string()
}

pub fn get_id() -> usize {
    static COUNTER : AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed) 
}

pub fn substring_range(utf8_str: &str, start_idx: usize, end_idx: usize) -> &str {
    let start_char = start_idx;
    let end_char = end_idx;
    let mut indices = utf8_str.char_indices().map(|(i, _)| i);
    let start = indices.nth(start_char).unwrap();
    println!("start:{}", start);
    let end = indices.nth(end_char - start_char - 1).unwrap_or(utf8_str.len());
    println!("end:{}", end);
    return &utf8_str[start..end];
}

pub fn substring_start(utf8_str: &str, start_idx: usize) -> &str {    
    let start_char = start_idx;
    let end_char = utf8_str.len();
    let mut indices = utf8_str.char_indices().map(|(i, _)| i);
    let start = indices.nth(start_char).unwrap();
    println!("start:{}", start);
    let end = indices.nth(end_char - start_char - 1).unwrap_or(utf8_str.len());
    println!("end:{}", end);
    // println!("{:?}", &utf8_str[start..end]);
    return &utf8_str[start..end];
}

pub fn insert_line(utf8_str: &str, num_per_line: usize) -> String {
    let len = utf8_str.char_indices().count();
    let cnt = len / num_per_line;
    let mut i = 0;
    let mut ret_str = String::from("");

    loop {
        if i > cnt {
            break;
        }

        let cur_start = num_per_line*i;
        if cur_start >= len {
            break;
        }
        let mut cur_end = num_per_line*(i+1);
        if cur_end >= len {
            cur_end = len;
        }
        println!("cur_start: {}, cur_end: {}", cur_start, cur_end);
        let result = substring_range(utf8_str, cur_start, cur_end);
        println!("result = {}", result);

        ret_str = ret_str.to_owned() + result + "\\n";
        i += 1;
    }
    
    println!("ret_str: {:?}", ret_str);
    ret_str
}


