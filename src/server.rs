use std::net::{TcpListener, TcpStream};
use std::sync::{Mutex, Arc, Condvar};
use std::thread::spawn;
use std::collections::VecDeque;
use uuid::Uuid;
use tungstenite::server::accept;
use tungstenite::protocol::{Message, WebSocket};

use crate::commands::CmdWrapper;
use crate::game_state::GameState;

pub struct Server {
    tcp_listener: TcpListener,
    pub cmd_deque: Arc<(Mutex<VecDeque<CmdWrapper>>, Condvar)>,
    pub broadcast_deque: Arc<(Mutex<VecDeque<GameState>>, Condvar)>,
    pub error_deque: Arc<(Mutex<VecDeque<(CmdWrapper, String)>>, Condvar)>,
    pub connections: Arc<Mutex<Vec<(usize, Uuid)>>>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            tcp_listener: TcpListener::bind("127.0.0.1:9001").unwrap(),
            cmd_deque: Arc::new( (Mutex::new(VecDeque::new()), Condvar::new()) ),
            broadcast_deque: Arc::new( (Mutex::new(VecDeque::new()), Condvar::new()) ),
            error_deque: Arc::new( (Mutex::new(VecDeque::new()), Condvar::new()) ),
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(&self) {
        // A WebSocket echo server
        for stream in self.tcp_listener.incoming() {
            let arc_cmd_deque = Arc::clone(&self.cmd_deque);
            let arc_connections = Arc::clone(&self.connections);
            spawn (move || {
                let websocket = accept(stream.unwrap()).unwrap();
                let player_uuid = Uuid::new_v4();
                let player_id;
                {
                    let mut connections = arc_connections.lock().unwrap();
                    player_id = connections.len();
                    connections.push((player_id, player_uuid));
                }
                let mut connection = Connection::new(websocket, player_id, player_uuid, arc_cmd_deque);
                println!("[LOG] New connection player_id: {}, uuid: {}", player_id, player_uuid);
                let mut connection_open = true;
                while connection_open {
                    match connection.handle_read() {
                        Ok(_) => (),
                        Err(_) => connection_open = false,
                    }
                }
            });
        }
    }
}

pub struct Connection {
    pub socket: WebSocket<TcpStream>,
    pub player_id: usize,
    pub player_uuid: Uuid,
    pub arc_cmd_deque: Arc<(Mutex<VecDeque<CmdWrapper>>, Condvar)>,
}

impl Connection {
    pub fn new(
            socket: WebSocket<TcpStream>,
            id: usize,
            uuid: Uuid,
            arc_cmd_deque: Arc<(Mutex<VecDeque<CmdWrapper>>, Condvar)>
    ) -> Connection {
        Connection {
            socket: socket,
            player_id: id,
            player_uuid: uuid,
            arc_cmd_deque: arc_cmd_deque,
        }
    }

    fn handle_read(&mut self) -> Result<(), tungstenite::Error> {
        // receive new message
        let msg: Message;
        // TODO add socket to player list
        // TODO need option for state_machine to interrupt for broadcast to all players
        match self.socket.read_message() {
            Ok(val) => msg = val,
            Err(err) => {
                println!("[ERR] Read after close {:?}", err);
                return Err(err);
            }
        }

        // Only accept text messages
        match msg {
            Message::Close(frame) => {
                println!("[LOG] Closing websocket, received closing msg with frame {:?}", frame);
                return Err(tungstenite::Error::ConnectionClosed);
            }
            Message::Text(json_msg) => {
                println!("[DEBUG] Received message {}", json_msg);
                let unpacked_cmd: CmdWrapper;
                match serde_json::from_str(&json_msg) {
                    Ok(cmd) => unpacked_cmd = cmd,
                    Err(err) => {
                        println!("[ERR] Failed to parse message, encountered error {}", err);
                        self.socket.write_message(Message::Text(
                            format!("{{ \"msg\": 'Invalid message! Could not parse command {} Encountered error: {}'}}", json_msg, err)
                        )).unwrap();
                        return Ok(());
                    }
                }

                println!("[LOG] Received and enqueue cmd {:?}", unpacked_cmd);
                let (deque_lock, deque_cvar) = &*self.arc_cmd_deque;
                let mut cmd_deque = deque_lock.lock().unwrap();
                cmd_deque.push_back(unpacked_cmd);
                deque_cvar.notify_one();
                // TODO forward success/failure of cmd to client
            }
            Message::Binary(_) => (), // ignore Binary messages
            Message::Ping(_) => (), // ignore Ping messages
            Message::Pong(_) => (), // ignore Pong messages
        }
        return Ok(())
    }
}
