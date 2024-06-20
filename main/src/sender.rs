
use crate::{channel::{MessageType, ServerHandler, SessionContext, SessionType, Sessions, SessionsType}, player};
use std::{net::{SocketAddr, TcpListener, TcpStream}, str::Bytes};
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
    println!("on_sender: {:?}", message);
    let msg: Message = serde_json::from_str(&message).unwrap();
    let sessions_ok = sessions.lock().unwrap();
    let ctx = match sessions_ok.get(&msg.addr) {
        Some(a) => a,
        None => {return;}
    };
    let mut stream = &ctx.cur_session.0;

    //没有提示符
    if msg.msg_type == MessageType::NoPrompt {
        let val = msg.content.to_owned() + "\n" ;
        let _ = stream.write(val.as_bytes() );
        return;
    }

    if msg.msg_type == MessageType::IacDoTerm {
        let tail: &[u8; 3] = &[0xff, 0xfd, 0x18];
        let val = msg.content.as_bytes();
        let mut buff = [val, tail].concat(); 
        let _ = stream.write(&buff.as_slice() );
         
        return;
    }

    if msg.msg_type == MessageType::IacWillGmcp {
        // let tail: &[u8; 3] = &[0xff, 0xfb, 0xc9];
        let val = msg.content.as_bytes();
        let mut buff = [val].concat(); 
        let _ = stream.write(&buff.as_slice() );
         
        return;
    }

    if msg.msg_type == MessageType::IacDoGmcp {        
        let val = msg.content.as_bytes();
        let head = &[0xff, 0xfd, 0xc9, 0xff, 0xfa, 0xc9];
        let tail: &[u8; 2] = &[0xff, 0xf0];
        let mut buff = [head, val, tail].concat();        

        let _ = stream.write(&buff.as_slice());
        
        return;
    }

    //正常情况下，有提示符
    let val = msg.content + "\n>" ;
    let _ = stream.write(val.as_bytes() );
    let _ = stream.write(&[0xff, 0xf9] );
        
}

