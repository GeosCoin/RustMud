#![allow(warnings)]

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

fn main() {

    let server = Server::new();
    let listener = server.start("127.0.0.1", "7878");
    let sessions = server.sessions.clone();
    
    server.listen(listener, sessions);

    println!("Hello, world!");
}
