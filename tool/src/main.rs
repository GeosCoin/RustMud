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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Player {
    pub name: String,   //名称
    pub level: u32,     //等级
    pub role: Role, //角色
    pub physical: u32,  //物理输出
    pub mental: u32,    //魔法输出
    pub offense: u32,   //攻击，使用武器限制
    pub defence: u32,   //防御，使用盾牌限制
    pub hp: u32,    //血量
    pub mp: u32,    //精神
    pub xp: u32,    //经验
    pub max_hp: u32,    //最大血量
    pub max_mp: u32,    //最大精神
    pub max_xp: u32,    //最大经验
    pub hp_regen: u32,  //血量增量
    pub mp_regen: u32,  //精神增量
    pub accuracy: u32,  //准确度  
    pub avoidance: u32, //规避力
    
    pub timer_id: usize, //定时器ID
}


impl Player {
    pub fn new() -> Self {
        Player{
            name: String::from(""),   
            level: 1,  
            role: Role::Magician,
            physical: 0,
            mental: 0,
            offense: 0,
            defence: 0,
            hp: 0,
            mp: 0,    
            xp: 0,    
            max_hp: 0,
            max_mp: 0,
            max_xp: 0,    
            hp_regen: 0,    
            mp_regen: 0,
            accuracy: 0,
            avoidance: 0,
            timer_id:0
        }
    }
}

fn create_login(){
    let mut logins :Vec<Login> = Vec::new();
    let mut login1 = Login::new();
    let mut login2 = Login::new();

    login1.login_name = "a".to_string();
    login1.password = "1".to_string();

    login2.login_name = "b".to_string();
    login2.password = "1".to_string();

    logins.push(login1);
    logins.push(login2);

    let w_file = utils::create_file("login.json"); 
    serde_json::to_writer(w_file, &logins).unwrap();
}

fn create_user(){
    let mut players: Vec<Player> = Vec::new();
    
    let mut player1 = Player::new();
    player1.name = "a".to_string();
    player1.level = 2;
    player1.hp = 100;
    player1.mp = 100;
    players.push(player1);

    let mut player2 = Player::new();
    player2.name = "b".to_string();
    player2.level = 7;
    player2.hp = 10000;
    player2.mp = 90;
    players.push(player2);
    
    let w_file = utils::create_file("users.json"); 
    serde_json::to_writer(w_file, &players).unwrap();
}

fn main() {    

    //创建登录用户
    create_login();

    //创建用户信息
    create_user();
    
    
}

