use std::{collections::HashMap, net::SocketAddr};

use crossbeam::channel::Sender;

use crate::{channel::Message, player::Player, process::PostProcess, service::LoginInfo};

pub struct ProcessFight<'a> {
    ret_str: String,
    s_service: &'a Sender<String>,
    players: &'a mut HashMap<SocketAddr, Player>,
    login_info: &'a LoginInfo,
    msg: &'a Message,
}

impl<'a> ProcessFight<'a> {
    pub fn new(
        ret_str: String,
        s_service: &'a Sender<String>,
        players: &'a mut HashMap<SocketAddr, Player>,
        login_info: &'a LoginInfo,
        msg: &'a Message,
    ) -> Self{
        ProcessFight{
            ret_str,
            s_service,
            players,
            login_info,
            msg,
        }
    }
}

impl<'a> PostProcess for ProcessFight<'a> {
    fn execute(&mut self) -> String {
        if self.ret_str.contains("fight") {
            let ret_str = self.ret_str.split(" ").collect::<Vec<&str>>();
            let opponent = match ret_str.get(1){
                Some(a) => a,
                None => return "".to_string(),
            };
            let p_hp = match ret_str.get(2){
                Some(a) => a,
                None => return "0".to_string(),
            };
            let o_hp = match ret_str.get(3){
                Some(a) => a,
                None => return "0".to_string(),
            };
            let timer_id = match ret_str.get(4){
                Some(a) => a,
                None => return "0".to_string(),
            };
            for item in self.players.iter_mut() {
                if item.1.name == self.login_info.login.login_name {
                    item.1.hp = p_hp.parse().unwrap();
                    item.1.timer_id = timer_id.parse().unwrap();
                }else if item.1.name == opponent.to_string() {
                    item.1.hp = o_hp.parse().unwrap();
                }
            }
        }
        "".to_string()
    }   
}