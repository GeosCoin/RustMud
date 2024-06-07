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
}
impl<'a>  Command for LookCommand<'a>  {
    fn execute(&self) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();

        let node = match self.nodes.get(&player.pos) {
            Some(a) => a,
            None => {return "no map!".to_string()}
        };

        if self.msg.content == "localmaps"
            || self.msg.content == "lm" { 
        
            let mut read = utils::load_file(&node.localmaps);
            let mut l_view = String::new();
            read.read_to_string(&mut l_view);
            let l_view = l_view.replace(&node.name,
                 &("[1;41m".to_string() + &node.name + "[0;00m"));
            let val = wrap_message(self.msg.addr, l_view.to_string());
            self.s_service.send(val).unwrap();
            return "ok".to_string();
        }

        let cmds: Vec<&str> = self.msg.content.split(" ").collect();
        let cmd = match cmds.get(1) {
            Some(a) => a,
            None => "",
        };
        
        if cmd != "" {
            let view = match node.lookat.get(cmd){
                Some(a) => a,
                None => "要看什么?",
            };
            let val = wrap_message(self.msg.addr, view.to_string());
            self.s_service.send(val).unwrap();
            return "".to_string();
        }
        
        let mut read = utils::load_file(&node.look);
        let mut l_view = String::new();
        read.read_to_string(&mut l_view);

        for p in self.players.iter() {
            println!("pos: {} player.pos: {}", p.1.pos, player.pos);
        }

        let others: Vec<(&SocketAddr, &Player)> = self.players.iter()
            .filter(|p| p.1.name != player.name && p.1.pos == player.pos)
            .collect();
        let mut names = String::from("");
        for p in others {
            names = names
                 + "    普通百姓 " + &p.1.name + "\n";
        }
        l_view = l_view + &names;
        let val = wrap_message(self.msg.addr, l_view.to_string());
        self.s_service.send(val).unwrap();
        "ok".to_string()
    }
}