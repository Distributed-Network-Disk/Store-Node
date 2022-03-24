// Copyright [2022] [totoroyyw]

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// 	http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate websocket;
use super::ws::WS;
use std::sync::{Arc, Condvar, Mutex};

pub struct ClientConn {
    self_data_port: i32,
    self_ip: String,
    server: websocket::server::WsServer<websocket::server::NoTlsAcceptor, std::net::TcpListener>,
}

impl ClientConn {
    pub fn new(port: i32, self_ip: String) -> ClientConn {
        let mut addr = String::new();
        addr.push_str(&self_ip);
        addr.push_str(&":");
        addr.push_str(&port.to_string());
        ClientConn {
            self_data_port: port,
            self_ip: self_ip,
            server: websocket::sync::Server::bind(addr).unwrap(),
        }
    }

    pub fn run(&mut self, status1: Arc<(Mutex<i32>, Condvar)>) {
        println!(
            "WebSocket Server has started on {} :{}.\r\nWaiting for a connection...",
            self.self_ip, self.self_data_port
        );
        loop {
            let user: WS = WS::new(&mut self.server).unwrap();
            println!("A user connected.");
        }
    }
}
