use std::{collections::HashMap, net::{Ipv4Addr, SocketAddr, SocketAddrV4}, rc::Rc};
use crossbeam::channel::Sender;
use utils::{get_id, show_color, Color};
use crate::{channel::{wrap_message, wrap_message_ext, wrap_message_timer, Message, MessageType}, command::Command, player::Player};

pub struct FightCommand<'a> {
    players: &'a HashMap<SocketAddr, Player>,
    s_service: &'a Sender<String>,
    msg: &'a Message,
    s_combat: &'a Sender<String>
}

impl<'a> FightCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message,
        s_combat: &'a Sender<String>
        ) -> FightCommand<'a>  {
            FightCommand {
            players,
            s_service,
            msg,
            s_combat
        }
    }
}

impl<'a>  Command for FightCommand<'a>  {
    fn execute(&self) -> String {
        let player = match self.players.get(&self.msg.addr){
            Some(a) => a,
            None => {return "none".to_string()},
        };

        let multi = self.msg.content.split(" ").collect::<Vec<&str>>();
        let opponent = match multi.get(1) {
            Some(a) => a,
            None => {
                println!("Do not know fight who?");
                return "no".to_string();
            }
        };

        //自己不能打自己
        if player.name == opponent.to_string() {
            let val = wrap_message(self.msg.addr,
                "自己不能fight自己".to_string());
            self.s_service.send(val).unwrap();
            return "deny fight yourself".to_string();
        }

        //对手不能是空的 不在当前位置
        let opponent_vec: Vec<(&SocketAddr, &Player)> = self.players.iter()
            .filter(|p| p.1.name == opponent.to_string() && p.1.pos == player.pos)
            .collect();
        if opponent_vec.is_empty() {
            let val = wrap_message(self.msg.addr,
                "你要fight谁?".to_string());
            self.s_service.send(val).unwrap();
            return "fight who?".to_string();
        }

        // e.g fight b/ fight b continue
        // continue is the action
        // otherwise there is "0"
        let action = match multi.get(2) {
            Some(a) => a,
            None => "0"
        };

        print!("{}", opponent);

        let timer_id = &self.msg.timer_id;
        let mut timer_id = match timer_id.parse() {
            Ok(a) => a,
            Err(_) => 0
        };
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

        if action == "0" {
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
            timer_id = get_id();

            let val = wrap_message_timer(MessageType::CombatStart,
                self.msg.addr, self.msg.content.to_string(),
                timer_id.to_string());
            self.s_combat.send(val).unwrap();
        } else {  // action == "continue"
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
            + " " + &timer_id.to_string()
    }
}