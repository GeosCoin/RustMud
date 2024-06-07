use std::{collections::HashMap, fs::read_to_string, io::Read, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, Message}, command::Command, map::Node, player::Player};

pub struct LookCommand<'a> {
    players: &'a HashMap<SocketAddr, Player>,
    s_service: &'a Sender<String>,
    msg: &'a Message,
    nodes: &'a HashMap<u32, Node>
}

impl<'a> LookCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message,
        nodes: &'a HashMap<u32, Node>
        ) -> LookCommand<'a>  {
            LookCommand {
            players,
            s_service,
            msg,
            nodes
        }
    }

    pub fn do_localmaps(node: &Node, s_service: &'a Sender<String>,
    msg: &'a Message) -> String{
        
        let mut read = utils::load_file(&node.localmaps);
        let mut l_view = String::new();
        read.read_to_string(&mut l_view);
        let l_view = l_view.replace(&node.name,
             &("[1;41m".to_string() + &node.name + "[0;00m"));
        let val = wrap_message(msg.addr, l_view.to_string());
        s_service.send(val).unwrap();
        return "ok".to_string();
    }

    pub fn do_look(players: &'a HashMap<SocketAddr, Player>, node: &Node, s_service: &'a Sender<String>,
    msg: &'a Message) -> String {
        let cmds: Vec<&str> = msg.content.split(" ").collect();
        let cmd = match cmds.get(1) {
            Some(a) => a,
            None => "",
        };
        
        if cmd != "" {
            let view = match node.lookat.get(cmd){
                Some(a) => a,
                None => "要看什么?",
            };
            let val = wrap_message(msg.addr, view.to_string());
            s_service.send(val).unwrap();
            return "".to_string();
        }
        
        let mut read = utils::load_file(&node.look);
        let mut l_view = String::new();
        read.read_to_string(&mut l_view);

        let player = players.get(&msg.addr).unwrap();

        for p in players.iter() {
            println!("pos: {} player.pos: {}", p.1.pos, player.pos);
        }

        let others: Vec<(&SocketAddr, &Player)> = players.iter()
            .filter(|p| p.1.name != player.name && p.1.pos == player.pos)
            .collect();
        let mut names = String::from("");
        for p in others {
            names = names
                 + "    普通百姓 " + &p.1.name + "\n";
        }
        l_view = l_view + &names;
        let val = wrap_message(msg.addr, l_view.to_string());
        s_service.send(val).unwrap();
        "ok".to_string()
    }

    pub fn do_knock(player: &Player, node: &Node, s_service: &'a Sender<String>,
        msg: &'a Message) -> String{
        let val = wrap_message(msg.addr, "敲什么？".to_string());
        s_service.send(val).unwrap();
        "knock 1".to_string()
    }
}

impl<'a>  Command for LookCommand<'a>  {
    fn execute(&self) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();

        let node = match self.nodes.get(&player.pos) {
            Some(a) => a,
            None => {return "no map!".to_string()}
        };

        let cmd = self.msg.content.to_ascii_lowercase();
        match cmd.as_str() {
            "localmaps" | "lm" => {return LookCommand::<'a>::do_localmaps(node, self.s_service, self.msg)},
            "l" | "ls" | "look" => {
                return LookCommand::<'a>::do_look(self.players,node, self.s_service, self.msg)},
            "knock" => {
                return LookCommand::<'a>::do_knock(player, node, self.s_service, self.msg)},
            
            _ => {return "ok".to_string();}
        }
        
        "ok".to_string()
    }
}