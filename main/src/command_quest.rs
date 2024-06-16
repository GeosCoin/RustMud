use std::{collections::HashMap, net::SocketAddr, rc::Rc};
use crossbeam::channel::Sender;
use utils::{show_color, Color};
use crate::{channel::{wrap_message, Message}, command::Command, player::Player, quest::{self, Quest}};

pub struct QuestCommand<'a> {
    pub players: &'a HashMap<SocketAddr, Player>,
    pub s_service: &'a Sender<String>,
    pub msg: &'a Message,
    pub quests: &'a HashMap<u32, Quest>
}

impl<'a> QuestCommand<'a> {
    pub fn new(
        players: &'a HashMap<SocketAddr, Player>,
        s_service: &'a Sender<String>,
        msg: &'a Message,
        quests: &'a HashMap<u32, Quest>
        ) -> Self  {
            QuestCommand {
            players,
            s_service,
            msg,
            quests
        }
    }

    pub fn do_jobquery(&self, player: &Player, quests: &HashMap<u32, Quest>) -> String {        
        println!("player.newbie_next = {}", player.newbie_next);
        
        let quest = match quests.get(&player.newbie_next){
            Some(a) => a,
            None => {
                let val = wrap_message(self.msg.addr, "当前没有任务".to_string());
                self.s_service.send(val).unwrap();  
                return "jobquery".to_string();
            },
        };

        let val = wrap_message(self.msg.addr, quest.job.to_string());
        self.s_service.send(val).unwrap();  
        "jobquery".to_string() 
    }

}
impl<'a>  Command for QuestCommand<'a>  {
    
    fn execute(&self) -> String {
        let player = self.players.get(&self.msg.addr).unwrap();

        let cmd = self.msg.content.to_ascii_lowercase();
        match cmd.as_str() {
            "jq" | "jobquery" => {return QuestCommand::<'a>::do_jobquery(&self, player, self.quests)},            
            _ => {return "ok".to_string();}
        }
        
    }
}

