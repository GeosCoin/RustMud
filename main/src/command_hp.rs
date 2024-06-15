use std::{collections::HashMap, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, Message}, command::Command, player::Player};

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
        let max_hp = show_color(&player.max_hp.to_string(), Color::YELLOW);
        let max_mp = show_color(&player.max_mp.to_string(), Color::YELLOW);

        let hpframe = r"
    ┌─── ".to_owned() + &name + "状态────────────┬───────────────────┐
    │【精神】 "+ &mp +"     / "+&max_mp +"      [100%]    │【精力】 100     / 100     (+   0)    │
    │【气血】 "+ &hp +"      / "+&max_hp+"      [100%]    │【内力】 141     / 71      (+   0)    │
    │【真气】 0       / 0        [  0%]    │【战意】 100%               [正常]    │
    │【食物】 0       / 300      [饥饿]    │【潜能】 5075                         │
    │【饮水】 0       / 300      [饥渴]    │【经验】 830                          │
    ├───────────────────┴───────────────────┤
    │【状态】 健康                                                                 │
    └──────────────────────────────北大侠客行────┘";

        let val = wrap_message(self.msg.addr, hpframe);
        self.s_service.send(val).unwrap();  
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