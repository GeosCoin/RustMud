use std::{collections::HashMap, net::{Ipv4Addr, SocketAddr, SocketAddrV4}, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, wrap_message_ext, Message, MessageType}, command::Command, player::Player};

pub struct FightCommand<'a> {
    players: &'a HashMap<SocketAddr, Player>,
    s_service: &'a Sender<String>,
    msg: &'a Message,
    s_timer: &'a Sender<String>,
}

impl<'a> FightCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message,
        s_timer: &'a Sender<String>,
        ) -> FightCommand<'a>  {
            FightCommand {
            players,
            s_service,
            msg,
            s_timer
        }
    }
}
impl<'a>  Command for FightCommand<'a>  {
    fn execute(&self) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();

        let multi = self.msg.content.split(" ").collect::<Vec<&str>>();
        let opponent = match multi.get(1) {
            Some(a) => a,
            None => {
                println!("Do not know fight who?");
                return "no".to_string();
            }
        };

        if player.name == opponent.to_string() {
            let val = wrap_message(self.msg.addr,
                "自己不能fight自己".to_string());
            self.s_service.send(val).unwrap();
            return "deny fight yourself".to_string();
        }

        let timer = match multi.get(2) {
            Some(a) => a,
            None => "0"
        };

        print!("{}", opponent);

        let mut o_player = &Player::new();
        let mut addr = &SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(0,0,0,0), 
            0));
        for item in self.players.iter() {
            if item.1.name == opponent.to_string() {
                o_player = item.1;
                addr = item.0;
            }
        }
        
        let p_hp = player.hp - 3;
        let o_hp = o_player.hp - 7;

        if timer == "0" {
            //向对手叫板
            let val = wrap_message(self.msg.addr,
                    "你对着".to_owned() + &o_player.name +"吼道：「畜生！你死期已到，今天就让小爷我送你上西天吧！」");
            self.s_service.send(val).unwrap();

            //对手收到叫板
            let val = wrap_message(*addr,
                player.name.to_owned() + "对着你吼道：「畜生！你死期已到，今天就让小爷我送你上西天吧！」");
            self.s_service.send(val).unwrap();
            
            //启动定时器
            //todo: 每个用户一个定时器，已有定时器，需要先关闭，然后再修改原定时器
            
            let val = wrap_message_ext(MessageType::Combat,
                self.msg.addr, self.msg.content.to_string());
            self.s_timer.send(val).unwrap();
        } else {
            //向对手叫板
            let val = wrap_message(self.msg.addr,
                "你对着".to_owned() + &o_player.name +"你在攻击中不断积蓄攻势。(气势：8%)");
            self.s_service.send(val).unwrap();

            //对手收到叫板
            let val = wrap_message(*addr,
            player.name.to_owned() + "( 野兔动作似乎开始有点不太灵光，但是仍然有条不紊。 )");
            self.s_service.send(val).unwrap();
        }
        
        "fight ".to_string()+ opponent + " "
            +&p_hp.to_string()+" "+&o_hp.to_string()
    }
}