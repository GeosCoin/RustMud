use std::{collections::HashMap, io::Read};

use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct XpTable {
    level: u32,
    upgrade_xp: u32,
}

impl XpTable {
    pub fn new() -> Self {
        XpTable {
            level: 0,
            upgrade_xp: 0,
        }
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
    min_absorb: f32,
    max_absorb: f32,
    min_resist: f32,
    max_resist: f32,
    min_block: f32,
    max_block: f32,
    min_avoidance: f32,
    max_avoidance: f32,
    min_miss_damage: f32,
    max_miss_damage: f32,
    min_crit_damage: f32,
    max_crit_damage: f32,
    min_overhit_damage: f32,
    max_overhit_damage: f32,
    resource_round_method: RoundMethod,
}

impl Combat {
    pub fn new() -> Self {
        Combat {
            min_absorb: 0.0,
            max_absorb: 0.0,
            min_resist: 0.0,
            max_resist: 0.0,
            min_block: 0.0,
            max_block: 0.0,
            min_avoidance: 0.0,
            max_avoidance: 0.0,
            min_miss_damage: 0.0,
            max_miss_damage: 0.0,
            min_crit_damage: 0.0,
            max_crit_damage: 0.0,
            min_overhit_damage: 0.0,
            max_overhit_damage: 0.0,
            resource_round_method: RoundMethod::CEIL,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EngineSetting {
    xp_talbe: HashMap<u32, XpTable>,
    classes: Classes,
    combat: Combat,
}

pub fn init_xp_table() -> Result<HashMap<u32, u32>, _> {
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

    Ok(factory)
}