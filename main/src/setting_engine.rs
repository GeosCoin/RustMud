use std::{collections::HashMap, fs::read_to_string, io::Read};

use walkdir::WalkDir;

use crate::utils_parsing::{get_key_pair, get_section_title, to_int};

#[derive(Debug, Clone)]
pub struct XpTable {
    xp_table: Vec<usize>,
}

impl XpTable {
    pub fn new() -> Self {
        XpTable {
            xp_table: Vec::new()
        }
    }

    pub fn get_level_xp(&self, level: usize) -> usize {
        let len = self.xp_table.len();
        if level <= 1 || len == 0 {
            0
        } else if level > len {
            self.xp_table[len - 1]
        } else {
            self.xp_table[level - 1]
        }
    }

    pub fn get_max_level(&self) -> usize {
        self.xp_table.len()
    }
}

#[derive(Debug, Clone)]
pub struct Classes {
    name: String,
    description: String,
    equipment: Vec<u32>, 
    carried: Vec<u32>,
    primary: HashMap<String, u32>, 
    powers: u32,
    actionbar: Vec<u32>,
    default_power_tab: u32,
}

impl Classes {
    pub fn new() -> Self {
        Classes {
            name: String::from(""),
            description: String::from(""),
            equipment: Vec::new(), 
            carried: Vec::new(),
            primary: HashMap::new(), 
            powers: 0,
            actionbar: Vec::new(),
            default_power_tab: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum RoundMethod {
    NO = 0,
    ROUND,
    FLOOR,
    CEIL
}

#[derive(Debug, Clone)]
pub struct Combat {    
    min_resist: f32,
    max_resist: f32,
    min_avoidance: f32,
    max_avoidance: f32,
    min_miss_damage: f32,
    max_miss_damage: f32,
    min_crit_damage: f32,
    max_crit_damage: f32,
    resource_round_method: RoundMethod,
}

impl Combat {
    pub fn new() -> Self {
        Combat {
            min_resist: 0.0,
            max_resist: 0.0,            
            min_avoidance: 0.0,
            max_avoidance: 0.0,
            min_miss_damage: 0.0,
            max_miss_damage: 0.0,
            min_crit_damage: 0.0,
            max_crit_damage: 0.0,
            resource_round_method: RoundMethod::CEIL,
        }
    }

    pub fn load(&mut self) {
        let min_resist = 0;
        let max_resist = 100;        
        let min_avoidance = 0;
        let max_avoidance = 100;
        let min_miss_damage = 0;
        let max_miss_damage = 0;
        let min_crit_damage = 200;
        let max_crit_damage = 200;
        
        let resource_round_method = RoundMethod::ROUND;

        for line in 
         read_to_string("setting/engine/combat.txt").unwrap().lines() {
            let mut key: String= "".to_string();
            let mut val: String= "".to_string();
            get_key_pair(line, &mut key, &mut val);
            let section_title = get_section_title(line);
            if section_title != "" {
                println!("section: {}", section_title);
            }
            println!("{} : {}", key, val);

            let a = "12";
            let b = to_int::<u32>(a);

            println!("{}", b);
        }

        

    }

    pub fn resource_round(resource_val: f32) -> f32 {
        0.0
    }
}

#[derive(Debug, Clone)]
pub struct DamageType {
    id: String,
    name: String,
    name_min: String,
    name_max: String,
    description: String,
    min: String,
    max: String,
}

impl DamageType {
    pub fn new() -> Self {
        DamageType {
            id: String::from(""),
            name: String::from(""),
            name_min: String::from(""),
            name_max: String::from(""),
            description: String::from(""),
            min: String::from(""),
            max: String::from(""),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EngineSetting {
    xp_table: HashMap<u32, XpTable>,
    classes: Classes,
    combat: Combat,
    damage_type: DamageType,
}

pub fn init_xp_table() -> String {
    let xp_table = XpTable::new();

    for entry in WalkDir::new("D:\\mwnd\\RustMud\\setting\\engine")
        .into_iter().filter_map(|e| e.ok())  {
        let filename = entry.file_name().to_str().unwrap();
        let fullpath = entry.path().display().to_string();       

        if fullpath.contains(".txt") {
            let mut read = utils::load_file(&fullpath);
            let mut content = String::new();
            read.read_to_string(&mut content);

        }
    }

    String::from("")
}