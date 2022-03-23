use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::mpsc::channel;

use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

const CONNECTION: &'static str = "ws://127.0.0.1:8000";
static mut sta_port: i32 = -1;

pub struct WS {
    wsclient: websocket::sync::Client<std::net::TcpStream>,
}

impl WS {
    pub fn new(
        server: &mut websocket::server::WsServer<
            websocket::server::NoTlsAcceptor,
            std::net::TcpListener,
        >,
    ) -> Option<WS> {
        let request = &mut server.filter_map(Result::ok);
        let request = request.next().unwrap();
        let client = request.use_protocol("websocket").accept().unwrap();
        let result: Option<WS> = Some(WS { wsclient: client });
        return result;
    }

    pub fn send_file(&mut self, f_path: &PathBuf) {
        let mut f: File = File::open(&f_path.as_path()).unwrap();
        let mut contents = Vec::new();
        f.read_to_end(&mut contents);
        let message = OwnedMessage::Binary(contents);
        let _ = &mut self.wsclient.send_message(&message).unwrap();
    }

    pub fn send_message(&mut self, msg: String) {
        let message = OwnedMessage::Text(msg);
        let _ = &mut self.wsclient.send_message(&message).unwrap();
    }

    pub fn echo(&mut self) -> OwnedMessage {
        let message: OwnedMessage = self.recv();
        println!("Receive Loop: {:?}", message);
        let _ = &mut self.wsclient.send_message(&message).unwrap();
        return message;
    }

    pub fn recv_file(&mut self, f_path: &PathBuf) {
        //let mut f:File = File::open(&f_path.as_path()).unwrap();
        let message: OwnedMessage = self.recv();
        match message {
            OwnedMessage::Binary(contents) => {
                fs::write(f_path.as_path(), contents);
            }
            _ => println!("no binary for file\n"),
        }
    }

    pub fn recv(&mut self) -> OwnedMessage {
        let message_record: OwnedMessage = OwnedMessage::Close(None);
        //let receive_loop = thread::spawn(move || {
        // Receive loop
        //let (mut receiver, mut sender) = self.client.split().unwrap();
        let (tx, rx) = channel();
        loop {
            //for message in receiver.incoming_messages() {
            let message = self.wsclient.recv_message();
            let message = match message {
                Ok(m) => m,
                Err(e) => {
                    match e {
                        no_data_available => break, // 没有receive到消息时，break跳出while true
                        _ => {
                            println!("Receive Loop: {:?}", e);
                            let _ = tx.send(OwnedMessage::Close(None));
                            return message_record;
                        }
                    }
                }
            };
            let message_record = message.clone();
            match message {
                OwnedMessage::Close(_) => {
                    // Got a close message, so send a close message and return
                    let _ = tx.send(OwnedMessage::Close(None));
                }
                OwnedMessage::Ping(data) => {
                    match tx.send(OwnedMessage::Pong(data)) {
                        Ok(()) => (),
                        Err(e) => {
                            println!("Receive Loop: {:?}", e);
                        }
                    }
                }
                _ => {
                    println!("Receive Loop: {:?}", message);
                    return message;
                }
            }
        }
        return message_record;
    }
    pub fn close(&self) -> Result<(), std::io::Error> {
        self.wsclient.shutdown()
    }
}
