use std::{collections::HashMap, io::Read, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, wrap_message_ext, wrap_message_timer, Message, MessageType}, command::{Command, Gmcp}, map::Node, player::Player};

pub struct WalkCommand<'a> {
    players: &'a HashMap<SocketAddr, Player>,
    s_service: &'a Sender<String>,
    msg: &'a Message,
    s_combat: &'a Sender<String>,
    nodes: &'a HashMap<u32, Node>,
    new_pos: u32
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
            nodes,
            new_pos: 0
        }
    }
}
impl<'a>  Command for WalkCommand<'a>  {
    fn execute(&mut self) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();
        if player.climbing == 1 || player.sleep == 1 {
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

        //判断open/knocked状态
        let mut hash_key = 0;
        if !node.openat.is_empty() {
            hash_key = player.opened;
        } else if (!node.knockat.is_empty()){
            hash_key = player.knocked;
        }

        let east_id = match node.easts.get(&hash_key) {
            Some(a) => a,
            None => &node.east_id,
        };
        let west_id = match node.wests.get(&hash_key) {
            Some(a) => a,
            None => &node.west_id,
        };
        let south_id = match node.souths.get(&hash_key) {
            Some(a) => a,
            None => &node.south_id,
        };
        let north_id = match node.norths.get(&hash_key) {
            Some(a) => a,
            None => &node.north_id,
        };
        let northeast_id = match node.northeasts.get(&hash_key) {
            Some(a) => a,
            None => &node.northeast_id,
        };
        let northwest_id = match node.northwests.get(&hash_key) {
            Some(a) => a,
            None => &node.northwest_id,
        };
        let southeast_id = match node.southeasts.get(&hash_key) {
            Some(a) => a,
            None => &node.southeast_id,
        };
        let southwest_id = match node.southwests.get(&hash_key) {
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

        //发送到地图
        self.new_pos = new_pos;
        self.send_msg(&self.msg.addr, "");

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

impl<'a> Gmcp for WalkCommand<'a> {
    fn send_msg(&self, addr: &SocketAddr, message: &str) -> String {
        // let mut content = "<pre>◆柳秀山庄地图◆ <br /><br />               藏书阁25 <br />                  │ <br /> 男浴室21      尚武堂24      女浴室23<br />    │             │           │ <br /> 西厢房20──── 柳秀山庄正厅19────东厢房22<br />                  │ <br />               岩桂花园18 <br />                  │<br />                长廊17<br />                  &and;<br />       当铺16──山庄大门14──票号15<br />                  │<br />       铁匠铺13─集镇小道12─杂货铺11<br />                  │<br />                  │ 药铺10<br />                  │╱<br />      (扬州)26──杏子林9──集镇小道6<br />        │         │╲<br />      车马行8      │ 酒铺7<br />                  │<br />                缓坡5<br />                  〓 <br />          树林3─未明谷1─乱石阵4<br />                  │<br />               青石桥头2 <br /> </pre>";
        
        let cur_node = match self.nodes.get(&self.new_pos) {
            Some(a) => a,
            None => {return "".to_string()}
        };

        let mut read = utils::load_file(&cur_node.localmaps_gmcp);
        let mut content = String::new();
        read.read_to_string(&mut content);
        
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
