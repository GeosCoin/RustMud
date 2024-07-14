use std::{collections::HashMap, io::Read, net::SocketAddr};

use crossbeam::channel::Sender;

use crate::{channel::{wrap_message, wrap_message_ext, Message, MessageType}, command::{Command, Gmcp}, map::Node, player::{self, Group, Groups, Player}, setting_maps::MapFile};

pub struct MapCommand<'a> {
    pub players: &'a HashMap<SocketAddr, Player>,
    pub s_service: &'a Sender<String>,
    pub msg: &'a Message,
    pub msg_type: String,
    pub nodes: &'a HashMap<u32, Node>,
    pub mapfiles: &'a HashMap<String, MapFile>,
}


impl<'a> MapCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message,
        nodes: &'a HashMap<u32, Node>,
        mapfiles: &'a HashMap<String, MapFile>,
        ) -> Self  {
            MapCommand {
            players,
            s_service,
            msg,
            msg_type: String::from(""),
            nodes,
            mapfiles
        }
    }
}


impl<'a> Gmcp for MapCommand<'a> {
    fn send_msg(&self, addr: &SocketAddr, message: &str) -> String {  
        
        let player = self.players.get(&self.msg.addr).unwrap();

        
        // let mut content = "<pre>◆柳秀山庄地图◆ <br /><br />               藏书阁25 <br />                  │ <br /> 男浴室21      尚武堂24      女浴室23<br />    │             │           │ <br /> 西厢房20──── 柳秀山庄正厅19────东厢房22<br />                  │ <br />               岩桂花园18 <br />                  │<br />                长廊17<br />                  &and;<br />       当铺16──山庄大门14──票号15<br />                  │<br />       铁匠铺13─集镇小道12─杂货铺11<br />                  │<br />                  │ 药铺10<br />                  │╱<br />      (扬州)26──杏子林9──集镇小道6<br />        │         │╲<br />      车马行8      │ 酒铺7<br />                  │<br />                缓坡5<br />                  〓 <br />          树林3─未明谷1─乱石阵4<br />                  │<br />               青石桥头2 <br /> </pre>";
        
        let cur_node = match self.nodes.get(&player.pos) {
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

impl<'a>  Command for MapCommand<'a>  {
    
    fn execute(&mut self) -> String {
        self.msg_type = self.msg.content.to_ascii_lowercase();
        self.send_msg(&self.msg.addr, "");
        self.msg.content.to_string()
    }
}

