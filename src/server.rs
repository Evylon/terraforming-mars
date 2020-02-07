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
    // TODO add pub list of all player sockets
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
                    // TODO add socket to player list
                    // TODO need option for state_machine to interrupt for broadcast to all players
                    match websocket.read_message() {
                        Ok(val) => msg = val,
                        Err(err) => {
                            println!("[LOG] Closing websocket, encountered error {:?}", err);
                            return;
                        }
                    }

                    // Only accept text messages
                    match msg {
                        Message::Close(frame) => {
                            println!("[LOG] Closing websocket, received closing msg with frame {:?}", frame);
                            return;
                        }
                        Message::Text(json_msg) => {
                            println!("[DEBUG] Received message {}", json_msg);
                            let unpacked_cmd: CmdWrapper;
                            match serde_json::from_str(&json_msg) {
                                Ok(cmd) => unpacked_cmd = cmd,
                                Err(err) => {
                                    println!("[LOG] Failed to parse message, encountered error {}", err);
                                    websocket.write_message(Message::Text(
                                        format!("{{ \"msg\": 'Invalid message! Could not parse command{} Encountered error: {}'}}", json_msg, err)
                                    )).unwrap();
                                    continue;
                                }
                            }

                            println!("[LOG] Received and enqueue cmd {:?}", unpacked_cmd);
                            let (deque_lock, deque_cvar) = &*arc_cmd_deque;
                            let mut cmd_deque = deque_lock.lock().unwrap();
                            cmd_deque.push_back(unpacked_cmd);
                            deque_cvar.notify_one();
                            // TODO forward success/failure of cmd to client
                        }
                        Message::Binary(_) => (), // ignore Binary messages
                        Message::Ping(_) => (), // ignore Ping messages
                        Message::Pong(_) => (), // ignore Pong messages
                    }
                }       
            });
        }
    }
}
