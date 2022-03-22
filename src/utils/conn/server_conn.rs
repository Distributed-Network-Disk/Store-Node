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

use std::io::prelude::*;
use std::io::BufReader;
use std::net::Shutdown;
use std::string::String;
use std::sync::{Arc, Condvar, Mutex};

pub struct ServerConn {
    serverip: String,
    control_port: u16,
    client_id: i32,
    self_ip: String,
    self_data_port: i32,
    serverctl: Option<std::net::TcpStream>,
    connecting: bool,
}

static mut sta_server_ip: String = String::new();
static mut sta_control_port: u16 = 0;
impl ServerConn {
    pub fn new(c_id: i32, self_ip: String, self_data_port: i32) -> ServerConn {
        ServerConn {
            serverip: unsafe { sta_server_ip.clone() },
            control_port: unsafe { sta_control_port },

            client_id: c_id,
            connecting: true,
            self_ip: self_ip,
            self_data_port: self_data_port,
            serverctl: None,
        }
    }

    pub fn init(/*&mut self,*/ s_ip: &String, c_port: &u16) {
        unsafe {
            sta_server_ip = (*s_ip).clone();
            sta_control_port = *c_port;
        }
    }

    pub fn run(&mut self, status1: Arc<(Mutex<i32>, Condvar)>) {
        let mut status = true;
        while self.connecting {
            //println!("serverip:{},control_port:{}",self.serverip,self.control_port);
            if let Ok(connect_socket) =
                std::net::TcpStream::connect((&self.serverip[..], self.control_port))
            {
                self.serverctl = Some(connect_socket);
                println!("Connect to server successfully(control)!");
            } else {
                println!("Couldn't connect to server...");
                status = false;
            }

            if !status {
                break;
            }

            let mut input_buf = String::new();
            match &mut self.serverctl {
                None => println!("Error! server not connected..."),
                Some(socket) => {
                    let socket_read = socket.try_clone().expect("clone failed...");
                    let mut in_from_server = BufReader::new(socket_read);
                    socket.write_fmt(format_args!(
                        "3 {} {} {}\n",
                        self.client_id.to_string(),
                        self.self_ip,
                        self.self_data_port
                    ));
                    socket.flush();
                    input_buf.clear();
                    in_from_server.read_line(&mut input_buf).unwrap();
                    println!("input_buf:{}", input_buf);

                    //     while self.connecting {
                    //         socket.write_fmt(format_args!(
                    //             "1 {} {}\n",
                    //             self.client_id.to_string()
                    //         ));
                    //         socket.flush();
                    //         input_buf.clear();
                    //         in_from_server.read_line(&mut input_buf).unwrap();
                    //         let input_buf = input_buf.trim();
                    //         println!("ServerConn -- input_buf:{}\n", input_buf);
                    //         let mut input_vec: Vec<&str> = input_buf[..].split(' ').collect();

                    //         //sleep
                    //         let five_seconds = std::time::Duration::new(5, 0);
                    //         std::thread::sleep(five_seconds);
                    //     }
                    // }
                }
            }

            match &mut self.serverctl {
                None => println!("Error! server not connected..."),
                Some(socket) => {
                    socket.write(b"exit\n");
                    socket.flush();
                    socket
                        .shutdown(Shutdown::Both)
                        .expect("socket shutdown call failed");
                }
            }
        }
        if self.connecting {
            //syn.setStatus(1);

            let &(ref lock, ref cvar) = &*status1;
            let mut status_cur = lock.lock().unwrap();
            *status_cur = 1;
            cvar.notify_all();
            println!("notify main thread");
        }
    }

    pub fn stop_connect(&mut self) {
        self.connecting = false;
    }
}
