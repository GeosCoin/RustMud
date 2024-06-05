use std::{collections::HashMap, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, wrap_message_timer, Message, MessageType}, command::Command, player::Player};

pub struct WalkCommand<'a> {
    players: &'a HashMap<SocketAddr, Player>,
    s_service: &'a Sender<String>,
    msg: &'a Message,
    s_combat: &'a Sender<String>
}

impl<'a> WalkCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message,
        s_combat: &'a Sender<String>
        ) -> WalkCommand<'a>  {
            WalkCommand {
            players,
            s_service,
            msg,
            s_combat
        }
    }
}
impl<'a>  Command for WalkCommand<'a>  {
    fn execute(&self) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();

        let val = wrap_message_timer(MessageType::CombatStop,
            self.msg.addr, "stop".to_string(), player.timer_id.to_string());
        self.s_combat.send(val).unwrap();
        self.msg.content.to_string()
    }
}
