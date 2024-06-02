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
use std::time::Duration;
use std::thread;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Login {
    login_name: String,
    password: String,
    b_login:bool,
}

impl Login {
    fn new() -> Self{
        Login {
            login_name: "".to_string(),
            password: "".to_string(),
            b_login: false,
        }
    }
}

pub fn _handle_service(
    sessions: SessionsType,  //共享在线数据
    s_service: Sender<String>,  //发送到socket       
    r_service: Receiver<String>   //service接收数据
){
    let mut logins: HashMap<SocketAddr, Login> = HashMap::new();
    let mut players: HashMap<SocketAddr, Player> = HashMap::new();

    loop {
        match r_service.recv() {
            Ok(a) => {                
                let s_service_clone = s_service.clone();
                on_service(&a, s_service_clone, &sessions, &mut logins);
            },
            Err(s) => {
                println!("{:?}", s);
                thread::sleep(Duration::from_secs(5000));
            }
        }
    }
}

fn do_login(
    s_service: Sender<String>, 
    login: &mut Login,
    msg: Message
) -> Result<u32, Error>
{
    //登录名还未赋值
    if login.login_name.is_empty() {

        //从文件中读取用户
        let user_file = utils::load_file("users.json");
        let logins: Vec<Login> = serde_json::from_reader(user_file).expect("Error: failed to read json file");
        let filter: Vec<&Login> = logins.iter()
            .filter(|item| item.login_name == msg.content)
            .collect();

        //用户存在
        if filter.len() > 0 {
            login.login_name = msg.content;
            let val = wrap_message(msg.addr, "此ID档案已存在,请输入密码:".to_string());
            s_service.send(val).unwrap(); 
            return Ok(1);
        } else { //用户不存在
            let val = wrap_message(msg.addr, "用户不存在".to_string());
            s_service.send(val).unwrap();
            return Ok(2);
        }
    } else {
        //直接就是密码
        login.password = msg.content;

        let user_file = utils::load_file("users.json");
        let logins: Vec<Login> = serde_json::from_reader(user_file).expect("Error: failed to read json file");
        let filter: Vec<&Login> = logins.iter()
            .filter(|item| 
                item.login_name == login.login_name &&
                item.password == login.password
            )
            .collect();
        
        // println!("{} {}", login.login_name, login.password);
        if filter.len() > 0 {
            login.b_login = true;
            let val = wrap_message(msg.addr, "重新连线完毕。".to_string());
            s_service.send(val).unwrap(); 
        } else { //用户不存在
            let val = wrap_message(msg.addr, "密码错误！".to_string());
            s_service.send(val).unwrap();
            return Ok(4);
        }
    }

    Ok(0)
}

//业务处理入口
pub fn on_service(
    message: &str, 
    s_service: Sender<String>, 
    sessions: &SessionsType,
    logins: &mut HashMap<SocketAddr, Login>
){
    let msg: Message = serde_json::from_str(&message).unwrap();

    let login = logins.entry(msg.addr)
                .or_insert(Login::new());
    
    if !login.b_login {
        match (do_login(s_service, login, msg)){
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

    println!("on_service: {}", message);

    //新用户登录
    let mut sessions_ok = sessions.lock().unwrap();
    
    // if message.trim() == "upgrade" {
    //     sessions_ok.entry(addr)
    //         .and_modify(|ctx| {
    //             ctx.player.name = "龙年".to_string();
    //             ctx.player.level = 30;
    //         });
    // } 

    // let player = &sessions_ok.get(&addr).unwrap().player;
    
    // let hpframe = r"
    // ┌─── ".to_owned() + &utils::show_color(&player.name, Color::YELLOW) + "状态────────────┬───────────────────┐
    // │【精神】 "+ &utils::show_color(&player.level.to_string(), Color::RED) +"     / 125      [100%]    │【精力】 100     / 100     (+   0)    │
    // │【气血】 17      / 127      [100%]    │【内力】 141     / 71      (+   0)    │
    // │【真气】 0       / 0        [  0%]    │【战意】 100%               [正常]    │
    // │【食物】 0       / 300      [饥饿]    │【潜能】 5075                         │
    // │【饮水】 0       / 300      [饥渴]    │【经验】 830                          │
    // ├───────────────────┴───────────────────┤
    // │【状态】 健康                                                                 │
    // └──────────────────────────────北大侠客行────┘\n>";

    // if message.trim() == "hp" {
    //     Self::send(session,  &hpframe);
    //     return;
    // }

    // if message.trim() == "l" {
        
    //     let mut other = 
    //         sessions_ok
    //         .iter()
    //         .filter(|p| !(p.0.to_string() == addr.to_string())) ;

    //     println!("{:?}", other.next().unwrap().1.player.name);
    // }

}