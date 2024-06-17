use crate::channel::SessionsType;
use crate::player::Player;
use crate::quest::Quest;
use crate::service::Login;
use crate::service::LoginInfo;
use crossbeam::channel::Sender;
use crate::channel::Message;
use crate::channel::wrap_message;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::num::ParseIntError;

pub struct LoginService<'a> {
    s_service: &'a Sender<String>, 
    login_info: &'a mut LoginInfo,
    msg: &'a Message,
    players: &'a mut HashMap<SocketAddr, Player>,
    sessions:  &'a SessionsType,
    quests: &'a HashMap<u32, Quest>
}

impl<'a> LoginService<'a> {
    pub fn new(
        s_service: &'a Sender<String>, 
        login_info: &'a mut LoginInfo,
        msg: &'a Message,
        players: &'a mut HashMap<SocketAddr, Player>,
        sessions:  &'a SessionsType,
        quests: &'a HashMap<u32, Quest>
    ) -> Self {
        LoginService {
            s_service, 
            login_info,
            msg,
            players,
            sessions,
            quests
        }
    }

    pub fn do_login(&mut self) -> u32 {

        let mut ps  =  self.players.clone();

        let player = self.players.entry(self.msg.addr)
            .or_insert(Player::new());

        let player_clone = player.clone();

        //登录名还未赋值
        if self.login_info.login.login_name.is_empty() {

            //从文件中读取用户
            let user_file = utils::load_file("login.json");
            let logins: Vec<Login> = serde_json::from_reader(user_file).expect("Error: failed to read json file");
            let filter: Vec<&Login> = logins.iter()
                .filter(|item| item.login_name == self.msg.content)
                .collect();

            //用户存在
            if filter.len() > 0 {
                self.login_info.login.login_name = self.msg.content.to_string();
                let val = wrap_message(self.msg.addr, "此ID档案已存在,请输入密码:".to_string());
                self.s_service.send(val).unwrap(); 
                return 1;
            } else { //用户不存在
                let val = wrap_message(self.msg.addr, "用户不存在".to_string());
                self.s_service.send(val).unwrap();
                return 2;
            }
        } else {
            //直接就是密码
            self.login_info.login.password = self.msg.content.to_string();

            let user_file = utils::load_file("login.json");
            let logins: Vec<Login> = serde_json::from_reader(user_file).expect("Error: failed to read json file");
            let filter: Vec<&Login> = logins.iter()
                .filter(|item| 
                    item.login_name == self.login_info.login.login_name &&
                    item.password == self.login_info.login.password
                )
                .collect();
            
            // println!("{} {}", login.login_name, login.password);
            if filter.len() > 0 {
                self.login_info.b_login = true;
                let val = wrap_message(self.msg.addr, "重新连线完毕。".to_string());
                self.s_service.send(val).unwrap(); 

                //获取用户资料
                let user_file = utils::load_file("users.json");
                let users: Vec<Player> = serde_json::from_reader(user_file).expect("Error: failed to read json file");
                let filter: Vec<&Player> = users.iter()
                    .filter(|item| item.name == self.login_info.login.login_name)
                    .collect();

                let filter_player = filter[0].clone();
                *player = filter_player;

            } else { //用户不存在
                let val = wrap_message(self.msg.addr, "密码错误！".to_string());
                self.s_service.send(val).unwrap();            
                return 4;
            }
        }

        let p_vec : Vec<(&SocketAddr, &Player)> = ps.iter()
            .filter(|p| p.1.name == self.login_info.login.login_name)
            .collect();
        if !p_vec.is_empty() {
            println!("{}", p_vec.len());
            let val = wrap_message(self.msg.addr, 
                "此用户已在服务器上登录过，其将被强制退出。".to_string());
                self.s_service.send(val).unwrap(); 

            //删除已登录用户
            let sessions_login = self.sessions.lock().unwrap();
            
            for p in p_vec.iter() {
                //先删除，后面stream不一定能得到
                let addr = p.0;
                self.players.remove(addr);

                let stream = match sessions_login.get(p.0) {
                    Some(a) => a,
                    None => continue
                };
                let _ = stream.cur_session.0.shutdown(std::net::Shutdown::Both); 
                
            }
            println!(" Connect count = {}", sessions_login.len());

            return 99;
        }

        //已登录并且新手向导存在
        if self.login_info.b_login && player_clone.newbie_next != 0 {
            //登录时就进行首轮提示                    
            let quest = match self.quests.get(&player_clone.newbie_next){
                Some(a) => a,
                None => return 99,
            };
    
            //提示
            let val = wrap_message(self.msg.addr, quest.job.to_string());
            self.s_service.send(val).unwrap();                     
            return 0;
        }

        return 0;
    }

}
