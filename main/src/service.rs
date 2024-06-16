use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use tokio::stream;
use crate::channel::wrap_message;
use crate::channel::wrap_message_ext;
use crate::channel::wrap_message_timer;
use crate::channel::Message;
use crate::channel::MessageType;
use crate::command::EmptyCommand;
use crate::command::Invoker;
use crate::command_climb::ClimbCommand;
use crate::command_fight::FightCommand;
use crate::command_hp::HpCommand;
use crate::command_look::LookCommand;
use crate::command_quest::QuestCommand;
use crate::command_walk::WalkCommand;
use crate::command_x::XCommand;
use crate::log::SINGLETON;
use crate::map;
use crate::map::Node;
use crate::player::Player;
use crate::process::PostProcess;
use crate::process_fight::ProcessFight;
use crate::process_quest::ProcessQuest;
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

pub struct Service<'a> {
    sessions:  &'a SessionsType,
    s_service: &'a Sender<String>,  //发送到socket       
    r_service: &'a Receiver<String>,   //service接收数据
    s_combat: &'a Sender<String>, //发送到fight模块
    login_infos: HashMap<SocketAddr, LoginInfo>,
    players: HashMap<SocketAddr, Player>,
    nodes: HashMap<u32, Node>,
    quests: HashMap<u32, Quest>
}

impl<'a>  Service<'a> {
    pub fn new(sessions:  &'a SessionsType,
        s_service: &'a Sender<String>,  //发送到socket       
        r_service: &'a Receiver<String>,   //service接收数据
        s_combat: &'a Sender<String>) -> Self {        
        Service {
            sessions,
            s_service,
            r_service,
            s_combat,
            login_infos: HashMap::new(),
            players: HashMap::new(),
            nodes: map::init_map(),
            quests: quest::init_quest()
        }
    }

    pub fn handle(&mut self){

        loop {
            match self.r_service.recv() {
                Ok(a) => {
                    self.on_service(&a);
                },
                Err(s) => {
                    println!("{:?}", s);
                    thread::sleep(Duration::from_secs(5000));
                }
            }
        }
    }

