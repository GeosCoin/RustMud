#![allow(warnings)]

#[macro_use]
extern crate lazy_static;

use setting_engine::Combat;

use crate::channel::Server;
use crate::channel::ServerHandler;

mod channel;
mod service;
mod sender;
mod timer;
mod combat;
mod player;
mod quest;
mod login;
mod command;
mod command_hp;
mod command_look;
mod command_fight;
mod command_walk;
mod command_climb;
mod command_quest;
mod command_x;
mod command_chat;
mod command_friend;
mod command_map;
mod process;
mod process_quest;
mod process_fight;
mod process_walk;
mod process_climb;
mod log;
mod map;
mod setting_maps;
mod setting_engine;
mod utils_parsing;
mod file_parser;

fn main() {
    let mut combat = Combat::new();
    combat.load();

    let server = Server::new();
    let listener = server.start("127.0.0.1", "7878");
    let sessions = server.sessions.clone();
    
    server.listen(listener, sessions);

    println!("Hello, world!");
}
