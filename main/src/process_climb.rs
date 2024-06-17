use std::collections::HashMap;
use std::net::SocketAddr;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use crate::channel::wrap_message_ext;
use crate::channel::Message;
use crate::channel::MessageType;
use crate::service::LoginInfo;
use crate::{player::Player, process::PostProcess, quest::Quest};

pub struct ProcessClimb<'a> {
    ret_str: String,
    s_service: &'a Sender<String>,
    players: &'a mut HashMap<SocketAddr, Player>,
    login_info: &'a LoginInfo,
    msg: &'a Message,
}

impl<'a> ProcessClimb<'a> {
    pub fn new(
        ret_str: String,
        s_service: &'a Sender<String>,
        players: &'a mut HashMap<SocketAddr, Player>,
        login_info: &'a LoginInfo,
        msg: &'a Message,
    ) -> Self{
        ProcessClimb{
            ret_str,
            s_service,
            players,
            login_info,
            msg,
        }
    }
}

impl<'a> PostProcess for ProcessClimb<'a> {
    fn execute(&mut self) -> String {

        if self.ret_str.contains("climbing") {
            let pendings: Vec<&str> = self.ret_str.split(" ").collect();
            let pending_status = match pendings.get(1) {
                Some(a) => a,
                None => "0",
            };
            let new_pos = match pendings.get(3) {
                Some(a) => a,
                None => "0",
            };
            
            for item in self.players.iter_mut() {
                if item.1.name == self.login_info.login.login_name {                    
                    item.1.climbing = pending_status.parse().unwrap(); 
    
                    if new_pos != "0" {
                        item.1.pos = new_pos.parse().unwrap();
                    }                        
                }
            }
        }
    
        if self.ret_str.contains("knocked") 
        || self.ret_str.contains("opened")
        || self.ret_str.contains("sleep") {
            let knockeds: Vec<&str> = self.ret_str.split(" ").collect();
            let knocked_status = match knockeds.get(1) {
                Some(a) => a,
                None => "0",
            };
            
            for item in self.players.iter_mut() {
                if item.1.name == self.login_info.login.login_name {  
                    if self.ret_str.contains("knocked") {                  
                        item.1.knocked = knocked_status.parse().unwrap();          
                    }else if self.ret_str.contains("opened") {
                        item.1.opened = knocked_status.parse().unwrap();          
                    }else if self.ret_str.contains("sleep") {
                        item.1.sleep = knocked_status.parse().unwrap(); 
                    }
                }
            }
        }
        "".to_string()
    }
}