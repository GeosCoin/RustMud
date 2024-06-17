use std::{clone, collections::HashMap, fs::read_to_string, net::ToSocketAddrs, ptr::{null, NonNull}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Quest {
    pub id: u32,        //唯一标识
    pub name: String,   //匹配命令 q_look_river
    pub job: String,    //任务
    pub award: String,  //奖励
    pub after: String,  //之后提示
    pub xp: u32,        //经验
    pub sp: u32,        //信心
    pub subquest: HashMap<u32, bool>, //子任务及完成情况
    pub parent: u32,    //父任务
    pub next: u32,      //下一个任务
    pub node: u32,  
    pub completed: bool,  //完成与否
}

impl Quest {
    pub fn new() -> Self {
        Quest {
            id: 0,        //唯一标识
            name: String::from(""),   //匹配命令 q_look_river
            job: String::from(""),    //任务
            award: String::from(""),  //奖励
            after: String::from(""),  //之后提示
            xp: 0,        //经验
            sp: 0,        //信心
            subquest: HashMap::new(), //子任务
            parent: 0,    //父任务
            next: 0,
            node: 0,
            completed: false, 
        }
    }
}

pub fn init_quest() -> HashMap<u32, Quest> {
        
    let mut quests: HashMap<u32, Quest> = HashMap::new();

    let buf = match read_to_string("quests/quests.txt"){
        Ok(a) => a,
        Err(_) => "".to_string(),
    };

    let n_group: Vec<&str> = buf.split("[quest]").collect();

    for n in n_group.iter() {
        if !n.contains("=") {
            continue;
        }

        let mut quest = Quest::new();             
        for i in n.lines() {
            let group: Vec<&str> = i.split("=").collect();
            let key = match group.get(0) {
                Some(a) => a,
                None => "none",
            };
            
            let item = match group.get(1){
                Some(a) => a,
                None => "",
            };

            match key {
                "id" => {quest.id = item.parse().unwrap(); },
                "name" => {quest.name = item.to_string(); },
                "job" => {quest.job = item.to_string(); },
                "award" => {quest.award = item.to_string(); },
                "after" => {quest.after = item.to_string(); },
                "xp" => {quest.xp = item.parse().unwrap(); },
                "sp" => {quest.sp = item.parse().unwrap(); },
                "node" => {quest.node = item.parse().unwrap(); },
                "subquest" => {
                    let subs: Vec<&str> = item.split("|").collect();
                    for sub in subs.iter() {
                        quest.subquest.insert(sub.to_string().parse().unwrap(), false);
                    }
                },       
                "parent" => {quest.parent = item.parse().unwrap();},
                "next" => {quest.next = item.parse().unwrap();},
                _ => (),
            };
        }
        quests.insert(quest.id, quest);
    }
    
    quests

}

