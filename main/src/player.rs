use serde::{Serialize, Deserialize};

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
    pub map: Map,       //地图
    pub pos: u32,       //位置
    pub level: u32,     //等级
    pub role: Role, //角色
    pub physical: u32,  //物理输出
    pub mental: u32,    //魔法输出
    pub offense: u32,   //攻击，使用武器限制
    pub defence: u32,   //防御，使用盾牌限制
    pub hp: i32,    //血量
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
            name: String::from("成王败寇"),   
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
            xp: 0,    
            max_hp: 0,
            max_mp: 0,
            max_xp: 0,    
            hp_regen: 0,    
            mp_regen: 0,
            accuracy: 0,
            avoidance: 0,
            timer_id: 0,
        }
    }
}
