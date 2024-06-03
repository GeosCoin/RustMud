
use crossbeam::channel::{Receiver, Sender};
use tokio::{self, runtime::Runtime, time::{self, Duration, Instant}};
use std::{thread, time::SystemTime};
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

use crate::channel::{wrap_message, Message};

use core::sync::atomic::{AtomicUsize, Ordering};

static MY_VALUE: AtomicUsize = AtomicUsize::new(0);

fn on_timer (message: &String, s_rt: Sender<String>){

    println!("on_timer : {:?}", message);
    let msg: Message = serde_json::from_str(&message).unwrap();
    
    let val: usize = msg.content.parse::<usize>().unwrap();
    MY_VALUE.store(val, Ordering::Relaxed);

    let t = thread::spawn(move ||{
        let rt = Runtime::new().unwrap();
        rt.block_on(async {

            let start = Instant::now();
            let mut intv = time::interval_at(start,
                Duration::from_secs(2));

            loop{
                let a = MY_VALUE.load(Ordering::Relaxed);
                if a == 10 {
                    return;
                }

                intv.tick().await;
                
                let addr = SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(0,0,0,0), 
                    0));
                let msg = wrap_message(addr, utils::now());
                s_rt.send(msg).unwrap();
            }
        });
    });

}

pub fn _handle_timer(
    s_rt: Sender<String>,  //发送到service    
    r_timer: Receiver<String>, //从service接收控制信息
){
    
    loop {
        match r_timer.recv() {
            Ok(a) => {
                let s_rt_clone = s_rt.clone();
                on_timer(&a,  s_rt_clone);
            },
            Err(s) => {
                println!("{:?}", s);
                thread::sleep(Duration::from_secs(5000));
            }
        }
    }
    
    
    
}

