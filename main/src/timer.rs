
use crossbeam::channel::{Receiver, Sender};
use tokio::{self, runtime::Runtime, time::{self, Duration, Instant}};
use std::{thread, time::SystemTime};
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

use crate::channel::{wrap_message, wrap_message_ext, Message, MessageType};

use core::sync::atomic::{AtomicUsize, Ordering};

pub fn _handle_timer(
    s_rt: Sender<String>,  //发送到service 
){

    thread::spawn(move ||{
        let rt = Runtime::new().unwrap();
        rt.block_on(async {

            let start = Instant::now();
            let mut intv = time::interval_at(start,
                Duration::from_secs(10));

            loop{

                intv.tick().await;                
                
                let addr = SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(0,0,0,0), 
                    0));
                let msg = wrap_message_ext(MessageType::Timer,
                    addr, utils::now());
                // s_rt.send(msg).unwrap();
            }
        });
    });

}

