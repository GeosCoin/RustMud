use std::{collections::HashMap, fs::read_to_string, net::ToSocketAddrs, ptr::{null, NonNull}};


pub struct Quest {
    pub col: HashMap<(u32, String), String>,
}

impl Quest {
    pub fn new(&self) -> Self {
        Quest {
            col: HashMap::new()
        }
    }

    pub fn init_quest() -> HashMap<(u32, String), String> {
        //从文件中读取写入内存
        let mut quests: HashMap<(u32, String), String> = HashMap::new();

        let buf = match read_to_string("quests/quests.txt"){
            Ok(a) => a,
            Err(_) => "".to_string(),
        };

        for i in buf.lines() {
            // println!("{:?}", i);

            //todo: 
        }

        quests
    }
}