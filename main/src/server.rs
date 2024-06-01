use crate::{channel::{ServerHandler, SessionType, Sessions, SessionsType}, player};
use utils::{show_color, Color};
use std::{net::SocketAddr, sync::Arc, time::SystemTime};


const WELCOME: &str = "
              
              

  ☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆
☆ 飞雪连天射白鹿，笑书神侠倚碧鸳 ☆
  ☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆
   本游戏参考北大侠客行编写     

   [1;36m----====   小 草  ====----

∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷
    野火烧不尽，春风吹又生


            火



∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷[0;00m
";

pub struct Server {
    pub sessions: SessionsType
}

impl Server {
    pub fn new() -> Self {
        Self {
            sessions: Sessions::new()
        }
    }

}

impl ServerHandler for Server {


    fn on_connect(
        session: &mut SessionType, 
        sessions: &SessionsType,
        run_start_time: SystemTime
    ) {

        let duration = run_start_time.elapsed();

        Self::send(session, WELCOME);

        let cnt = Self::get_connect(sessions);

        let span = "小草已经执行了".to_owned() 
            + &show_color(&((duration.unwrap().as_secs()/60).to_string() + "分"), Color::YELLOW) 
            + "。\n"
            + "目前共有 " 
            + &show_color(&(cnt+1).to_string(), Color::YELLOW)
            +" 位玩家在线上。\n";
        
        // println!("{}", &span);
        Self::send(session, &span);

        Self::send(session,  "您的英文名字（要注册新人物请输入new。）：");
    }

    fn on_disconnect(session: &mut SessionType) {
        println!("Client disconnected! {}", session.1);
    }

    fn on_message(session: &mut SessionType, message: &str, addr: SocketAddr, sessions: &SessionsType) {
        // if !message.is_empty() {                
        //     return;
        // }
        println!("on_message: {} {:?}", message, addr);

        if message.trim() == "lxz" {
            Self::send(session,  "此ID档案已存在,请输入密码:");
            return;
        }

        if message.trim() == "abc123" {
            Self::send(session,  "重新连线完毕。");
            return;
        }

        //新用户登录
        let mut sessions_ok = sessions.lock().unwrap();
        
        if message.trim() == "upgrade" {
            sessions_ok.entry(addr)
                .and_modify(|ctx| {
                    ctx.player.name = "龙年".to_string();
                    ctx.player.level = 30;
                });
        } 

        let player = &sessions_ok.get(&addr).unwrap().player;
        
        let hpframe = r"
        ┌─── ".to_owned() + &utils::show_color(&player.name, Color::YELLOW) + "状态────────────┬───────────────────┐
        │【精神】 "+ &utils::show_color(&player.level.to_string(), Color::RED) +"     / 125      [100%]    │【精力】 100     / 100     (+   0)    │
        │【气血】 17      / 127      [100%]    │【内力】 141     / 71      (+   0)    │
        │【真气】 0       / 0        [  0%]    │【战意】 100%               [正常]    │
        │【食物】 0       / 300      [饥饿]    │【潜能】 5075                         │
        │【饮水】 0       / 300      [饥渴]    │【经验】 830                          │
        ├───────────────────┴───────────────────┤
        │【状态】 健康                                                                 │
        └──────────────────────────────北大侠客行────┘\n>";

        if message.trim() == "hp" {
            Self::send(session,  &hpframe);
            return;
        }

        if message.trim() == "l" {
            
            let mut other = 
                sessions_ok
                .iter()
                .filter(|p| !(p.0.to_string() == addr.to_string())) ;

            println!("{:?}", other.next().unwrap().1.player.name);
        }

        // echos back the message
        // let x = show_color(message, Color::PINK);
        // Self::send(session,  &x);

        
        //全局通知
        // let mut all = Arc::clone(&sessions);
        // let msg = address.to_string() + ":" + &message;
        // Self::send_all(&mut all, &msg);
    }
}

