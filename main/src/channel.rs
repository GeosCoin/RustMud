    
    
    use std::collections::HashMap;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::net::Ipv4Addr;
    use std::net::SocketAddrV4;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::os::windows::io::IntoRawSocket;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use std::time::SystemTime;
    use crate::player::Player;
    use crate::service::Service;
    use crate::timer;
    use crossbeam::channel::Sender;
    use crossbeam::channel::unbounded;
    use serde::Deserialize;
    use serde::Serialize;
    use tokio::stream;
    use utils::{show_color, Color};

    pub type SessionType = (TcpStream, SocketAddr);

    pub type SessionsType = Arc<Mutex<HashMap<SocketAddr, SessionContext>>>;

    pub struct SessionContext {
        pub cur_session: SessionType,
        pub player_name: String
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
        NoPrompt,   //不带提示符
        IacDoTerm,    //GMCP Server:  IAC DO TERMINAL-TYPE  ff fd 18
        IacWillTerm,  //GMCP Client:  IAC WILL TERMINAL-TYPE   ff fb 18
        IacWillGmcp,  //GMPC Server:  ff fb c9 
        IacDoGmcp,    //GMCP Mutual:  ff fd c9 ff fa c9 ... ff f0
        
    }

    //线程间消息
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Message {
        pub msg_type:  MessageType,    //消息类型
        pub content: String,  //消息内容
        pub addr: SocketAddr,    //消息地址，用于获取用户信息
        pub timer_id: String,  //启动关闭定时器使用
        pub max_cnt: u32,      //线程循环最大次数
    }

    impl Message {
        pub fn new() -> Self {
            Message {
                msg_type: MessageType::Normal,
                content: String::from(""),
                addr: SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(0,0,0,0), 
                    0)),
                timer_id : String::from("0"),
                max_cnt: 0,
            }
        }
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
                    

            随着马斯克的星舰的到来，一个星际时代即将来临。
            让我们一起来开创火星之旅，开启未知世界之旅。

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
            let srv_sessions = Arc::clone(&sessions);

            //service线程            
            threads.push(thread::spawn(move || {
                let mut service = Service::new(&srv_sessions,
                    &s_service,
                    &r_service,
                    &s_combat);
                
                service.handle();
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

            let mut stream_clone = stream.try_clone().unwrap();

            Self::on_connect(&mut session, &sessions, run_start_time);

            Self::add_connect(addr, &stream, &sessions);
            
            loop {
                let mut message = String::new();
                let mut buf:[u8; 1024] = [0; 1024]; // !!!Important!!!
                
                match reader.read(&mut buf){
                    Ok(_success) => {                        
                        let mut bufx0: Vec<u8> = buf.into_iter().filter(|p| *p != b'\0').collect();
                        println!("raw bytes: {:?}, len = {}", bufx0.as_slice(), bufx0.len());
                        if bufx0.len() == 0 {
                            Self::on_disconnect(&mut session);
                            Self::del_connect(addr, &sessions);
                            return;
                        }

                        let mut buf_clone = bufx0.clone();                        
                        message = match String::from_utf8(bufx0) {
                            Ok(a) => a,
                            Err(e) => "".to_string(),
                        };
                        
                        if message.is_empty() {
                            message = do_raw_data(&mut buf_clone, &mut stream_clone);
                        }

                        if message.is_empty() {
                            continue;
                        }
                    },
                    Err(_e) => {   
                        println!("error: {:?}", _e);           
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
            stream: &TcpStream,
            sessions: &SessionsType
        ){
            let mut sessions_ok = sessions.lock().unwrap();
            let session_cur: SessionType = (stream.try_clone().unwrap(), addr);
            sessions_ok.entry(addr)                   
                .or_insert(SessionContext{
                    cur_session: session_cur,
                    player_name: String::new(),
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

    }

    fn do_raw_data(buf_clone: &[u8], stream_clone: &mut TcpStream) -> String {
        if buf_clone.len() < 3 {  
            stream_clone.flush();                     
            return "奇怪的来宾".to_string();
        }

        //以下是GMCP的处理
        if (buf_clone[0] == 0xff && buf_clone[1] == 0xfb && buf_clone[2] == 0x18)
        || (buf_clone[0] == 0xff && buf_clone[1] == 0xfb && buf_clone[2] == 0xc9)
        || (buf_clone[0] == 0xff && buf_clone[1] == 0xfc && buf_clone[2] == 0xc9) {
            let buf = &buf_clone[3..];
            println!("there is GMCP .");
            stream_clone.flush();
            return String::from_utf8_lossy(buf).to_string();
        } else if buf_clone[0] == 0xff && buf_clone[1] == 0xfa && buf_clone[2] == 0xc9
            && buf_clone[buf_clone.len()-2] == 0xff && buf_clone[buf_clone.len()-1] == 0xf0
        {
            let buf = &buf_clone[3..buf_clone.len()-2];
            println!("GMCP cmd: {:?}", buf);
            stream_clone.flush();
            return String::from_utf8_lossy(buf).to_string();
        } else{
            return "奇怪的来宾".to_string();
        }
    }
    
    pub fn wrap_message_climb(msg_type: MessageType, addr: SocketAddr, message: String, timer_id: String, max_cnt: u32) -> String {
        let msg = serde_json::to_string(&Message {
            msg_type,
            content: message.trim().to_string(),
            addr,
            timer_id,
            max_cnt
        }).unwrap();
        msg
    }

    pub fn wrap_message_timer(msg_type: MessageType, addr: SocketAddr, message: String, timer_id: String) -> String {
        let msg = serde_json::to_string(&Message {
            msg_type,
            content: message.trim().to_string(),
            addr,
            timer_id,
            max_cnt: 60
        }).unwrap();
        msg
    }

    pub fn wrap_message_ext(msg_type: MessageType, addr: SocketAddr, message: String) -> String {
        let msg = serde_json::to_string(&Message {
            msg_type,
            content: message.trim().to_string(),
            addr,
            timer_id: "".to_string(),
            max_cnt: 60, //默认两分钟
        }).unwrap();
        msg
    }

    pub fn wrap_message(addr: SocketAddr, message: String) -> String {
        let msg = serde_json::to_string(&Message {
            msg_type: MessageType::Normal,
            content: message.trim().to_string(),
            addr: addr,
            timer_id: "".to_string(),
            max_cnt: 0
        }).unwrap();
        msg
    }

    pub fn wrap_message_raw(addr: SocketAddr, message: String) -> String {
        let msg = serde_json::to_string(&Message {
            msg_type: MessageType::Normal,
            content: message,
            addr: addr,
            timer_id: "".to_string(),
            max_cnt: 0
        }).unwrap();
        msg
    }
