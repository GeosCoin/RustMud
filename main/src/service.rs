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
use crate::command_chat::ChatCommand;
use crate::command_climb::ClimbCommand;
use crate::command_fight::FightCommand;
use crate::command_friend::FriendCommand;
use crate::command_hp::HpCommand;
use crate::command_look::LookCommand;
use crate::command_map::MapCommand;
use crate::command_quest::QuestCommand;
use crate::command_walk::WalkCommand;
use crate::command_x::XCommand;
use crate::setting_maps;
use crate::setting_maps::MapFile;
use crate::login::LoginService;
use crate::map;
use crate::map::Node;
use crate::player::Player;
use crate::process::PostProcess;
use crate::process::ProcessNone;
use crate::process_climb::ProcessClimb;
use crate::process_fight::ProcessFight;
use crate::process_quest::ProcessQuest;
use crate::process_walk::ProcessWalk;
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
    quests: HashMap<u32, Quest>,
    mapfiles: HashMap<String, MapFile>,
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
            quests: quest::init_quest(),
            mapfiles: setting_maps::init_mapfiles(),
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
        
        //登录处理
        if !login_info.b_login {

            let mut login_srv = LoginService::new(self.s_service, login_info, &msg, &mut self.players, self.sessions, &self.quests);
            return login_srv.do_login();
        }

        //命令处理
        let mut invoker = Invoker::new();
        let cmd_key = ms.content.split(" ").collect::<Vec<&str>>();
            let cmd_key = match cmd_key.get(0) {
                Some(a) => a,
                None => "none",
            };
        match cmd_key {
            "hp"|"who" => invoker.set(Box::new(HpCommand::new(&ps, &self.s_service, &ms))),
            "jq"|"jobquery" => invoker.set(Box::new(QuestCommand::new(&ps, &self.s_service, &ms, &self.quests))),
            "l"|"ls"|"look"|"localmaps"|"lm"
                |"list"|"startgmcp"|"xgmcp"  => invoker.set(Box::new(LookCommand::new(&ps, &self.s_service, &ms, &self.nodes, &self.mapfiles))),
            "fight" => invoker.set(Box::new(FightCommand::new(&ps, &self.s_service, &ms, &self.s_combat))),
            "e"|"w"|"s"|"n"|"ne"|"sw"|"se"|"nw" => invoker.set(Box::new(WalkCommand::new(&ps, &self.s_service, &ms, &self.s_combat, &self.nodes, &self.mapfiles))),
            "climb"|"knock"|"open"|"sleep"|"bath" => invoker.set(Box::new(ClimbCommand::new(&ps, &self.s_service, &ms, &self.s_combat, &self.nodes, &self.mapfiles))),
            "chat"|"`" => invoker.set(Box::new(ChatCommand::new(&ps, &self.s_service, &ms))),
            "friend"|"group" => invoker.set(Box::new(FriendCommand::new(&ps, &self.s_service, &ms))),
            "gmcp.localmap" => invoker.set(Box::new(MapCommand::new(&ps, &self.s_service, &ms, &self.nodes, &self.mapfiles))),
            _ => {
                let nomatch = "要做什么?";
                let val = wrap_message(msg.addr, nomatch.to_string());
                self.s_service.send(val).unwrap();
                return 0;
            }
        }    
        let ret_str = invoker.execute();

        //处理战斗 fight
        let mut post_fight = ProcessFight::new(ret_str.to_string(), self.s_service, &mut self.players, login_info, &msg);
        post_fight.execute();

        //处理任务 quest
        let mut post_quest = ProcessQuest::new(ret_str.to_string(), &self.quests, &self.s_service, &mut self.players, login_info, &msg);
        post_quest.execute();

        //处理行走 east west ...
        let mut post_walk = ProcessWalk::new(ret_str.to_string(), &self.s_service, &mut self.players, login_info, &msg);
        post_walk.execute();
        
        //处理climb, knock, open, sleep等持续动作
        let mut post_climb = ProcessClimb::new(ret_str.to_string(), &self.s_service, &mut self.players, login_info, &msg);
        post_climb.execute();

        //处理未匹配情况 none
        let mut post_none = ProcessNone::new(ret_str, &self.s_service, &msg);
        post_none.execute();

        return 0;
    }
}

