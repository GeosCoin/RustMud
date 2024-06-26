use std::{collections::HashMap, net::SocketAddr};

use crossbeam::channel::Sender;

use crate::{channel::{wrap_message, wrap_message_ext, Message, MessageType}, command::{Command, Gmcp}, player::{self, Group, Groups, Player}};

pub struct FriendCommand<'a> {
    pub players: &'a HashMap<SocketAddr, Player>,
    pub s_service: &'a Sender<String>,
    pub msg: &'a Message,
    pub msg_type: String,
    pub groups: Vec<Group>,
}


impl<'a> FriendCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message
        ) -> Self  {
            FriendCommand {
            players,
            s_service,
            msg,
            msg_type: String::from(""),
            groups: player::init_groups(),
        }
    }
}


impl<'a> Gmcp for FriendCommand<'a> {
    fn send_msg(&self, addr: &SocketAddr, message: &str) -> String {  

        let player = self.players.get(&self.msg.addr).unwrap();
        
        let mut i = 0;
        let mut len = 0;
        let mut view = String::from("");

        if self.msg_type == "friend" {  
            i = 0;     
            len = player.friends.len(); 
            view = String::from("
            Friend [");
            for friend in player.friends.iter() {
                i += 1;
                if i < len {
                    view = view + "{\"name\" : \""+&friend+"\",},";
                } else if i == len {
                    view = view + "{\"name\" : \""+&friend+"\"}";
                }
            }
            view += "]";
        }

        if self.msg_type == "group" {
            println!("self.groups.len() = {}", self.groups.len());
            let groups: Vec<&Group> = self.groups.iter().filter(|p|p.name == player.group_name).collect();
            let group = match groups.get(0) {
                Some(a) => a,
                None => {return "no group".to_string()}
            };

            i = 0;  
            len = group.members.len(); 
            view = String::from("
            Group [");
            for m in group.members.iter() {
                i += 1;
                if i < len {
                    view = view + "{\"name\" : \""+&m+"\"},";
                } else if i == len {
                    view = view + "{\"name\" : \""+&m+"\"}";
                }
            }
            view += "]";
        }

        let val = wrap_message_ext(MessageType::IacDoGmcp, *addr, view.to_string());
        self.s_service.send(val).unwrap();
        "".to_string()
    }
}

impl<'a>  Command for FriendCommand<'a>  {
    
    fn execute(&mut self) -> String {
        self.msg_type = self.msg.content.to_ascii_lowercase();
        self.send_msg(&self.msg.addr, "");
        self.msg.content.to_string()
    }
}