    pub fn on_service(&mut self, message: &str) -> u32{
        println!("on_service: {}", message);

    let ps  =  self.players.clone();

    let msg: Message = serde_json::from_str(&message).unwrap();

    let ms = msg.clone();

    if msg.msg_type == MessageType::Timer {
        return 0;
    }

    let login_info = self.login_infos.entry(msg.addr)
                .or_insert(LoginInfo::new());    
    
    if !login_info.b_login {

        let player = self.players.entry(msg.addr)
            .or_insert(Player::new());
        let player_clone = player.clone();
        match crate::login::do_login(&self.s_service, login_info, player, &msg, &ps) {
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
                            self.s_service.send(val).unwrap(); 

                        //删除已登录用户
                        let sessions_login = self.sessions.lock().unwrap();
                        
                        for p in p_vec.iter() {
                            //先删除，后面stream不一定能得到
                            let addr = p.0;
                            self.players.remove(addr);

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
                    let quest = match self.quests.get(&player_clone.newbie_next){
                        Some(a) => a,
                        None => return 99,
                    };
            
                    //提示
                    let val = wrap_message(msg.addr, quest.job.to_string());
                    self.s_service.send(val).unwrap();                     
                    return 0;
                }
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
        "hp"|"who" => invoker.set(Box::new(HpCommand::new(&ps, &self.s_service, &ms))),
        "jq" | "jobquery" => invoker.set(Box::new(QuestCommand::new(&ps, &self.s_service, &ms, &self.quests))),
        "l" | "ls" | "look" | "localmaps" | "lm"
        | "list" | "startgmcp" | "xgmcp"  => invoker.set(Box::new(LookCommand::new(&ps, &self.s_service, &ms, &self.nodes))),
        "fight" => invoker.set(Box::new(FightCommand::new(&ps, &self.s_service, &ms, &self.s_combat))),
        "e"|"w"|"s"|"n"|"ne"|"sw"|"se"|"nw" => invoker.set(Box::new(WalkCommand::new(&ps, &self.s_service, &ms, &self.s_combat, &self.nodes))),
        "climb"|"knock"|"open"|"sleep"|"bath" => invoker.set(Box::new(ClimbCommand::new(&ps, &self.s_service, &ms, &self.s_combat, &self.nodes))),
        _ => {
            let nomatch = "要做什么?";
            let val = wrap_message(msg.addr, nomatch.to_string());
            self.s_service.send(val).unwrap();
            return 0;
        }
    }    
    let ret_str = invoker.execute();

    //处理fight事务
    let mut post_fight = ProcessFight::new(ret_str.to_string(), self.s_service, &mut self.players, login_info, &msg);
    post_fight.execute();

    //处理新手任务
    let mut post_quest = ProcessQuest::new(ret_str.to_string(), &self.quests, &self.s_service, &mut self.players, login_info, &msg);
    post_quest.execute();

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
        for item in self.players.iter_mut() {
            if item.1.name == login_info.login.login_name {                    
                item.1.timer_id = 0;
                if new_pos != "0" {
                    item.1.pos = new_pos.parse().unwrap();
                }                
            }
        }
    }
    
    if ret_str.contains("climbing") {
        let pendings: Vec<&str> = ret_str.split(" ").collect();
        let pending_status = match pendings.get(1) {
            Some(a) => a,
            None => "0",
        };
        let new_pos = match pendings.get(3) {
            Some(a) => a,
            None => "0",
        };
        
        for item in self.players.iter_mut() {
            if item.1.name == login_info.login.login_name {                    
                item.1.climbing = pending_status.parse().unwrap(); 

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
        
        for item in self.players.iter_mut() {
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
        self.s_service.send(val).unwrap();
        return 0;
    }

    return 0;
    }
}


fn deal_quest(ret_str: String, quests: &HashMap<u32, Quest>, player: &Player
, s_service: &Sender<String>, players: &mut HashMap<SocketAddr, Player>, 
login_info: &LoginInfo, msg: &Message) {
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
            let node = quest[0].1.node;
            let next = quest[0].1.next;
            println!("next: {:?} node: {}", next, node);
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

            let new_pos_vec: Vec<&str> = ret_str.split("@").collect();
            let new_pos = match new_pos_vec.get(1) {
                Some(a) => a,
                None => "0",
            };
            let new_pos: u32 = new_pos.parse().unwrap();

            for item in players.iter_mut() {
                if item.1.name == login_info.login.login_name {  
                    println!("{}{}", node, new_pos);
                    if node == new_pos {
                        item.1.newbie_quest.insert(id, true); 
                    }
                    if parent == 0 {           
                        item.1.newbie_next = next;                    
                        item.1.xp += xp;
                        item.1.sp += sp;
                    }

                    //如果有父项，判断父项下的子项是否已经全部满足
                    if parent != 0 {
                        println!("parent= {}", parent);
                        let quest = match quests.get(&parent) {
                            Some(a) => a,
                            None => return,
                        };
                        
                        let subquest = &quest.subquest;
                        let mut all_exist = true;
                        for sub in subquest.iter() {
                            println!("sub: {:?}", sub.0);
                            let is_exist = match item.1.newbie_quest.get(&sub.0){
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
                            item.1.newbie_next = quest.next;                    
                            item.1.xp += quest.xp;
                            item.1.sp += quest.sp;

                            let val = wrap_message_ext(MessageType::NoPrompt,msg.addr, quest.award.to_string());
                            s_service.send(val).unwrap();    

                            let val = wrap_message_ext(MessageType::NoPrompt,msg.addr, quest.after.to_string());
                            s_service.send(val).unwrap();
                        }
                    }
                }
            }

        }
        // *newbie_prompt = 0;
    }
}
