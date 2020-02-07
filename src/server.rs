use std::net::TcpListener;
use std::sync::{Mutex, Arc, Condvar};
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite::protocol::Message;
use std::collections::VecDeque;

use crate::commands::CmdWrapper;

pub struct Server {
    tcp_listener: TcpListener,
    pub cmd_deque: Arc<(Mutex<VecDeque<CmdWrapper>>, Condvar)>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            tcp_listener: TcpListener::bind("127.0.0.1:9001").unwrap(),
            cmd_deque: Arc::new( (Mutex::new(VecDeque::new()), Condvar::new()) ),
        }
    }

    pub fn start(&self) {
        // A WebSocket echo server
        for stream in self.tcp_listener.incoming() {
            let arc_cmd_deque = Arc::clone(&self.cmd_deque);
            spawn (move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                loop {
                    // receive new message
                    let msg: Message;
                    match websocket.read_message() {
                        Ok(val) => msg = val,
                        Err(err) => {
                            println!("[LOG] Closing websocket, encountered error {:?}", err);
                            return;
                        }
                    }

                    // Only accept text messages
                    if msg.is_text() {
                        println!("[DEBUG] Received message {}", msg);
                        let unpacked_cmd: CmdWrapper;
                        let msg_string = &msg.into_text().unwrap();
                        match serde_json::from_str(msg_string) {
                            Ok(cmd) => unpacked_cmd = cmd,
                            Err(err) => {
                                println!("[LOG] Failed to parse message, encountered error {}", err);
                                websocket.write_message(Message::Text(
                                    format!("{{ \"msg\": 'Invalid message! Could not parse command{} Encountered error: {}'}}", msg_string, err)
                                )).unwrap();
                                continue;
                            }
                        }

                        println!("[LOG] Received and enqueue cmd {:?}", unpacked_cmd);
                        let (deque_lock, deque_cvar) = &*arc_cmd_deque;
                        let mut cmd_deque = deque_lock.lock().unwrap();
                        cmd_deque.push_back(unpacked_cmd);
                        deque_cvar.notify_one();
                    }
                }       
            });
        }
    }
}
