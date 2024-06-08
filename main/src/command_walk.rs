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
        if player.pending == 1 {
            let val = wrap_message(self.msg.addr,
                 "你的动作还没有完成，不能移动。".to_string());
                 self.s_service.send(val).unwrap();
            return "".to_string();
        }

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

        let east_id = match node.easts.get(&player.knocked) {
            Some(a) => a,
            None => &node.east_id,
        };
        let west_id = match node.wests.get(&player.knocked) {
            Some(a) => a,
            None => &node.west_id,
        };
        let south_id = match node.souths.get(&player.knocked) {
            Some(a) => a,
            None => &node.south_id,
        };
        let north_id = match node.norths.get(&player.knocked) {
            Some(a) => a,
            None => &node.north_id,
        };
        let northeast_id = match node.northeasts.get(&player.knocked) {
            Some(a) => a,
            None => &node.northeast_id,
        };
        let northwest_id = match node.northwests.get(&player.knocked) {
            Some(a) => a,
            None => &node.northwest_id,
        };
        let southeast_id = match node.southeasts.get(&player.knocked) {
            Some(a) => a,
            None => &node.southeast_id,
        };
        let southwest_id = match node.southwests.get(&player.knocked) {
            Some(a) => a,
            None => &node.southwest_id,
        };

        // if "e"|"w"|"s"|"n"|"ne"|"sw"|"se"|"nw"
        match self.msg.content.as_str() {
            "e" => {new_pos = *east_id;},
            "w" => {new_pos = *west_id;},
            "s" => {new_pos = *south_id;},
            "n" => {new_pos = *north_id;},
            "se" => {new_pos = *southeast_id;},
            "sw" => {new_pos = *southwest_id;},
            "ne" => {new_pos = *northeast_id;},
            "nw" => {new_pos = *northwest_id;},
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

        //展示对应层次的look
        let look_book = match node.looks.get(&player.knocked) {
            Some(a) => a,
            None => &node.look,
        };

        let mut read = utils::load_file(look_book);
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
                 + "    普通百姓 " + &p.1.name + "\n";
        }
        l_view = l_view + &names;

        let val = wrap_message(self.msg.addr, l_view.to_string());
        self.s_service.send(val).unwrap();
        self.msg.content.to_owned() + "@" + &new_pos.to_string()
    }
}
