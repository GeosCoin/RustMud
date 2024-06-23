use std::{collections::HashMap, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, wrap_message_ext, wrap_message_raw, Message, MessageType}, command::{Command, Gmcp}, player::Player};

pub struct HpCommand<'a> {
    pub players: &'a HashMap<SocketAddr, Player>,
    pub s_service: &'a Sender<String>,
    pub msg: &'a Message
}

impl<'a> HpCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message
        ) -> Self  {
        HpCommand {
            players,
            s_service,
            msg
        }
    }

    pub fn do_hp(&self, player: &Player) -> String {
        let name = show_color(&player.name, Color::YELLOW);
        let hp: String = show_color(&player.hp.to_string(), Color::YELLOW);
        let mp = show_color(&player.mp.to_string(), Color::YELLOW);
        let fp = show_color(&player.fp.to_string(), Color::YELLOW);
        let xp = show_color(&player.xp.to_string(), Color::YELLOW);
        let max_hp = show_color(&player.max_hp.to_string(), Color::YELLOW);
        let max_mp = show_color(&player.max_mp.to_string(), Color::YELLOW);
        let max_fp = show_color(&player.max_fp.to_string(), Color::YELLOW);
        let max_xp = show_color(&player.max_xp.to_string(), Color::YELLOW);

        let hpframe = r"
    ┌─── ".to_owned() + &name + "状态────────────
     【气血】 "+ &mp +"     / "+&max_mp +"      [100%]     
     【法力】 "+ &hp +"     / "+&max_hp+"      [100%]     
     【信心】 "+ &fp +"     / "+&max_fp+"     [100%]
     【经验】 "+ &xp +"     / "+&max_xp+"                    
    ├────────────────────────────────────────────
    │【状态】 健康                                 
    └──────────────────────────────未知世界──────┘
    ";

        let val = wrap_message_raw(self.msg.addr, hpframe);
        self.s_service.send(val).unwrap();  
        self.send_msg(&self.msg.addr, "");
        "hp".to_string() 
    }

    pub fn do_who(&self) -> String {
        let mut view = String::from("");
        let mut cnt = 0;
        for player in self.players {
            cnt += 1;
            if (cnt % 2 == 0){
                view = view.to_owned() + &show_color(&player.1.name, Color::GREEN) + "\n";
            }else {
                view = view.to_owned() + &show_color(&player.1.name, Color::GREEN) + " ";
            }
        }
        let val = wrap_message(self.msg.addr, view);
        self.s_service.send(val).unwrap();  

        
        "who".to_string() 
    }

}

impl<'a> Gmcp for HpCommand<'a> {
    fn send_msg(&self, addr: &SocketAddr, message: &str) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();

        let view = "
        Char {
         \"Vitals\" : {
         \"name\": \"".to_owned()+&player.name.to_string()+&"\",
         \"fullname\": \"".to_owned()+&player.fullname.to_string()+&"\", 
         \"level\": "+&player.level.to_string()+&",
         \"hp\": "+&player.hp.to_string()+&",
         \"mp\": "+&player.mp.to_string()+&",
         \"fp\": "+&player.fp.to_string()+&",
         \"xp\": "+&player.xp.to_string()+&",
         \"maxhp\" : "+&player.max_hp.to_string()+&",
         \"maxmp\" : "+&player.max_mp.to_string()+&",
         \"maxfp\" : "+&player.max_fp.to_string()+&",
         \"maxxp\" : "+&player.max_xp.to_string()+&"
         } 
        }".to_owned();
        let val = wrap_message_ext(MessageType::IacDoGmcp, self.msg.addr, view.to_string());
        self.s_service.send(val).unwrap();
        "".to_string()
    }
}

impl<'a>  Command for HpCommand<'a>  {
    
    fn execute(&self) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();

        let cmd = self.msg.content.to_ascii_lowercase();
        match cmd.as_str() {
            "hp" => {return HpCommand::<'a>::do_hp(&self, player)},
            "who" => {return HpCommand::<'a>::do_who(&self)},
            _ => {return "ok".to_string();}
        }
        
    }
}