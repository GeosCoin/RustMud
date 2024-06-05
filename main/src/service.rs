use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use crate::channel::wrap_message;
use crate::channel::wrap_message_ext;
use crate::channel::wrap_message_timer;
use crate::channel::Message;
use crate::channel::MessageType;
use crate::command::Invoker;
use crate::command_fight::FightCommand;
use crate::command_hp::HpCommand;
use crate::command_look::LookCommand;
use crate::command_walk::WalkCommand;
use crate::player::Player;
use crate::{channel::{ServerHandler, SessionType, Sessions, SessionContext, SessionsType}, player};
use std::collections::HashMap;
use std::fmt::Error;
use std::hash::Hash;
use std::io::Empty;
use std::net::SocketAddr;
use std::ops::Add;
use std::time::Duration;
use std::thread;
use serde::{Serialize, Deserialize};
use utils;
use std::cell::RefCell;
use std::rc::Rc;
use std::net::{SocketAddrV4, Ipv4Addr};
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Login {
    pub login_name: String,
    pub password: String,
}

impl Login {
    fn new() -> Self{
        Login {
            login_name: "".to_string(),
            password: "".to_string(),
        }
    }
}

pub struct LoginInfo {
    pub login: Login,
    pub b_login: bool,
}

impl LoginInfo {
    fn new() -> Self{
        LoginInfo {
            login: Login::new(),
            b_login: false,
        }
    }
}

pub fn _handle_service(
    s_service: Sender<String>,  //发送到socket       
    r_service: Receiver<String>,   //service接收数据
    s_combat: Sender<String>, //发送到fight模块
){
    let mut login_infos: HashMap<SocketAddr, LoginInfo> = HashMap::new();
    let mut players: HashMap<SocketAddr, Player> = HashMap::new();

    loop {
        match r_service.recv() {
            Ok(a) => {                
                let s_service_clone = s_service.clone();
                let s_combat_clone = s_combat.clone();
                on_service(&a, 
                    s_service_clone, 
                    s_combat_clone, 
                    &mut login_infos,
                    &mut players);
            },
            Err(s) => {
                println!("{:?}", s);
                thread::sleep(Duration::from_secs(5000));
            }
        }
    }
}



//业务处理入口
pub fn on_service(
    message: &str, 
    s_service: Sender<String>, 
    s_combat: Sender<String>,
    login_infos: &mut HashMap<SocketAddr, LoginInfo>,
    players: &mut HashMap<SocketAddr, Player>
){
    println!("on_service: {}", message);

    let ps  =  players.clone();

    let msg: Message = serde_json::from_str(&message).unwrap();

    let ms = msg.clone();

    if msg.msg_type == MessageType::Timer {
        return;
    }

    let login_info = login_infos.entry(msg.addr)
                .or_insert(LoginInfo::new());    
    
    if !login_info.b_login {
        let player = players.entry(msg.addr)
            .or_insert(Player::new());
        match crate::login::do_login(&s_service, login_info, player, &msg) {
            Ok(_) => return,
            Err(_) => return
        };
    }

    let mut invoker = Invoker::new();
    let cmd_key = ms.content.split(" ").collect::<Vec<&str>>();
        let cmd_key = match cmd_key.get(0) {
            Some(a) => a,
            None => "none",
        };
    match cmd_key {
        "hp" => invoker.set(Box::new(HpCommand::new(&ps, &s_service, &ms))),
        "l" | "ls" | "look" => invoker.set(Box::new(LookCommand::new(&ps, &s_service, &ms))),
        "fight" => invoker.set(Box::new(FightCommand::new(&ps, &s_service, &ms, &s_combat))),
        "e"|"w"|"s"|"n"|"ne"|"sw"|"se"|"nw" => invoker.set(Box::new(WalkCommand::new(&ps, &s_service, &ms, &s_combat))),
        _ => {
            let nomatch = "There is no match command.";
            let val = wrap_message(msg.addr, nomatch.to_string());
            s_service.send(val).unwrap();
            return;
        }
    }    
    let ret_str = invoker.execute();

    if ret_str.contains("fight") {
        println!("{}", ret_str);
        let ret_str = ret_str.split(" ").collect::<Vec<&str>>();
        let opponent = match ret_str.get(1){
            Some(a) => a,
            None => return,
        };
        let p_hp = match ret_str.get(2){
            Some(a) => a,
            None => return,
        };
        let o_hp = match ret_str.get(3){
            Some(a) => a,
            None => return,
        };
        let timer_id = match ret_str.get(4){
            Some(a) => a,
            None => return,
        };
        for item in players.iter_mut() {
            if item.1.name == login_info.login.login_name {
                item.1.hp = p_hp.parse().unwrap();
                item.1.timer_id = timer_id.parse().unwrap();
            }else if item.1.name == opponent.to_string() {
                item.1.hp = o_hp.parse().unwrap();
            }
        }
    }

    match ret_str.as_str() {
        "e"|"w"|"s"|"n"|"ne"|"sw"|"se"|"nw" => {
            for item in players.iter_mut() {
                if item.1.name == login_info.login.login_name {                    
                    item.1.timer_id = 0;
                }
            }
        },
        _ => (),
    }
    
    if ret_str == "none" {
        let nomatch = "There is no match command.";
        let val = wrap_message(msg.addr, nomatch.to_string());
        s_service.send(val).unwrap();
        return;
    }

}
