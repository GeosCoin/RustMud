
use crossbeam::channel::{Receiver, Sender};
use tokio::{self, runtime::Runtime, time::{self, Duration, Instant}};
use std::{thread, time::SystemTime};
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

use crate::channel::{wrap_message, wrap_message_ext, wrap_message_timer, Message, MessageType};

use core::sync::atomic::{AtomicUsize, Ordering};

static COMBAT_TIMER_ID: AtomicUsize = AtomicUsize::new(0);

fn on_timer (message: &String, s_rt: Sender<String>){

    println!("combat on_timer : {:?}", message);
    let msg: Message = serde_json::from_str(&message).unwrap();

    if msg.msg_type != MessageType::CombatStart
     && msg.msg_type != MessageType::CombatStop
     {
        println!("not combat msg.");
        return;
    }
    
    let val: usize = msg.timer_id.parse::<usize>().unwrap();
    let mut max_cnt = msg.max_cnt; 

    //结束时，比较timerid
    if msg.msg_type == MessageType::CombatStop {
        COMBAT_TIMER_ID.store(val, Ordering::Relaxed);
    }
    
    //开始时，启动新线程
    if msg.msg_type == MessageType::CombatStart {
        //每个战斗任务规定最多2分钟就会自动结束
        thread::spawn(move ||{
            let rt = Runtime::new().unwrap();
            rt.block_on(async {

                let start = Instant::now();
                let mut intv = time::interval_at(start,
                    Duration::from_millis(1000));

                let mut cnt = 0;

                intv.tick().await;  //提前等一下

                loop{
                    cnt += 1;

                    //最多2分钟时间，战斗线程会释放
                    //避免线程永远没有机会释放
                    if cnt > max_cnt {  
                        println!(" > 2 minutes auto kill thread.");
                        return;
                    }

                    let a = COMBAT_TIMER_ID.load(Ordering::Relaxed);
                    if a == val {
                        println!("timer id {} kill thread.", val);
                        return;
                    }

                    intv.tick().await;               
                    
                    //最后一次，发送停止命令
                    if cnt == max_cnt { 
                        let msg = wrap_message_timer(MessageType::CombatStop,
                            msg.addr, msg.content.to_owned() + " stop", val.to_string());
                        s_rt.send(msg).unwrap();
                    } else {
                        let msg = wrap_message_timer(MessageType::CombatIn,
                            msg.addr, msg.content.to_owned() + " continue", val.to_string());
                        s_rt.send(msg).unwrap();
                    }
                }
            });
        });
    }
}

pub fn _handle_timer(
    s_rt: Sender<String>,  //发送到service    
    r_combat: Receiver<String>, //从service接收控制信息
){
    
    loop {
        match r_combat.recv() {
            Ok(a) => {
                let s_rt_clone = s_rt.clone();
                on_timer(&a,  s_rt_clone);
            },
            Err(s) => {
                println!("{:?}", s);
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
    
    
    
}

