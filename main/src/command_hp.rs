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
        let sp = show_color(&player.sp.to_string(), Color::YELLOW);
        let max_hp = show_color(&player.max_hp.to_string(), Color::YELLOW);
        let max_mp = show_color(&player.max_mp.to_string(), Color::YELLOW);
        let max_sp = show_color("100", Color::YELLOW);
        let xp = show_color(&player.xp.to_string(), Color::YELLOW);
        let next_xp = show_color("2000", Color::YELLOW);

        let hpframe = r"
    ┌─── ".to_owned() + &name + "状态────────────
     【气血】 "+ &mp +"     / "+&max_mp +"      [100%]     
     【法力】 "+ &hp +"     / "+&max_hp+"      [100%]     
     【信心】 "+ &sp +"     / "+&max_sp+"     [100%]
     【经验】 "+ &xp +"     / "+&next_xp+"                    
    ├────────────────────────────────────────────
    │【状态】 健康                                 
    └──────────────────────────────未知世界──────┘
    ";

        let val = wrap_message_raw(self.msg.addr, hpframe);
        self.s_service.send(val).unwrap();  
        self.send_msg();
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
    fn send_msg(&self) -> String {
        
        let view = "Player.Vital {\"hp\": 200, \"maxhp\": 800, \"msg\": \"兄弟，有时间参加攻城不？\"}".to_owned();
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