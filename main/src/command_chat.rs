use std::{collections::HashMap, net::SocketAddr};

use crossbeam::channel::Sender;

use crate::{channel::{wrap_message, wrap_message_ext, Message, MessageType}, command::{Command, Gmcp}, player::Player};

pub struct ChatCommand<'a> {
    pub players: &'a HashMap<SocketAddr, Player>,
    pub s_service: &'a Sender<String>,
    pub msg: &'a Message
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
            msg
        }
    }
}


impl<'a> Gmcp for ChatCommand<'a> {
    fn send_msg(&self) -> String {
        let msg_vec:Vec<&str> = self.msg.content.split(" ").collect();
        if msg_vec.len() < 2 {
            return "".to_string();
        }
        let msg_str = &msg_vec[1..];
        let mut dialog = String::from("");
        for i in msg_str.iter() {
            dialog = dialog + i + " ";
        }

        let view = "Player.Vital {\"hp\": 200, \"maxhp\": 800, \"msg\": \"".to_owned()+ &dialog+"\"}";
        let val = wrap_message_ext(MessageType::IacDoGmcp, self.msg.addr, view.to_string());
        self.s_service.send(val).unwrap();
        "".to_string()
    }
}

impl<'a>  Command for ChatCommand<'a>  {
    
    fn execute(&self) -> String {
        println!("{:?}", self.msg.content);

        let arr: Vec<&str> = self.msg.content.split(" ").collect();
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

        //判断第二个参数值，如果为其他玩家，或者 todo 组织
        //则要求有第三个参数
        let another_player:Vec<(&SocketAddr, &Player)> = self.players.iter()
            .filter(|p| p.1.name == para1.to_string()).collect();
        
        //表示第1个参数不是用户名，则是对世界的广播
        if another_player.is_empty() {
            for p in self.players.iter() {
                // if p.1.name == player.name {
                //     continue;
                // }

                let view = "【世界】".to_owned() 
                + &player.name +": "+ para1;
                let val = wrap_message_ext(MessageType::NoPrompt, *p.0, view.to_string());
                self.s_service.send(val).unwrap();
            }
            return "world".to_string()
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

        let view = "【私聊】".to_owned() 
        + "来自"+&player.name +"的消息: "+ para2;
        let val = wrap_message_ext(MessageType::NoPrompt, *another_player[0].0, view.to_string());
        self.s_service.send(val).unwrap();

        let view = "【私聊】".to_owned() 
        + "发送给" + &another_player[0].1.name +"的消息: "+ para2;
        let val = wrap_message_ext(MessageType::NoPrompt, self.msg.addr, view.to_string());
        self.s_service.send(val).unwrap();

        // self.send_msg();
        self.msg.content.to_string()
    }
}