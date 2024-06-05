    
    use std::collections::HashMap;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::SystemTime;
    use crate::player::Player;
    use crossbeam::channel::Sender;
    use crossbeam::channel::unbounded;
    use serde::Deserialize;
    use serde::Serialize;
    use utils::{show_color, Color};

    pub type SessionType = (TcpStream, SocketAddr);

    pub type SessionsType = Arc<Mutex<HashMap<SocketAddr, SessionContext>>>;

    pub struct SessionContext {
        pub cur_session: SessionType,
        pub player: Player
    }

    pub struct Sessions {}

    impl Sessions {        
        pub fn new() -> SessionsType {
            Arc::new(Mutex::new(HashMap::new()))
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    pub enum MessageType {
        Timer,      //定时消息 
        CombatStart,    //战斗开始
        CombatIn,       //战斗中
        CombatStop,     //战斗结束
        Command,    //命令
        Sender,     //发送
        Normal,     //一般消息
    }

    //线程间消息
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Message {
        pub msg_type:  MessageType,    //消息类型
        pub content: String,  //消息内容
        pub addr: SocketAddr,    //消息地址，用于获取用户信息
        pub timer_id: String,  //启动关闭定时器使用
    }
    
    const WELCOME: &str = "
                    
                        

                    ☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆
                        ☆  宇宙时空之旅 ☆
                    ☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆
            

                [1;33m----====   未知世界  ====----[0;00m

            [1;36m∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷
            
            开阔的道路仍在呼唤，
            就像童年时几乎被遗忘的歌曲一样。
            我们所有的失败，尽管有局限性和易错性，
            但我们人类仍有能力创造伟大事业。

                            —— 奈尔·泰森
                    



            ∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷∷[0;00m

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

        let span = utils::show_color("未知世界", Color::BLUE)+ "已经执行了"
            + &show_color(&((duration.unwrap().as_secs()/60 + 1).to_string() + "分"), Color::YELLOW) 
            + "。\n"
            + "目前共有 " 
            + &show_color(&(cnt+1).to_string(), Color::YELLOW)
            +" 位玩家在线上。\n";

        Self::send(session, &span);
        Self::send(session,  "您的英文名字（要注册新人物请输入new。）：");
    }

    fn on_disconnect(session: &mut SessionType) {
        println!("Client disconnected! {}", session.1);
    }

    fn on_message(
        session: &mut SessionType, 
        message: &str, 
        addr: SocketAddr, 
        sessions: &SessionsType
    ) {

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

    pub trait ServerHandler {
        
        fn start(&self, addr: &str, port: &str) -> TcpListener {
            let listener: TcpListener = TcpListener::bind(format!("{}:{}", addr, port)).unwrap();
            println!("Server initialized!");
            return listener;
        }

        fn listen(&self,             
            listener: TcpListener, 
            sessions: SessionsType
        ) {
            let run_start_time = SystemTime::now();
            let mut threads = vec![];
            
            println!(
                "Server started. Listening at {}",
                listener.local_addr().unwrap()
            );

            let (s_rt, r_service) = unbounded::<String>();
            let (s_service, r_sender) = unbounded::<String>();            
            let s_rt_clone = s_rt.clone();
            let s_rt_clone2 = s_rt_clone.clone();
            let (s_combat, r_combat) = unbounded::<String>();

            let sender_sessions = Arc::clone(&sessions);

            //service线程            
            threads.push(thread::spawn(move || {
                crate::service::_handle_service(
                    s_service,
                    r_service,
                    s_combat);
            }));

            //timer线程
            threads.push(thread::spawn(move || {
                crate::timer::_handle_timer(
                    s_rt
                );
            }));

            //combat线程
            threads.push(thread::spawn(move || {
                crate::combat::_handle_timer(
                    s_rt_clone2,
                    r_combat
                );
            }));

            //sender线程
            threads.push(thread::spawn(move || {
                crate::sender::_handle_sender(
                    sender_sessions,
                    r_sender
                );
            }));

            loop {
                match listener.accept() {
                    Ok((stream, addr)) => {
                        let sessions = Arc::clone(&sessions);
                        let s_rt_clone = s_rt_clone.clone();

                        //接收线程
                        threads.push(thread::spawn(move || {
                            Self::_handle_client(stream, sessions, addr,
                                run_start_time, s_rt_clone);
                        }));
                    }
                    Err(e) => println!("couldn't get client: {:?}", e),
                }
            }
        }

        fn _handle_client(
            stream: TcpStream,
            sessions: SessionsType,
            addr: SocketAddr,
            run_start_time: SystemTime,
            s_rt: Sender<String>
        ) {
            let mut session: SessionType = (stream.try_clone().unwrap(), addr);
            let mut reader = BufReader::new(stream.try_clone().unwrap());

            Self::on_connect(&mut session, &sessions, run_start_time);

            Self::add_connect(addr, stream, &sessions);
            
            loop {
                let mut message = String::new();

                match reader.read_line(&mut message) {
                    Ok(_success) => {
                        if message.is_empty() {
                            Self::on_disconnect(&mut session);
                            Self::del_connect(addr, &sessions);
                            return;
                        }
                    },
                    Err(_e) => {                        
                        Self::on_disconnect(&mut session);
                        Self::del_connect(addr, &sessions);
                        return;
                    }
                }

                //分发消息到service模块                
                let msg = wrap_message_ext(MessageType::Command,
                    addr, message);
                s_rt.send(msg).unwrap();
            }
        }

        fn send(session: &mut SessionType, message: &str) {
            let _ = session.0.write(message.as_bytes());
        }

        #[allow(dead_code)]
        fn send_all(sessions: &mut SessionsType, message: &str) {
            for session in sessions.lock().unwrap().iter_mut() {
                println!("Send message to {:?}: {}", session.0, message);
                Self::send(&mut session.1.cur_session, message);
            }
        }

        
        fn add_connect(
            addr: SocketAddr, 
            stream: TcpStream,
            sessions: &SessionsType
        ){
            let mut sessions_ok = sessions.lock().unwrap();
            let session_cur: SessionType = (stream.try_clone().unwrap(), addr);
            sessions_ok.entry(addr)                   
                .or_insert(SessionContext{
                    cur_session: session_cur,
                    player: Player::new(),
                });
            println!(" Connect count = {}", sessions_ok.len());
        }

        fn del_connect(
            addr: SocketAddr, 
            sessions: &SessionsType
        ){
            let mut sessions_err = sessions.lock().unwrap();
            sessions_err.remove(&addr);
            println!(" Connect count = {}", sessions_err.len());
        }

        fn get_connect(sessions: &SessionsType) -> u32 {
            let sessions_cnt = sessions.lock().unwrap();            
            println!(" Connect count = {}", sessions_cnt.len());
            sessions_cnt.len().try_into().unwrap()
        }

        fn on_connect(session: &mut SessionType, sessions: &SessionsType, run_start_time: SystemTime);

        fn on_disconnect(session: &mut SessionType);

        fn on_message(session: &mut SessionType, message: &str, addr: SocketAddr, sessions: &SessionsType);
    }

    pub fn wrap_message_timer(msg_type: MessageType, addr: SocketAddr, message: String, timer_id: String) -> String {
        let msg = serde_json::to_string(&Message {
            msg_type,
            content: message.trim().to_string(),
            addr,
            timer_id
        }).unwrap();
        msg
    }

    pub fn wrap_message_ext(msg_type: MessageType, addr: SocketAddr, message: String) -> String {
        let msg = serde_json::to_string(&Message {
            msg_type,
            content: message.trim().to_string(),
            addr,
            timer_id: "".to_string()
        }).unwrap();
        msg
    }

    pub fn wrap_message(addr: SocketAddr, message: String) -> String {
        let msg = serde_json::to_string(&Message {
            msg_type: MessageType::Normal,
            content: message.trim().to_string(),
            addr: addr,
            timer_id: "".to_string()
        }).unwrap();
        msg
    }
    