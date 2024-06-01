
use crate::{channel::{ServerHandler, SessionType, Sessions, SessionContext, SessionsType}, player};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;

pub fn on_sender(sessions: &SessionsType, message: String){
    for session in sessions.lock().unwrap().iter_mut() {
        println!("Send message to {:?}: {}", session.0, message);
        
        let _ = session.1.cur_session.0.write(message.as_bytes());
    }
}

