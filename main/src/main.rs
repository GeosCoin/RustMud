#![allow(warnings)]

use clap::{Parser, Subcommand};

#[macro_use]
extern crate lazy_static;

use log::init_log;
use log::Level;
use setting_engine::Combat;
use setting_engine::PrimaryStats;
use setting_engine::XpTable;
use settings::init_settings;
use settings::load_settings;
use settings::Settings;

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
mod settings;
mod setting_maps;
mod setting_engine;
mod utils_parsing;
mod file_parser;


/// RustMud 参数
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 日志级别 2: Info 1: Warn 0: Error
    /// 默认级别0
    #[arg(short, long)]
    #[clap(default_value = "1")]
    debug: u8,

}

fn init() {
    
    //初始化基本设置
    init_settings();
    load_settings();
}

fn main() {

    let args = Args::parse();
    
    if args.debug == 2 {
        init_log(Level::Info);
    } else if args.debug == 1 {
        init_log(Level::Warn)
    } else {
        init_log(Level::Error);
    }
    
    init();
    
    let server = Server::new();
    let listener = server.start("127.0.0.1", "7878");
    let sessions = server.sessions.clone();
    
    server.listen(listener, sessions);

    println!("Hello, world!");
}
