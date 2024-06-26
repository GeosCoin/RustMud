use std::{collections::HashMap, net::SocketAddr, task::Context};

use chrono::Date;
use crossbeam::channel::Sender;
use utils::now;

use crate::{channel::{wrap_message, wrap_message_ext, Message, MessageType}, command::{Command, Gmcp}, player::{self, Player}};

pub struct ChatCommand<'a> {
    pub players: &'a HashMap<SocketAddr, Player>,
    pub s_service: &'a Sender<String>,
    pub msg: &'a Message,
    pub msg_type: String,
    pub ps: Vec<Player>,
}


impl<'a> ChatCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message
        ) -> Self  {
            ChatCommand {
            players,
            s_service,
            msg,
            msg_type:String::from(""),
            ps: player::init_players(),
        }
    }
}


impl<'a> Gmcp for ChatCommand<'a> {
    fn send_msg(&self, addr: &SocketAddr, message: &str) -> String {   

        let mut message_0a = message.clone().to_string();
        
        message_0a = utils::insert_line(&message_0a, 19);

        let view = "
        Chat {".to_owned() + "
           \"channel\": \""+&self.msg_type+"\",
           \"message\" : \""+&message_0a+"\"
        }";
        let val = wrap_message_ext(MessageType::IacDoGmcp, *addr, view.to_string());
        self.s_service.send(val).unwrap();
        "".to_string()
    }
}

impl<'a>  Command for ChatCommand<'a>  {
    
    fn execute(&mut self) -> String {
        println!("{:?}", self.msg.content);

        let arr: Vec<&str> = self.msg.content.split(" ").collect();
        let para0 = match arr.get(0){
            Some(a) => a,
            None => {return "".to_string();}
        };

        let para1 = match arr.get(1) {
            Some(a) => a,
            None => {
                let view = "要chat什么？";
                let val = wrap_message(self.msg.addr, view.to_string());
                self.s_service.send(val).unwrap();
                return "no content".to_string()
            }
        };

        let player = self.players.get(&self.msg.addr).unwrap();

        //自己不能和自己聊
        if player.name == para1.to_string() {
            let view = "自己不能和自己聊天";
            let val = wrap_message(self.msg.addr, view.to_string());
            self.s_service.send(val).unwrap();
            return "no content".to_string()
        }

        //判断第二个参数值，没有其他玩家或组织,则向世界广播        
        let mut ps_clone = self.ps.clone();
        ps_clone.retain(|p| p.name == para1.to_string());

        self.ps.retain(|p| p.group_name == para1.to_string());

        //玩家和组织都为空时,向世界广播
        if ps_clone.is_empty() && self.ps.is_empty() {
            for p in self.players.iter() {
                
                let content = self.msg.content.trim_start_matches(para0).trim();
                let view = "【世界】".to_owned() 
                + &player.fullname +"("+&player.name+")" +": "+ content;
                let val = wrap_message_ext(MessageType::NoPrompt, *p.0, view.to_string());
                self.s_service.send(val).unwrap();

                self.msg_type = "world".to_string();
                self.send_msg(p.0, &view);
            }
            return "world".to_string()
        }

        //如果有组织,则向组织广播
        if !self.ps.is_empty() {
            let group_players: Vec<(&SocketAddr, &Player)> = self.players.iter()
                .filter(|p| p.1.group_name == para1.to_string())
                .collect();

            for p in group_players.iter() {
                
                let mut content = self.msg.content.trim_start_matches(para0).trim();
                content = content.trim_start_matches(para1).trim();
                let view = "【同盟】".to_owned() 
                + &player.fullname +"("+&player.name+")" +": "+ content;
                let val = wrap_message_ext(MessageType::NoPrompt, *p.0, view.to_string());
                self.s_service.send(val).unwrap();

                self.msg_type = "group".to_string();
                self.send_msg(p.0, &view);
            }
            return "group".to_string()
        }

        //判断是否在线
        let another_player:Vec<(&SocketAddr, &Player)> = self.players.iter()
            .filter(|p| p.1.name == para1.to_string()).collect();
        
        //用户不在线时, 无法发送信息
        if another_player.is_empty() {
            let view = para1.to_string().to_owned() + "现在未上线";
            let val = wrap_message(self.msg.addr, view.to_string());
            self.s_service.send(val).unwrap();
            return "no content".to_string()
        }

        //找到用户，则需要第二个表示内容的参数
        let para2 = match arr.get(2) {
            Some(a) => a,
            None => {
                let view = "要对".to_owned() + para1 + "说什么？";
                let val = wrap_message(self.msg.addr, view.to_string());
                self.s_service.send(val).unwrap();
                return "no content".to_string()
            }
        };

        let content = self.msg.content.trim_start_matches(para0).trim();
        let content = content.trim_start_matches(para1).trim();

        let view = "【私聊】".to_owned() 
        + "来自"+&player.fullname +"("+&player.name+")的消息: "+ content;
        let val = wrap_message_ext(MessageType::NoPrompt, *another_player[0].0, view.to_string());
        self.s_service.send(val).unwrap();

        self.msg_type = "friend".to_string();        
        self.send_msg(another_player[0].0, &view);

        let view = "【私聊】".to_owned() 
        + "发送给" + &another_player[0].1.name +"的消息: "+ content;
        let val = wrap_message_ext(MessageType::NoPrompt, self.msg.addr, view.to_string());
        self.s_service.send(val).unwrap();
        self.send_msg(&self.msg.addr, &view);

        self.msg.content.to_string()
    }
}