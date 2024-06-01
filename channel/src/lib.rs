pub mod prelude {
    use std::collections::HashMap;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::SystemTime;

    pub type SessionType = (TcpStream, SocketAddr);

    pub type SessionsType = Arc<Mutex<HashMap<SocketAddr, SessionType>>>;

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

            loop {
                match listener.accept() {
                    Ok((stream, addr)) => {
                        let sessions = Arc::clone(&sessions);

                        threads.push(thread::spawn(move || {
                            Self::_handle_client(stream, sessions, addr,
                                run_start_time);
                        }));
                    }
                    Err(e) => println!("couldn't get client: {:?}", e),
                }
            }
        }

        fn add_connect(
            addr: SocketAddr, 
            stream: TcpStream,
            sessions: &SessionsType
        ){
            let mut sessions_ok: std::sync::MutexGuard<HashMap<SocketAddr, (TcpStream, SocketAddr)>> = sessions.lock().unwrap();
            let session_cur: SessionType = (stream.try_clone().unwrap(), addr);
            sessions_ok.entry(addr)                   
                .or_insert(session_cur);
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

        fn _handle_client(
            stream: TcpStream,
            sessions: Arc<Mutex<HashMap<SocketAddr, SessionType>>>,
            addr: SocketAddr,
            run_start_time: SystemTime
        ) {
            let mut session: SessionType = (stream.try_clone().unwrap(), addr);
            let mut reader = BufReader::new(stream.try_clone().unwrap());

            Self::on_connect(&mut session, &sessions, run_start_time);

            Self::add_connect(addr, stream, &sessions);

            loop {
                let mut message = String::new();

                match reader.read_line(&mut message) {
                    Ok(_success) => (),
                    Err(_e) => {                        
                        Self::on_disconnect(&mut session);
                        Self::del_connect(addr, &sessions);
                        return;
                    }
                }

                Self::on_message(&mut session, &message, &addr.to_string(), &sessions);
                
            }
        }

        fn send(session: &mut SessionType, message: &str) {
            let _ = session.0.write(message.as_bytes());
        }

        fn send_all(sessions: &mut SessionsType, message: &str) {
            for session in sessions.lock().unwrap().iter_mut() {
                println!("Send message to {:?}: {}", session.0, message);
                Self::send(session.1, message);
            }
        }

        fn on_connect(session: &mut SessionType, sessions: &SessionsType, run_start_time: SystemTime);

        fn on_disconnect(session: &mut SessionType);

        fn on_message(session: &mut SessionType, message: &str, address: &str, sessions: &SessionsType);
    }
}
