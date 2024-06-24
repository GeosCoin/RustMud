use std::{collections::HashMap, net::SocketAddr};

use crossbeam::channel::Sender;

use crate::{channel::{wrap_message, wrap_message_ext, Message, MessageType}, command::{Command, Gmcp}, player::Player};

pub struct FriendCommand<'a> {
    pub players: &'a HashMap<SocketAddr, Player>,
    pub s_service: &'a Sender<String>,
    pub msg: &'a Message
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
            msg
        }
    }
}


impl<'a> Gmcp for FriendCommand<'a> {
    fn send_msg(&self, addr: &SocketAddr, message: &str) -> String {  

        let player = self.players.get(&self.msg.addr).unwrap();

        let mut i = 0;
        let len = player.friends.len();
        let mut view = String::from("
        Friend [");
        for friend in player.friends.iter() {
            i += 1;
            if i < len {
                view = view + "{\"name\" : \""+&friend+"\"},";
            } else if i == len {
                view = view + "{\"name\" : \""+&friend+"\"}";
            }
        }
        view += "]";

        let val = wrap_message_ext(MessageType::IacDoGmcp, *addr, view.to_string());
        self.s_service.send(val).unwrap();
        "".to_string()
    }
}

impl<'a>  Command for FriendCommand<'a>  {
    
    fn execute(&self) -> String {
        self.send_msg(&self.msg.addr, "");
        self.msg.content.to_string()
    }
}