use std::{fs::read_to_string, sync::Mutex};
use once_cell::sync::OnceCell;
use crate::{log::{error, info, warn}, utils_parsing::{get_key_pair, pop_first_string, skip_line}};

pub static SETTINGS: OnceCell<Mutex<Settings>> = OnceCell::new();

pub fn init_settings() {    
    SETTINGS.get_or_init(|| Mutex::new(Settings::new()));    
}

pub fn load_settings(){
    SETTINGS.get().unwrap().lock().unwrap().load()
}

/// Just for Demo: implement mutable singleton 
pub fn set_name(s: &str){
    SETTINGS.get().unwrap().lock().unwrap().set_game(s);
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub game: String,
    pub description: String,
    pub version: String,
    pub language: String
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            game: "".to_string(),
            description: "".to_string(),
            version: "".to_string(),
            language: "".to_string(),
        }
    }

    pub fn set_game(&mut self, s:&str) {
        self.game = s.to_string();
    }

    pub fn load(&mut self) {
        let filename = "setting/settings.txt";
        
        for line in read_to_string(filename).unwrap().lines() {
            if skip_line(line) {
                continue;
            }
            
            let mut key = "".to_string();
            let mut val = "".to_string();
            get_key_pair(line, &mut key, &mut val);

            if val.is_empty() {
                continue;
            }

            match key.as_str() {
                "game" => {                    
                    let (outs, remains) = pop_first_string(&val);                    
                    self.game = outs;
                },
                "description" => {
                    let (outs, remains) = pop_first_string(&val);                    
                    self.description = outs;
                },
                "version" => {
                    let (outs, remains) = pop_first_string(&val);                    
                    self.version = outs;
                },
                "language" => {
                    let (outs, remains) = pop_first_string(&val);                    
                    self.language = outs;
                },
                _ => {},
            }
            
        }
        
        info("初始化Settings成功");
    }
}
