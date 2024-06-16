#![allow(warnings)]

#[macro_use]
extern crate lazy_static;

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
mod process;
mod process_quest;
mod process_fight;
mod log;
mod map;


fn main() {

    let server = Server::new();
    let listener = server.start("127.0.0.1", "7878");
    let sessions = server.sessions.clone();
    
    server.listen(listener, sessions);

    println!("Hello, world!");
}
