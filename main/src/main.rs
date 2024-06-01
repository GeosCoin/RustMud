use std::time::SystemTime;

use crate::server::Server;
use crate::channel::ServerHandler;
use tokio::{self, task, runtime::Runtime, time};
use chrono::Local;
use crossbeam::channel::{unbounded, bounded};

mod server;
mod channel;
mod player;
mod service;
mod sender;
mod timer;
fn main() {


    let run_start_time = SystemTime::now();
    let server = Server::new();
    let listener = server.start("127.0.0.1", "7878");
    let sessions = server.sessions.clone();
    
    server.listen(listener, sessions, run_start_time);

    println!("Hello, world!");
}
