use std::collections::HashMap;
use std::io::BufReader;
use serde::{Serialize, Deserialize};

use crate::quest::Quest;

//角色
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum Role {
    Warrior,  //战士
    Ranger,  //游骑兵
    Magician   //法师
}

//地图
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum Map {
    Liuxiu, //柳秀山庄
    Yangzhou, //扬州
}

//技能
pub struct Powers {

}

//箱子
pub struct Inventory {
    pub goods: Vec<u32>  //物品
}

//装备
pub struct Equipment {
    pub head: u32,      //头部
    pub hands: u32,     //手
    pub torso: u32,     //躯干
    pub artifact: u32,      //手工品 例如：珠宝
    pub ring_left: u32,     //左手戒指
    pub ring_right: u32,    //右手戒指
    pub main_hand: u32,     //右手
    pub off_hand: u32,      //左手
    pub feet: u32,          //脚
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Player {
    pub name: String,   //名称
    pub fullname: String, //全名
    pub group_name: String, //同盟名
    pub friends: Vec<String>, //朋友
    pub map: Map,       //地图
    pub pos: u32,       //位置
    pub level: u32,     //等级
    pub role: Role, //角色
    pub physical: u32,  //物理输出
    pub mental: u32,    //魔法输出
    pub offense: u32,   //攻击，使用武器限制
    pub defence: u32,   //防御，使用盾牌限制
    pub hp: i32,    //血量
    pub mp: u32,    //法术能量
    pub fp: u32,    //信心
    pub xp: u32,    //经验
    pub max_hp: u32,    //最大血量
    pub max_mp: u32,    //最大精神
    pub max_fp: u32,    //最大信心
    pub max_xp: u32,    //最大经验
    pub hp_regen: u32,  //血量增量
    pub mp_regen: u32,  //精神增量
    pub accuracy: u32,  //准确度  
    pub avoidance: u32, //规避力
    pub timer_id: usize, //定时器ID
    pub climbing: u32,  //暂停
    pub knocked: u32,  //是否已经敲门 0:否  1:是
    pub opened: u32,   //是否已打开门
    pub sleep: u32,    //是否睡觉
    pub newbie_quest: HashMap<u32, bool>, //任务完成情况
    pub newbie_next: u32,   //下一个向导
    pub newbie_prompt: u32,  //是否提示过
}



impl Player {
    pub fn new() -> Self {
        Player{
            name: String::from("成王败寇"),   
            fullname: String::from(""),
            group_name: String::from(""),
            friends: Vec::new(),
            level: 1,  
            map: Map::Liuxiu,
            pos: 1,
            role: Role::Magician,
            physical: 0,
            mental: 0,
            offense: 0,
            defence: 0,
            hp: 0,
            mp: 0,
            fp: 0,  //faith 信心程度 影响气血和魔法恢复
            xp: 0,    
            max_hp: 100,
            max_mp: 100,
            max_fp: 100,
            max_xp: 0,    
            hp_regen: 0,    
            mp_regen: 0,
            accuracy: 0,
            avoidance: 0,
            timer_id: 0,
            climbing: 0,
            knocked: 0,
            opened: 0,
            sleep: 0,
            newbie_quest: HashMap::new(),
            newbie_next: 1,
            newbie_prompt: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Group {
    pub name: String,   //组名
    pub members: Vec<String>,   //成员
}

impl Group {
    pub fn new() -> Self {
        Group {
            name: String::from(""),
            members: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Groups {
    pub group: Vec<Group>  
}

impl Groups {
    pub fn new() -> Self {
        Groups {
            group: Vec::new()
        }
    }
}

pub fn init_players() -> Vec<Player> {
    //从文件中读取群组
    let users_file = utils::load_file("users.json");
    let players: Vec<Player> = serde_json::from_reader(users_file).expect("Error: failed to read json file");
    players
}

pub fn init_groups() -> Vec<Group> {
    //从文件中读取群组
    let groups_file = utils::load_file("groups.json");
    let gs: Vec<Group> = serde_json::from_reader(groups_file).expect("Error: failed to read json file");
    gs
}
