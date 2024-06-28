use std::{collections::HashMap, fs::read_to_string, io::Read, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{get_id, show_color, Color};
use crate::{channel::{wrap_message, wrap_message_climb, wrap_message_ext, Message, MessageType}, command::{Command, Gmcp}, factory_mapfiles::MapFile, map::Node, player::{self, Player}};

pub struct ClimbCommand<'a> {
    players: &'a HashMap<SocketAddr, Player>,
    s_service: &'a Sender<String>,
    msg: &'a Message,
    s_combat: &'a Sender<String>,
    nodes: &'a HashMap<u32, Node>, 
    mapfiles: &'a HashMap<String, MapFile>,
    new_pos: u32,
}

impl<'a> Gmcp for ClimbCommand<'a> {
    fn send_msg(&self, addr: &SocketAddr, message: &str) -> String {
        let cur_node = match self.nodes.get(&self.new_pos) {
            Some(a) => a,
            None => {return "".to_string()}
        };

        let mut content = String::new();

        let factory = self.mapfiles;
        let mapfile = match factory.get(&cur_node.localmaps_gmcp){
            Some(a) => a,
            None => {return "".to_string()}
        };
        content = mapfile.content.clone();
        
        let old_str = &cur_node.name;
        let new_str = "<span style='color: yellow'>".to_owned()+old_str+"</span>";
        let new_content = content.replace(old_str, &new_str);
        let mut view = String::from("
            Map ");
        view = view + "{\"content\" : \""+&new_content+"\"}";
        let val = wrap_message_ext(MessageType::IacDoGmcp, *addr, view.to_string());
        self.s_service.send(val).unwrap();
        "".to_string()
    }
}

impl<'a> ClimbCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message,
        s_combat: &'a Sender<String>,
        nodes: &'a HashMap<u32, Node>,
        mapfiles: &'a HashMap<String, MapFile>,
        ) -> ClimbCommand<'a>  {
            ClimbCommand {
            players,
            s_service,
            msg,
            s_combat,
            nodes,
            mapfiles,
            new_pos: 0
        }
    }

    pub fn do_climb(&mut self, 
        player: &Player,
        node: &Node) -> String {

        let cmds: Vec<&str> = self.msg.content.split(" ").collect();
        let cmd = match cmds.get(1) {
            Some(a) => a,
            None => "",
        };
        let action = match cmds.get(2) {
            Some(a) => a,
            None => "0"
        };
        
        let view = match node.climbat.get(cmd){
            Some(a) => a,
            None => "",
        };

        if cmd == "" || view == ""  {
            let val = wrap_message(self.msg.addr, 
                "要爬到哪里去？试一下climb up".to_string());
            self.s_service.send(val).unwrap();
            return "".to_string();
        }

        let view = view.to_owned();

        if action == "0" {
            let val = wrap_message_ext(MessageType::NoPrompt,self.msg.addr, view.to_string());
            self.s_service.send(val).unwrap();

            //启动定时器
            let timer_id = get_id();
            let val = wrap_message_climb(MessageType::CombatStart, self.msg.addr
                , self.msg.content.to_string(), timer_id.to_string(), 3);
            self.s_combat.send(val).unwrap();
            return "climbing 1".to_string();
        } else if action == "continue" {
            let val = wrap_message_ext(MessageType::NoPrompt, self.msg.addr, view.to_string());
            self.s_service.send(val).unwrap();
            return "climbing 1".to_string();
        } else if action == "stop" {
            let param = cmd.to_string() + "done";
            let view = match node.climbat.get(&param){
                Some(a) => a,
                None => "",
            };

            if view != "" {
                let val = wrap_message(self.msg.addr, view.to_string());
                self.s_service.send(val).unwrap();
            }
        }

        //最后达到目的地，展示目的地的view
        let dest_pos = node.destpos;        
        let node = match self.nodes.get(&dest_pos) {
            Some(a) => a,
            None => {return "no map!".to_string()}
        };

        let mut l_view = String::new();

        let factory = self.mapfiles;
        let mapfile = match factory.get(&node.look){
            Some(a) => a,
            None => {return "".to_string()}
        };
        l_view = mapfile.content.clone();

        //发送到地图
        self.new_pos = dest_pos;
        self.send_msg(&self.msg.addr, "");

        for p in self.players.iter() {
            println!("pos: {} player.pos: {}", p.1.pos, dest_pos);
        }

        let others: Vec<(&SocketAddr, &Player)> = self.players.iter()
            .filter(|p| p.1.name != player.name && p.1.pos == dest_pos)
            .collect();
        let mut names = String::from("");
        for p in others {
            names = names
                 + "    普通百姓 " + &p.1.name + "\n";
        }
        l_view = l_view + &names;

        let val = wrap_message(self.msg.addr, l_view.to_string());
        self.s_service.send(val).unwrap();

        return "climbing 0 destpos ".to_string() + &dest_pos.to_string();
    }

    pub fn do_knock(&self,
        player: &Player,
        node: &Node) -> String {
        
        let cmds: Vec<&str> = self.msg.content.split(" ").collect();
        let cmd = match cmds.get(1) {
            Some(a) => a,
            None => "",
        };

        //接收定时器来的消息
        let action = match cmds.get(2) {
            Some(a) => a,
            None => "0"
        };
        
        let view = match node.knockat.get(cmd){
            Some(a) => a,
            None => "",
        };

        if (cmd == "" || view == "") && player.knocked == 0  {
            let val = wrap_message(self.msg.addr, 
                "要敲什么？试一下knock gate".to_string());
            self.s_service.send(val).unwrap();
            return "".to_string();
        }
        
        let view = view.to_owned();

        if action == "0" {
            let view = view.replace("\\n", "\n");
            let val = wrap_message_ext(MessageType::NoPrompt,self.msg.addr, view.to_string());
            self.s_service.send(val).unwrap();

            //启动定时器
            let timer_id = get_id();
            let val = wrap_message_climb(MessageType::CombatStart, self.msg.addr
                , self.msg.content.to_string(), timer_id.to_string(), 3);
            self.s_combat.send(val).unwrap();
            return "knocked 1".to_string();
        } else if action == "continue" {
            //什么也不做，等着关门就行
            return "knocked 1".to_string();
        } else if action == "stop" {
            let param = cmd.to_string() + "done";
            let view = match node.knockat.get(&param){
                Some(a) => a,
                None => "",
            };

            if view != "" {
                let view = view.replace("\\n", "\n");
                let val = wrap_message(self.msg.addr, view.to_string());
                self.s_service.send(val).unwrap();
            }
        }

        "knocked 0".to_string()
    }

    pub fn do_open(&self,
        player: &Player,
        node: &Node) -> String {
        
        let cmds: Vec<&str> = self.msg.content.split(" ").collect();
        let cmd = match cmds.get(1) {
            Some(a) => a,
            None => "",
        };

        //接收定时器来的消息
        let action = match cmds.get(2) {
            Some(a) => a,
            None => "0"
        };
        
        let view = match node.openat.get(cmd){
            Some(a) => a,
            None => "",
        };

        if (cmd == "" || view == "") && player.opened == 0  {
            let val = wrap_message(self.msg.addr, 
                "要打开什么？试一下open gate".to_string());
            self.s_service.send(val).unwrap();
            return "".to_string();
        }
        
        let view = view.to_owned();

        if action == "0" {
            let view = view.replace("\\n", "\n");
            let val = wrap_message_ext(MessageType::NoPrompt,self.msg.addr, view.to_string());
            self.s_service.send(val).unwrap();

            //启动定时器
            let timer_id = get_id();
            let val = wrap_message_climb(MessageType::CombatStart, self.msg.addr
                , self.msg.content.to_string(), timer_id.to_string(), 3);
            self.s_combat.send(val).unwrap();
            return "opened 1".to_string();
        } else if action == "continue" {
            //什么也不做，等着关门就行
            return "opened 1".to_string();
        } else if action == "stop" {
            let param = cmd.to_string() + "done";
            let view = match node.openat.get(&param){
                Some(a) => a,
                None => "",
            };

            if view != "" {
                let view = view.replace("\\n", "\n");
                let val = wrap_message(self.msg.addr, view.to_string());
                self.s_service.send(val).unwrap();
            }
        }

        "opened 0".to_string()
    }
    
    fn do_sleep(&self, player: &Player, node: &Node) -> String {
        let cmds: Vec<&str> = self.msg.content.split(" ").collect();

        //接收定时器来的消息
        let action = match cmds.get(1) {
            Some(a) => a,
            None => "0"
        };

        if node.sleep.is_empty() {
            let val = wrap_message(self.msg.addr, 
                "这里不是你能睡的地方！".to_string());
            self.s_service.send(val).unwrap();
            return "".to_string();
        }
        
        let view = node.sleep.to_owned();

        if action == "0" {
            let view = view.replace("\\n", "\n");
            let val = wrap_message_ext(MessageType::NoPrompt,self.msg.addr, view.to_string());
            self.s_service.send(val).unwrap();

            //启动定时器
            let timer_id = get_id();
            let val = wrap_message_climb(MessageType::CombatStart, self.msg.addr
                , self.msg.content.to_string(), timer_id.to_string(), 3);
            self.s_combat.send(val).unwrap();
            return "sleep 1".to_string();
        } else if action == "continue" {
            //什么也不做，等着关门就行
            return "sleep 1".to_string();
        } else if action == "stop" {
            let param = "done";
            let view = match node.sleepat.get(param){
                Some(a) => a,
                None => "",
            };

            if view != "" {
                let view = view.replace("\\n", "\n");
                let val = wrap_message(self.msg.addr, view.to_string());
                self.s_service.send(val).unwrap();
            }
        }

        "sleep 0".to_string()
    }
}

impl<'a>  Command for ClimbCommand<'a>  {
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
            "climb" => {return ClimbCommand::<'a>::do_climb(self, player, node)},
            "knock"  => {
                return ClimbCommand::<'a>::do_knock(&self, player, node)},
            "open" => {
                return ClimbCommand::<'a>::do_open(&self, player, node)},
            "sleep" => {
                return ClimbCommand::<'a>::do_sleep(&self, player, node)},            
            _ => {return "ok".to_string();}
        }

        
    }
}