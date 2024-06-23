#![allow(warnings)]

use std::{collections::HashMap, fs::read_to_string, io::Read};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Login {
    pub login_name: String,
    pub password: String,
}

impl Login {
    fn new() -> Self{
        Login {
            login_name: "".to_string(),
            password: "".to_string(),
        }
    }
}

//角色
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Player {
    pub name: String,   //名称
    pub fullname: String,
    pub group_name: String,
    pub map: Map,       //地图
    pub pos: u32,      //位置
    pub level: u32,     //等级
    pub role: Role, //角色
    pub physical: u32,  //物理输出
    pub mental: u32,    //魔法输出
    pub offense: u32,   //攻击，使用武器限制
    pub defence: u32,   //防御，使用盾牌限制
    pub hp: u32,    //血量
    pub mp: u32,    //精神
    pub fp: u32,
    pub xp: u32,    //经验
    pub max_hp: u32,    //最大血量
    pub max_mp: u32,    //最大精神
    pub max_fp: u32,
    pub max_xp: u32,    //最大经验
    pub hp_regen: u32,  //血量增量
    pub mp_regen: u32,  //精神增量
    pub accuracy: u32,  //准确度  
    pub avoidance: u32, //规避力
    
    pub timer_id: usize, //定时器ID
    pub climbing: usize, 
    pub knocked: usize,
    pub opened: u32,
    pub sleep: u32,
    pub newbie_quest: HashMap<u32, bool>,
    pub newbie_next: u32, 
    pub newbie_prompt: u32,
}


impl Player {
    pub fn new() -> Self {
        Player{
            name: String::from(""),   
            fullname: String::from(""),
            group_name: String::from(""),
            map: Map::Liuxiu,
            pos: 14,
            level: 1,  
            role: Role::Magician,
            physical: 0,
            mental: 0,
            offense: 0,
            defence: 0,
            hp: 0,
            mp: 0,
            fp: 0,    
            xp: 0,    
            max_hp: 100,
            max_mp: 150,
            max_fp: 200,
            max_xp: 3000,    
            hp_regen: 0,    
            mp_regen: 0,
            accuracy: 0,
            avoidance: 0,
            timer_id:0,
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

fn create_login(){
    let mut logins :Vec<Login> = Vec::new();
    let mut login1 = Login::new();
    let mut login2 = Login::new();
    let mut login3 = Login::new();

    login1.login_name = "a".to_string();
    login1.password = "1".to_string();

    login2.login_name = "b".to_string();
    login2.password = "1".to_string();

    login3.login_name = "c".to_string();
    login3.password = "1".to_string();

    logins.push(login1);
    logins.push(login2);
    logins.push(login3);

    let w_file = utils::create_file("login.json"); 
    serde_json::to_writer(w_file, &logins).unwrap();
}

fn create_user(){
    let mut players: Vec<Player> = Vec::new();
    
    let mut player1 = Player::new();
    player1.name = "a".to_string();
    player1.fullname = "西门吹雪".to_string();
    player1.level = 2;
    player1.hp = 100;
    player1.mp = 100;
    players.push(player1);

    let mut player2 = Player::new();
    player2.name = "b".to_string();
    player2.fullname = "小茶茶".to_string();
    player2.level = 7;
    player2.hp = 10000;
    player2.mp = 90;
    players.push(player2);

    let mut player2 = Player::new();
    player2.name = "c".to_string();
    player2.fullname = "西山老妖".to_string();
    player2.level = 7;
    player2.hp = 1000;
    player2.mp = 100;
    players.push(player2);
    
    let w_file = utils::create_file("users.json"); 
    serde_json::to_writer(w_file, &players).unwrap();
}

fn main() {    

    //创建登录用户
    create_login();

    //创建用户信息
    create_user();
    
    //读取地图
    // read_node();
}

fn read_node(){
    let mut read = utils::load_file("maps/liuxiu_1.txt");
    let mut buf = String::new();
    read.read_to_string(&mut buf);

}

