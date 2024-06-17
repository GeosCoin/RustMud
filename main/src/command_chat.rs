use std::{collections::HashMap, net::SocketAddr};

use crossbeam::channel::Sender;

use crate::{channel::{wrap_message_ext, Message, MessageType}, command::{Command, Gmcp}, player::Player};

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
        self.send_msg();
        "".to_string()
    }
}