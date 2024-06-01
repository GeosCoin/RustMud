
use crossbeam::channel::Sender;
use tokio::{self, runtime::Runtime, time::{self, Duration, Instant}};
use std::thread;

pub fn _handle_timer(
    s_rt: Sender<String>,  //发送到socket    
){

    thread::spawn(move ||{
        let rt = Runtime::new().unwrap();
        rt.block_on(async {

            let start = Instant::now();
            let mut intv = time::interval_at(start,
                Duration::from_secs(10));

            loop{
                intv.tick().await;
                let now = utils::now();
                s_rt.send(now).unwrap();
            }
        });
    });

    
}

