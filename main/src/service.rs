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
use crate::command_quest::QuestCommand;
use crate::command_walk::WalkCommand;
use crate::map;
use crate::map::Node;
use crate::player::Player;
use crate::quest;
use crate::quest::Quest;
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
    let quests = quest::init_quest();
    let mut newbie_prompt: u32 = 0; //未提示过

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
                    &nodes,
                    &quests,
                    &mut newbie_prompt);                
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
    nodes: &HashMap<u32, Node>,
    quests: &HashMap<u32, Quest>,
    newbie_prompt: &mut u32,
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
        let player_clone = player.clone();
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

                        // return 99;
                    }

                    
                }

                //已登录并且新手向导存在
                if login_info.b_login && player_clone.newbie_next != 0 {
                    //登录时就进行首轮提示                    
                    let quest = match quests.get(&player_clone.newbie_next){
                        Some(a) => a,
                        None => return 99,
                    };
            
                    //提示
                    let val = wrap_message(msg.addr, quest.job.to_string());
                    s_service.send(val).unwrap(); 
                    *newbie_prompt = 1;
                    return 0;
                }
            },
            Err(_) => {return 99;}
        };
    }

    //判断新手向导
    let player = match ps.get(&msg.addr) {
        Some(a) => a,
        None => return 99,
    };

    let mut invoker = Invoker::new();
    let cmd_key = ms.content.split(" ").collect::<Vec<&str>>();
        let cmd_key = match cmd_key.get(0) {
            Some(a) => a,
            None => "none",
        };
    match cmd_key {
        "hp"|"who" => invoker.set(Box::new(HpCommand::new(&ps, &s_service, &ms))),
        "jq" | "jobquery" => invoker.set(Box::new(QuestCommand::new(&ps, &s_service, &ms, quests))),
        "l" | "ls" | "look" | "localmaps" | "lm"
        | "list" | "startgmcp" | "xgmcp"  => invoker.set(Box::new(LookCommand::new(&ps, &s_service, &ms, nodes))),
        "fight" => invoker.set(Box::new(FightCommand::new(&ps, &s_service, &ms, &s_combat))),
        "e"|"w"|"s"|"n"|"ne"|"sw"|"se"|"nw" => invoker.set(Box::new(WalkCommand::new(&ps, &s_service, &ms, &s_combat, nodes))),
        "climb"|"knock"|"open"|"sleep"|"bath" => invoker.set(Box::new(ClimbCommand::new(&ps, &s_service, &ms, &s_combat, nodes))),
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

    //判断是否满足新手任务
    let mut quest_ret = ret_str.clone();    
    if quest_ret.contains("e@") {
        quest_ret = "east".to_string();
    } else if quest_ret.contains("w@") {
        quest_ret = "west".to_string();
    } else if quest_ret.contains("s@") {
        quest_ret = "south".to_string();
    } else if quest_ret.contains("n@") {
        quest_ret = "north".to_string();
    }

    quest_ret = "q_".to_owned() + &quest_ret;
    println!("quest_ret: {:?}", quest_ret);
    let quest: Vec<(&u32, &Quest)> = quests.iter().filter(|p| p.1.name == quest_ret).collect();
    if !quest.is_empty() {
        
        let id = quest[0].1.id;
        let is_id_exist = match player.newbie_quest.get(&id){
            Some(a) => true,
            None => false,
        };

        //如果不存在，表示有新任务
        if !is_id_exist {

            let parent = quest[0].1.parent;
            let next = quest[0].1.next;
            println!("next: {:?}", next);
            let xp = quest[0].1.xp;
            let sp = quest[0].1.sp;
            let award = &quest[0].1.award;
            let after = &quest[0].1.after;
            
            
            //如果没有父项，才会通知
            if parent == 0 {
                let val = wrap_message_ext(MessageType::NoPrompt,msg.addr, award.to_string());
                s_service.send(val).unwrap();    

                let val = wrap_message_ext(MessageType::NoPrompt,msg.addr, after.to_string());
                s_service.send(val).unwrap();
            }

            for item in players.iter_mut() {
                if item.1.name == login_info.login.login_name {        
                    item.1.newbie_quest.insert(id, true);            
                    item.1.newbie_next = next;
                    item.1.xp += xp;
                    item.1.sp += sp;

                    //如果有父项，判断父项下的子项是否已经全部满足
                    if parent != 0 {
                        let quest = match quests.get(&parent) {
                            Some(a) => a,
                            None => return 99,
                        };
                        
                        let subquest = &quest.subquest;
                        let mut all_exist = true;
                        for item in subquest.iter() {
                            let is_exist = match player.newbie_quest.get(&item.0){
                                Some(a) => true,
                                None => false,
                            };
                            if !is_exist {
                                all_exist = false;
                                break;
                            }
                        }
                        
                        if all_exist {
                            item.1.newbie_quest.insert(parent, true);
                        }
                    }
                }
            }

        }
        // *newbie_prompt = 0;
    }

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
