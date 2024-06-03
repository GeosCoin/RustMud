use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use crate::channel::wrap_message;
use crate::channel::Message;
use crate::player::Player;
use crate::{channel::{ServerHandler, SessionType, Sessions, SessionContext, SessionsType}, player};
use std::collections::HashMap;
use std::fmt::Error;
use std::hash::Hash;
use std::net::SocketAddr;
use std::ops::Add;
use std::time::Duration;
use std::thread;
use serde::{Serialize, Deserialize};
use utils;
use std::cell::RefCell;
use std::rc::Rc;
use std::net::{SocketAddrV4, Ipv4Addr};

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

pub struct LoginInfo {
    pub login: Login,
    pub b_login: bool,
}

impl LoginInfo {
    fn new() -> Self{
        LoginInfo {
            login: Login::new(),
            b_login: false,
        }
    }
}

pub fn _handle_service(
    sessions: SessionsType,  //共享在线数据
    s_service: Sender<String>,  //发送到socket       
    r_service: Receiver<String>,   //service接收数据
    s_timer: Sender<String>, //发送到定时器模块
){
    let mut login_infos: HashMap<SocketAddr, LoginInfo> = HashMap::new();
    let mut players: HashMap<SocketAddr, Player> = HashMap::new();

    loop {
        match r_service.recv() {
            Ok(a) => {                
                let s_service_clone = s_service.clone();
                let s_timer_clone = s_timer.clone();
                on_service(&a, s_service_clone, s_timer_clone, &sessions, 
                    &mut login_infos,
                    &mut players);
            },
            Err(s) => {
                println!("{:?}", s);
                thread::sleep(Duration::from_secs(5000));
            }
        }
    }
}

//业务处理入口
pub fn on_service(
    message: &str, 
    s_service: Sender<String>, 
    s_timer: Sender<String>,
    sessions: &SessionsType,
    login_infos: &mut HashMap<SocketAddr, LoginInfo>,
    players: &mut HashMap<SocketAddr, Player>
){
    println!("on_service: {}", message);

    let msg: Message = serde_json::from_str(&message).unwrap();

    let login_info = login_infos.entry(msg.addr)
                .or_insert(LoginInfo::new());    
    
    if !login_info.b_login {
        let player = players.entry(msg.addr)
            .or_insert(Player::new());
        match crate::login::do_login(&s_service, login_info, player, &msg) {
            Ok(a) => {
                if a != 0 {
                    return;
                }
            },
            Err(_) => {
                return;
            }
        };
    }

    //显示用户信息    
    if msg.content == "hp" {
        let player = players.entry(msg.addr)
            .or_insert(Player::new());
        let hpframe = r"
    ┌─── ".to_owned() + &utils::show_color(&player.name, utils::Color::YELLOW) + "状态────────────┬───────────────────┐
    │【精神】 "+ &utils::show_color(&player.level.to_string(), utils::Color::RED) +"     / 125      [100%]    │【精力】 100     / 100     (+   0)    │
    │【气血】 "+&utils::show_color(&player.hp.to_string(), utils::Color::YELLOW) +"      / 127      [100%]    │【内力】 141     / 71      (+   0)    │
    │【真气】 0       / 0        [  0%]    │【战意】 100%               [正常]    │
    │【食物】 0       / 300      [饥饿]    │【潜能】 5075                         │
    │【饮水】 0       / 300      [饥渴]    │【经验】 830                          │
    ├───────────────────┴───────────────────┤
    │【状态】 健康                                                                 │
    └──────────────────────────────北大侠客行────┘\n>";

        let val = wrap_message(msg.addr, hpframe);
        s_service.send(val).unwrap();        
        return;
    }

    //显示周边环境
    if msg.content == "l" || msg.content == "look" {
        // let player = players.entry(msg.addr)
        //     .or_insert(Player::new());
        let mut l_view = r"
        未明谷 -  
                    树林----未明谷----乱石阵    
                              ｜     
                           青石桥头             
    山谷中绿树成荫，却不见有多么明媚的花开于此，但你仍能闻见了远远飘来的花香。耳边听到了溪
水叮咚的声音，原来不远处有一条蜿蜒的小溪(river)，岸边似乎散落了一些物什。在山谷的北侧有条陡
峭的山坡(path)隐隐可以通向外界。
    你可以看看(look):river,path，。
    「初春」: 太阳无奈地缓缓挂向西边的才露新芽的树梢。

    这里明显的方向有 south、east 和 west。

    二个葫芦(Hu lu)
    二枚野果(Ye guo)
    普通百姓 博迪鸟(Birddy) 
    普通百姓 翻炒西瓜拌面(Esther) 
    ".to_string();

        let others: Vec<(&SocketAddr, &Player)> = players.iter()
            .filter(|p| p.1.name != login_info.login.login_name)
            .collect();
        let mut names = String::from("");
        for p in others {
            names = names
                 + "普通百姓 " + &p.1.name + "\n";
        }
        l_view = l_view + &names;
        let val = wrap_message(msg.addr, l_view.to_string());
        s_service.send(val).unwrap();        
        return;
    }

    //Fight指定玩家
    if msg.content.contains("fight") {
        let multi = msg.content.split(" ").collect::<Vec<&str>>();
        let opponent = match multi.get(1) {
            Some(a) => a,
            None => {
                println!("Do not know fight who?");
                return;
            }
        };

        print!("{}", opponent);

        let mut player = &mut Player::new();
        let mut o_player = &mut Player::new();
        let mut addr = &SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(0,0,0,0), 
            0));
        for item in players.iter_mut() {
            if item.1.name == login_info.login.login_name {
                player = item.1;
            }else if item.1.name == opponent.to_string() {
                o_player = item.1;
                addr = item.0;
            }
        }
        
        player.hp = player.hp - 7;
        o_player.hp = o_player.hp - 10;

        //向对手叫板
        let val = wrap_message(msg.addr,
                "你对着".to_owned() + &o_player.name +"吼道：「畜生！你死期已到，今天就让小爷我送你上西天吧！」");
        s_service.send(val).unwrap();

        //对手收到叫板
        let val = wrap_message(*addr,
            player.name.to_owned() + "对着你吼道：「畜生！你死期已到，今天就让小爷我送你上西天吧！」");
        s_service.send(val).unwrap();
        
        //启动定时器
        let val = wrap_message(*addr,
            "0".to_string());
        s_timer.send(val).unwrap();
        
        
    }

    if msg.content.contains("close") {
        let val = wrap_message(msg.addr,
            "10".to_string());
        s_timer.send(val).unwrap();
    }

}