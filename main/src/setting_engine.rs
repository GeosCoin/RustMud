use std::{collections::HashMap, fs::read_to_string, io::Read, vec};

use pathfinding::num_traits::ToPrimitive;
use walkdir::WalkDir;

use crate::utils_parsing::{get_key_pair, get_section_title, pop_first_float, pop_first_int, pop_first_string, skip_line, strip_carriage_return, to_float, to_int, trim};

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

    pub fn load(&mut self) {
        self.xp_table.clear();
        self.xp_table.push(0); //占位符

        let filename = "setting/engine/xp_table.txt";
        
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

            if key == "level" {
                let (outs, remains) = pop_first_int(&val);
                let (outs, remains) = pop_first_int(&remains);
                self.xp_table.push(outs.to_usize().unwrap());
            }
        }

        // println!("{}", &self.xp_table[20]);
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
pub struct PrimaryStat {
    id: String,
    name: String,
}

impl PrimaryStat {
    pub fn new() -> Self {
        PrimaryStat {
            id: "".to_string(),
            name: "".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrimaryStats {
    list: Vec<PrimaryStat>
}

impl PrimaryStats {
    pub fn new() -> Self {
        PrimaryStats {
            list: Vec::new()
        }
    }

    pub fn load(&mut self) {
        self.list.clear();

        let filename = "setting/engine/primary_stats.txt";
        
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
                "id" => {
                    let mut ps = PrimaryStat::new();
                    let (outs, remains) = pop_first_string(&val);                    
                    ps.id = outs;
                    self.list.push(ps);
                },
                "name" => {
                    let mut ps = PrimaryStat::new();
                    let last = self.list.pop().unwrap();
                    ps.id = last.id;
                    let (outs, remains) = pop_first_string(&val);                
                    ps.name = outs;
                    self.list.push(ps);
                },
                _ => {},
            }
        }

        // for i in self.list.iter() {
        //     println!("{}-{}", i.id, i.name);
        // }
    }

    pub fn get_index_by_id(&self, id:&str) -> usize {
        let mut i = 0;
        for ps in self.list.iter() {            
            if ps.id == id {
                break;
            }
            i = i + 1;
        }
        i
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

#[derive(Debug, Clone, PartialEq)]
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
        let filename = "setting/engine/combat.txt";
        
        for line in read_to_string(filename).unwrap().lines() {
            if skip_line(line) {
                continue;
            }
            
            let mut key: String= "".to_string();
            let mut val: String= "".to_string();
            get_key_pair(line, &mut key, &mut val);

            if val.is_empty() {
                continue;
            }

            match key.as_str() {
                "avoidance_percent" => {
                    let (outs, remains) = pop_first_float(&val);                
                    self.min_avoidance = outs;
                    let (outs, remains) = pop_first_float(&remains);                
                    self.max_avoidance = outs;
                },
                "resist_percent" => {
                    let (outs, remains) = pop_first_float(&val);                
                    self.min_resist = outs;
                    let (outs, remains) = pop_first_float(&remains);                
                    self.max_resist = outs;
                },
                "miss_damage_percent" => {
                    let (outs, remains) = pop_first_float(&val);                
                    self.min_miss_damage = outs;
                    let (outs, remains) = pop_first_float(&remains);                
                    self.max_miss_damage = outs;
                },
                "crit_damage_percent" => {
                    let (outs, remains) = pop_first_float(&val);                
                    self.min_crit_damage = outs;
                    let (outs, remains) = pop_first_float(&remains);                
                    self.max_crit_damage = outs;
                },
                "resource_round_method" => {
                    match val.as_str() {                        
                        "round" => {
                            self.resource_round_method = RoundMethod::ROUND;
                        },
                        "floor" => {
                            self.resource_round_method = RoundMethod::FLOOR;
                        },
                        "ceil" => {
                            self.resource_round_method = RoundMethod::CEIL;
                        },
                        _ => {
                            self.resource_round_method = RoundMethod::ROUND;
                        }
                    }
                },
                _ => (),
            }
            
        }

        println!("{}-{},{}-{},{}-{},{}-{},{:?}",
                self.min_avoidance,
                self.max_avoidance,
                self.min_resist,
                self.max_resist,
                self.min_miss_damage,
                self.max_miss_damage,
                self.min_crit_damage,
                self.max_crit_damage,
                self.resource_round_method
            );

    }

    pub fn resource_round(&self, resource_val: f32) -> f32 {
        match self.resource_round_method {
            RoundMethod::ROUND => return resource_val.round(),
            RoundMethod::CEIL => return resource_val.ceil(),
            RoundMethod::FLOOR => return resource_val.floor(),
            _ => resource_val
        }
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