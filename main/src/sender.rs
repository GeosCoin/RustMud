
use crate::{channel::{ServerHandler, SessionType, Sessions, SessionContext, SessionsType}, player};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use crossbeam::channel::Receiver;
use crate::channel::Message;

pub fn _handle_sender(
    sessions: SessionsType,  //共享在线数据
    r_sender: Receiver<String>  //接收发送到socket的数据
){
    loop {
        match r_sender.recv() {
            Ok(a) => {
                // println!("sender: {}", a);
                crate::sender::on_sender(&sessions, a);
            },
            Err(s) => {
                println!("{:?}", s);
                thread::sleep(Duration::from_secs(5000));
            }
        }
    }
}

pub fn on_sender(sessions: &SessionsType, message: String){
    let msg: Message = serde_json::from_str(&message).unwrap();
    let sessions_ok = sessions.lock().unwrap();
    let ctx = match sessions_ok.get(&msg.addr) {
        Some(a) => a,
        None => {return;}
    };
    let mut stream = &ctx.cur_session.0;
    let val = msg.content + "\n>";
    let _ = stream.write(val.as_bytes());

}

