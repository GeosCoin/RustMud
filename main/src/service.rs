use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use tokio::stream;
use crate::channel::wrap_message;
use crate::channel::wrap_message_ext;
use crate::channel::wrap_message_timer;
use crate::channel::Message;
use crate::channel::MessageType;
use crate::command::Invoker;
use crate::command_climb::ClimbCommand;
use crate::command_fight::FightCommand;
use crate::command_hp::HpCommand;
use crate::command_look::LookCommand;
use crate::command_walk::WalkCommand;
use crate::map;
use crate::map::Node;
use crate::player::Player;
use crate::{channel::{ServerHandler, SessionType, Sessions, SessionContext, SessionsType}, player};
use std::collections::HashMap;
use std::fmt::Error;
use std::hash::Hash;
use std::io::Empty;
use std::net::SocketAddr;
use std::ops::Add;
use std::sync::Arc;
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
    sessions:  SessionsType,
    s_service: Sender<String>,  //发送到socket       
    r_service: Receiver<String>,   //service接收数据
    s_combat: Sender<String>, //发送到fight模块
){
    let mut login_infos: HashMap<SocketAddr, LoginInfo> = HashMap::new();
    let mut players: HashMap<SocketAddr, Player> = HashMap::new();
    let nodes = map::init_map();

    loop {
        match r_service.recv() {
            Ok(a) => {                
                let s_service_clone = s_service.clone();
                let s_combat_clone = s_combat.clone();
                let srv_sessions = Arc::clone(&sessions);

                on_service(&a, 
                    s_service_clone, 
                    s_combat_clone, 
                    &mut login_infos,
                    &mut players,
                    srv_sessions,
                    &nodes);                
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
    players: &mut HashMap<SocketAddr, Player>,
    sessions: SessionsType,
    nodes: &HashMap<u32, Node>
) -> u32 {
    println!("on_service: {}", message);

    let ps  =  players.clone();

    let msg: Message = serde_json::from_str(&message).unwrap();

    let ms = msg.clone();

    if msg.msg_type == MessageType::Timer {
        return 0;
    }

    let login_info = login_infos.entry(msg.addr)
                .or_insert(LoginInfo::new());    
    
    if !login_info.b_login {

        let player = players.entry(msg.addr)
            .or_insert(Player::new());
        match crate::login::do_login(&s_service, login_info, player, &msg, &ps) {
            Ok(a) => {
                //重复用户登录判断  
                if a == 0 {
                    let p_vec : Vec<(&SocketAddr, &Player)> = ps.iter()
                        .filter(|p| p.1.name == login_info.login.login_name)
                        .collect();
                    if !p_vec.is_empty() {
                        println!("{}", p_vec.len());
                        let val = wrap_message(msg.addr, 
                            "此用户已在服务器上登录过，其将被强制退出。".to_string());
                        s_service.send(val).unwrap(); 

                        //删除已登录用户
                        let sessions_login = sessions.lock().unwrap();
                        
                        for p in p_vec.iter() {
                            //先删除，后面stream不一定能得到
                            let addr = p.0;
                            players.remove(addr);

                            let stream = match sessions_login.get(p.0) {
                                Some(a) => a,
                                None => continue
                            };
                            let _ = stream.cur_session.0.shutdown(std::net::Shutdown::Both); 
                            
                        }
                        println!(" Connect count = {}", sessions_login.len());

                        return 99;
                    }

                    
                }
                return 0;
            },
            Err(_) => {return 99;}
        };
    }

    let mut invoker = Invoker::new();
    let cmd_key = ms.content.split(" ").collect::<Vec<&str>>();
        let cmd_key = match cmd_key.get(0) {
            Some(a) => a,
            None => "none",
        };
    match cmd_key {
        "hp"|"who" => invoker.set(Box::new(HpCommand::new(&ps, &s_service, &ms))),
        "l" | "ls" | "look" | "localmaps" | "lm" => invoker.set(Box::new(LookCommand::new(&ps, &s_service, &ms, nodes))),
        "fight" => invoker.set(Box::new(FightCommand::new(&ps, &s_service, &ms, &s_combat))),
        "e"|"w"|"s"|"n"|"ne"|"sw"|"se"|"nw" => invoker.set(Box::new(WalkCommand::new(&ps, &s_service, &ms, &s_combat, nodes))),
        "climb"|"knock"|"open"|"sleep" => invoker.set(Box::new(ClimbCommand::new(&ps, &s_service, &ms, &s_combat, nodes))),
        _ => {
            let nomatch = "要做什么?";
            let val = wrap_message(msg.addr, nomatch.to_string());
            s_service.send(val).unwrap();
            return 0;
        }
    }    
    let ret_str = invoker.execute();

    if ret_str.contains("fight") {
        println!("{}", ret_str);
        let ret_str = ret_str.split(" ").collect::<Vec<&str>>();
        let opponent = match ret_str.get(1){
            Some(a) => a,
            None => return 0,
        };
        let p_hp = match ret_str.get(2){
            Some(a) => a,
            None => return 0,
        };
        let o_hp = match ret_str.get(3){
            Some(a) => a,
            None => return 0,
        };
        let timer_id = match ret_str.get(4){
            Some(a) => a,
            None => return 0,
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

    println!("{}", ret_str);
    if ret_str.contains("e@") || ret_str.contains("w@") 
    || ret_str.contains("n@") || ret_str.contains("s@") 
    || ret_str.contains("ne@") || ret_str.contains("nw@") 
    || ret_str.contains("se@") || ret_str.contains("sw@") 
    {
        let new_pos_vec: Vec<&str> = ret_str.split("@").collect();
        let new_pos = match new_pos_vec.get(1) {
            Some(a) => a,
            None => "0",
        };
        for item in players.iter_mut() {
            if item.1.name == login_info.login.login_name {                    
                item.1.timer_id = 0;
                if new_pos != "0" {
                    item.1.pos = new_pos.parse().unwrap();
                }                
            }
        }
    }
    
    if ret_str.contains("pending") {
        let pendings: Vec<&str> = ret_str.split(" ").collect();
        let pending_status = match pendings.get(1) {
            Some(a) => a,
            None => "0",
        };
        let new_pos = match pendings.get(3) {
            Some(a) => a,
            None => "0",
        };
        
        for item in players.iter_mut() {
            if item.1.name == login_info.login.login_name {                    
                item.1.pending = pending_status.parse().unwrap(); 

                if new_pos != "0" {
                    item.1.pos = new_pos.parse().unwrap();
                }                        
            }
        }
    }

    if ret_str.contains("knocked") 
    || ret_str.contains("opened")
    || ret_str.contains("sleep") {
        let knockeds: Vec<&str> = ret_str.split(" ").collect();
        let knocked_status = match knockeds.get(1) {
            Some(a) => a,
            None => "0",
        };
        
        for item in players.iter_mut() {
            if item.1.name == login_info.login.login_name {  
                if ret_str.contains("knocked") {                  
                    item.1.knocked = knocked_status.parse().unwrap();          
                }else if ret_str.contains("opened") {
                    item.1.opened = knocked_status.parse().unwrap();          
                }else if ret_str.contains("sleep") {
                    item.1.sleep = knocked_status.parse().unwrap(); 
                }
            }
        }
    }

    if ret_str == "none" {
        let nomatch = "There is no match command.";
        let val = wrap_message(msg.addr, nomatch.to_string());
        s_service.send(val).unwrap();
        return 0;
    }

    return 0;
}
