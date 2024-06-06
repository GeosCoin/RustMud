use std::{collections::HashMap, io::Read, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, wrap_message_ext, wrap_message_timer, Message, MessageType}, command::Command, map::Node, player::Player};

pub struct WalkCommand<'a> {
    players: &'a HashMap<SocketAddr, Player>,
    s_service: &'a Sender<String>,
    msg: &'a Message,
    s_combat: &'a Sender<String>,
    nodes: &'a HashMap<u32, Node>
}

impl<'a> WalkCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message,
        s_combat: &'a Sender<String>,
        nodes: &'a HashMap<u32, Node>
        ) -> WalkCommand<'a>  {
            WalkCommand {
            players,
            s_service,
            msg,
            s_combat,
            nodes
        }
    }
}
impl<'a>  Command for WalkCommand<'a>  {
    fn execute(&self) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();
        let mut new_pos = 0;

        //如果timer_id不为0,则关闭
        if player.timer_id != 0 {
            let val = wrap_message_timer(MessageType::CombatStop,
                self.msg.addr, "stop".to_string(), player.timer_id.to_string());
            self.s_combat.send(val).unwrap();
        }

        let node = match self.nodes.get(&player.pos) {
            Some(a) => a,
            None => {return "no map!".to_string()}
        };

        // if "e"|"w"|"s"|"n"|"ne"|"sw"|"se"|"nw"
        match self.msg.content.as_str() {
            "e" => {new_pos = node.east_id;},
            "w" => {new_pos = node.west_id;},
            "s" => {new_pos = node.south_id;},
            "n" => {new_pos = node.north_id;},
            "se" => {new_pos = node.southeast_id;},
            "sw" => {new_pos = node.southwest_id;},
            "ne" => {new_pos = node.northeast_id;},
            "nw" => {new_pos = node.northwest_id;},
            _ => {},
        };

        if new_pos == 0 {
            let val = wrap_message_ext(MessageType::Sender,
                self.msg.addr, "这里没有路".to_string());
            self.s_service.send(val).unwrap();
        }

        let node = match self.nodes.get(&new_pos) {
            Some(a) => a,
            None => {return "no map!".to_string()}
        };

        let mut read = utils::load_file(&node.look);
        let mut l_view = String::new();
        read.read_to_string(&mut l_view);

        for p in self.players.iter() {
            println!("pos: {} player.pos: {}", p.1.pos, new_pos);
        }

        let others: Vec<(&SocketAddr, &Player)> = self.players.iter()
            .filter(|p| p.1.name != player.name && p.1.pos == new_pos)
            .collect();
        let mut names = String::from("");
        for p in others {
            names = names
                 + "\n    普通百姓 " + &p.1.name + "\n";
        }
        l_view = l_view + &names;

        let val = wrap_message(self.msg.addr, l_view.to_string());
        self.s_service.send(val).unwrap();
        self.msg.content.to_owned() + "@" + &new_pos.to_string()
    }
}
