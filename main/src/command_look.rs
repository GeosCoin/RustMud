use std::{collections::HashMap, fs::read_to_string, io::Read, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, wrap_message_ext, Message, MessageType}, command::Command, map::Node, player::Player};

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

    pub fn do_localmaps(&self, node: &Node) -> String{
        
        let mut read = utils::load_file(&node.localmaps);
        let mut l_view = String::new();
        read.read_to_string(&mut l_view);
        let l_view = l_view.replace(&node.name,
             &("[1;41m".to_string() + &node.name + "[0;00m"));
        let val = wrap_message(self.msg.addr, l_view.to_string());
        self.s_service.send(val).unwrap();
        return "ok".to_string();
    }

    pub fn do_look(&self, node: &Node) -> String {
        let cmds: Vec<&str> = self.msg.content.split(" ").collect();
        let mut cmd = match cmds.get(1) {
            Some(a) => a,
            None => "",
        };
        let ret_cmd = "look_".to_owned()+cmd;

        let cmd2 = match cmds.get(2) {
            Some(a) => a,
            None => "",
        };
        let cmd = cmd.to_owned() + cmd2;
        
        if cmd != "" {
            let mut view = match node.lookat.get(&cmd){
                Some(a) => a,
                None => {
                    let val = wrap_message(self.msg.addr, "要看什么?".to_string());
                    self.s_service.send(val).unwrap();
                    return "".to_string();
                },
            };

            //来自文件
            let mut l_view = String::new();
            if view.contains(".txt") {                
                let mut read = utils::load_file(view);
                read.read_to_string(&mut l_view);
                view = &l_view;
            }

            let view = view.replace("\\n", "\n");            
            let val = wrap_message(self.msg.addr, view.to_string());
            self.s_service.send(val).unwrap();
            return ret_cmd;
        }

        let player = self.players.get(&self.msg.addr).unwrap();

        //展示对应层次的look
        let mut hash_key = 0;
        if !node.openat.is_empty() {
            hash_key = player.opened;
        } else if (!node.knockat.is_empty()){
            hash_key = player.knocked;
        }

        let look_book = match node.looks.get(&hash_key) {
            Some(a) => a,
            None => &node.look,
        };
        
        let mut read = utils::load_file(look_book);
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
        ret_cmd
    }
    
    fn do_list(&self, node: &Node) -> String {
        if node.list.is_empty() {
            let val = wrap_message(self.msg.addr, "什么？".to_string());
            self.s_service.send(val).unwrap();
            return "".to_string();
        }

        let mut l_view = String::new();   
        let mut read = utils::load_file(&node.list);
        read.read_to_string(&mut l_view);
        
        let view = l_view.replace("\\n", "\n");            
        let val = wrap_message(self.msg.addr, view.to_string());
        self.s_service.send(val).unwrap();
        return "".to_string();
    }
    
    fn do_start_gmcp(&self, node: &Node) -> String {
        let view = "RUSTMUD version : OS v0.01 on port 7878.";
        let val = wrap_message_ext(MessageType::IacDoTerm, self.msg.addr, view.to_string());
        self.s_service.send(val).unwrap();
        return "".to_string();
    }

    fn do_send_gmcp(&self, node: &Node) -> String {
        let view = "Player.Vital {\"hp\": 100, \"maxhp\": 500}".to_owned();
        let val = wrap_message_ext(MessageType::IacDoGmcp, self.msg.addr, view.to_string());
        self.s_service.send(val).unwrap();
        return "".to_string();
    }

}

impl<'a>  Command for LookCommand<'a>  {
    fn execute(&mut self) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();

        let node = match self.nodes.get(&player.pos) {
            Some(a) => a,
            None => {return "no map!".to_string()}
        };

        let cmd_key = self.msg.content.split(" ").collect::<Vec<&str>>();
        let cmd_key = match cmd_key.get(0) {
            Some(a) => a,
            None => "none",
        };

        let cmd = cmd_key.to_string().to_ascii_lowercase();
        match cmd.as_str() {
            "localmaps" | "lm" => {return LookCommand::<'a>::do_localmaps(&self, node)},
            "l" | "ls" | "look" => {return LookCommand::<'a>::do_look(&self, node)},
            "list" => {return LookCommand::<'a>::do_list(&self, node)},
            "startgmcp" => {return LookCommand::<'a>::do_start_gmcp(&self, node)},
            "xgmcp" => {return LookCommand::<'a>::do_send_gmcp(&self, node)},
            _ => {return "ok".to_string();}
        }
        
        "ok".to_string()
    }
}