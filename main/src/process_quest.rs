use std::collections::HashMap;
use std::net::SocketAddr;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use crate::channel::wrap_message_ext;
use crate::channel::Message;
use crate::channel::MessageType;
use crate::service::LoginInfo;
use crate::{player::Player, process::PostProcess, quest::Quest};

pub struct ProcessQuest<'a> {
    ret_str: String,
    quests: &'a HashMap<u32, Quest>,
    s_service: &'a Sender<String>,
    players: &'a mut HashMap<SocketAddr, Player>,
    login_info: &'a LoginInfo,
    msg: &'a Message,
}

impl<'a> ProcessQuest<'a> {
    pub fn new(
        ret_str: String,
        quests: &'a HashMap<u32, Quest>,
        s_service: &'a Sender<String>,
        players: &'a mut HashMap<SocketAddr, Player>,
        login_info: &'a LoginInfo,
        msg: &'a Message,
    ) -> Self{
        ProcessQuest{
            ret_str,
            quests,
            s_service,
            players,
            login_info,
            msg,
        }
    }
}

impl<'a> PostProcess for ProcessQuest<'a> {
    fn execute(&mut self) -> String {
        
        let mut quest_ret = self.ret_str.clone();    
        if quest_ret.contains("e@") {
            quest_ret = "east".to_string();
        } else if quest_ret.contains("w@") {
            quest_ret = "west".to_string();
        } else if quest_ret.contains("s@") {
            quest_ret = "south".to_string();
        } else if quest_ret.contains("n@") {
            quest_ret = "north".to_string();
        }

        quest_ret = "q_".to_owned() + &quest_ret;
        println!("quest_ret: {:?}", quest_ret);

        let player = match self.players.get(&self.msg.addr) {
            Some(a) => a,
            None => return "99".to_string(),
        };

        let quest: Vec<(&u32, &Quest)> = self.quests.iter().filter(|p| p.1.name == quest_ret).collect();
        if !quest.is_empty() {
            
            let id = quest[0].1.id;
            let is_id_exist = match player.newbie_quest.get(&id){
                Some(a) => true,
                None => false,
            };

            //如果不存在，表示有新任务
            if !is_id_exist {

                let parent = quest[0].1.parent;
                let node = quest[0].1.node;
                let next = quest[0].1.next;
                println!("next: {:?} node: {}", next, node);
                let xp = quest[0].1.xp;
                let sp = quest[0].1.sp;
                let award = &quest[0].1.award;
                let after = &quest[0].1.after;
                
                
                //如果没有父项，才会通知
                if parent == 0 {
                    let val = wrap_message_ext(MessageType::NoPrompt,self.msg.addr, award.to_string());
                    self.s_service.send(val).unwrap();    

                    let val = wrap_message_ext(MessageType::NoPrompt,self.msg.addr, after.to_string());
                    self.s_service.send(val).unwrap();
                }

                let new_pos_vec: Vec<&str> = self.ret_str.split("@").collect();
                let new_pos = match new_pos_vec.get(1) {
                    Some(a) => a,
                    None => "0",
                };
                let new_pos: u32 = new_pos.parse().unwrap();

                for item in self.players.iter_mut() {
                    if item.1.name == self.login_info.login.login_name {  
                        println!("{}{}", node, new_pos);
                        if node == new_pos {
                            item.1.newbie_quest.insert(id, true); 
                        }
                        if parent == 0 {           
                            item.1.newbie_next = next;                    
                            item.1.xp += xp;
                            item.1.sp += sp;
                        }

                        //如果有父项，判断父项下的子项是否已经全部满足
                        if parent != 0 {
                            println!("parent= {}", parent);
                            let quest = match self.quests.get(&parent) {
                                Some(a) => a,
                                None => return "".to_string(),
                            };
                            
                            let subquest = &quest.subquest;
                            let mut all_exist = true;
                            for sub in subquest.iter() {
                                println!("sub: {:?}", sub.0);
                                let is_exist = match item.1.newbie_quest.get(&sub.0){
                                    Some(a) => true,
                                    None => false,
                                };
                                if !is_exist {
                                    all_exist = false;
                                    break;
                                }
                            }
                            
                            if all_exist {
                                item.1.newbie_quest.insert(parent, true);
                                item.1.newbie_next = quest.next;                    
                                item.1.xp += quest.xp;
                                item.1.sp += quest.sp;

                                let val = wrap_message_ext(MessageType::NoPrompt,self.msg.addr, quest.award.to_string());
                                self.s_service.send(val).unwrap();    

                                let val = wrap_message_ext(MessageType::NoPrompt,self.msg.addr, quest.after.to_string());
                                self.s_service.send(val).unwrap();
                            }
                        }
                    }
                }

            }
        }
        "".to_string()
    }
}