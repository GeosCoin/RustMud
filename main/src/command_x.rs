use std::{collections::HashMap, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, Message}, command::Command, player::Player, quest::{self, Quest}};

pub struct XCommand {
    pub players: HashMap<SocketAddr, Player>,    
    pub msg: Message,
}

impl XCommand {
    pub fn new() -> Self  {
            XCommand {
            players: HashMap::new(),
            msg: Message::new(),
        }
    }

    pub fn set_players(&mut self, players: HashMap<SocketAddr, Player>) {
        self.players = players;
    }
}

impl  Command for XCommand  {
    
    fn execute(&self) -> String {
        let player = match self.players.get(&self.msg.addr){
            Some(a) => a,
            None => {return "no".to_string(); }
        };

        "".to_string()
        
    }
}

