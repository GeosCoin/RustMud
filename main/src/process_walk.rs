use std::collections::HashMap;
use std::net::SocketAddr;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use crate::channel::wrap_message_ext;
use crate::channel::Message;
use crate::channel::MessageType;
use crate::service::LoginInfo;
use crate::{player::Player, process::PostProcess, quest::Quest};

pub struct ProcessWalk<'a> {
    ret_str: String,
    s_service: &'a Sender<String>,
    players: &'a mut HashMap<SocketAddr, Player>,
    login_info: &'a LoginInfo,
    msg: &'a Message,
}

impl<'a> ProcessWalk<'a> {
    pub fn new(
        ret_str: String,
        s_service: &'a Sender<String>,
        players: &'a mut HashMap<SocketAddr, Player>,
        login_info: &'a LoginInfo,
        msg: &'a Message,
    ) -> Self{
        ProcessWalk{
            ret_str,
            s_service,
            players,
            login_info,
            msg,
        }
    }
}

impl<'a> PostProcess for ProcessWalk<'a> {
    fn execute(&mut self) -> String {

        if self.ret_str.contains("e@") || self.ret_str.contains("w@") 
            || self.ret_str.contains("n@") || self.ret_str.contains("s@") 
            || self.ret_str.contains("ne@") || self.ret_str.contains("nw@") 
            || self.ret_str.contains("se@") || self.ret_str.contains("sw@") 
            {
                let new_pos_vec: Vec<&str> = self.ret_str.split("@").collect();
                let new_pos = match new_pos_vec.get(1) {
                    Some(a) => a,
                    None => "0",
                };
                for item in self.players.iter_mut() {
                    if item.1.name == self.login_info.login.login_name {                    
                        item.1.timer_id = 0;
                        if new_pos != "0" {
                            item.1.pos = new_pos.parse().unwrap();
                        }                
                    }
                }
            }
    
        "".to_string()
    }
}