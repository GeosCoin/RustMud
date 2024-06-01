    
    use std::collections::HashMap;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use std::time::SystemTime;
    use crate::player::Player;
    use crate::service;
    use crossbeam::channel::Receiver;
    use crossbeam::channel::Sender;
    use crossbeam::channel::{unbounded, bounded};

    pub type SessionType = (TcpStream, SocketAddr);

    pub type SessionsType = Arc<Mutex<HashMap<SocketAddr, SessionContext>>>;

    pub struct SessionContext {
        pub cur_session: SessionType,
        pub player: Player
    }

    pub struct Sessions {}

    impl Sessions {
        #[allow(dead_code)]
        pub fn new() -> SessionsType {
            Arc::new(Mutex::new(HashMap::new()))
        }
    }



    pub trait ServerHandler {
        
        fn start(&self, addr: &str, port: &str) -> TcpListener {
            let listener: TcpListener = TcpListener::bind(format!("{}:{}", addr, port)).unwrap();
            println!("Server initialized!");
            return listener;
        }

        fn listen(&self, listener: TcpListener, 
            sessions: SessionsType, run_start_time: SystemTime) {
            let mut threads = vec![];
            
            println!(
                "Server started. Listening at {}",
                listener.local_addr().unwrap()
            );

            let (s_rt, r_service) = unbounded::<String>();
            let (s_service, r_sender) = unbounded::<String>();            
            let s_rt_clone = s_rt.clone();

            let service_sessions = Arc::clone(&sessions);
            let sender_sessions = Arc::clone(&sessions);

            //service线程            
            threads.push(thread::spawn(move || {
                crate::service::_handle_service(
                    service_sessions,
                    s_service,
                    r_service);
            }));

            //timer线程
            threads.push(thread::spawn(move || {
                crate::timer::_handle_timer(
                    s_rt
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

                println!("{}", message);

                // let msg:&str = &(message.clone());
                s_rt.send(message).unwrap();

                // Self::on_message(&mut session, &message, addr, &sessions);
                
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
