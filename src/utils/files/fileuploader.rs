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

use std::string::String;
use std::time::Duration;

use super::fileattr;

// #[derive(Clone)]
pub struct fileuploader {
    serverip: String,
    serverport: u16,
    fragment_folder: std::path::PathBuf,
    serverctl: Option<std::net::TcpStream>,
    connecting: bool,
}

static mut sta_serverip: String = String::new();
static mut sta_serverport: u16 = 0;
static mut sta_fragmentfolder: String = String::new();

impl fileuploader {
    pub fn init(f: &String, ip: &String, port: &u16) -> Self {
        unsafe {
            sta_serverip = (*ip).clone().to_string();
            sta_serverport = *port;
            sta_fragmentfolder = (*f).clone().to_string();
        }
        fileuploader {
            serverip: unsafe { sta_serverip.clone() },
            serverport: unsafe { sta_serverport },
            fragment_folder: unsafe { std::path::PathBuf::from(sta_fragmentfolder.clone()) },
            serverctl: None,
            connecting: false,
        }
    }

    pub fn isserverconnect(&mut self) -> bool {
        if self.serverip.is_empty() || self.serverport == 0 {
            println!("Err serverip or serverport");
            self.connecting = false;
            return false;
        }

        if let Ok(serverconnection) =
            std::net::TcpStream::connect((&self.serverip[..], self.serverport))
        {
            serverconnection
                .set_read_timeout(Some(Duration::new(5, 0)))
                .expect("set_read_timeout call failed");
            serverconnection
                .set_write_timeout(Some(Duration::new(5, 0)))
                .expect("set_read_timeout call failed");
            self.serverctl = Some(serverconnection);
            println!("Control link: Connect to server successfully");
            self.connecting = true;
            return true;
        } else {
            println!("Control link: Cannot connect to server");
            self.connecting = false;
            return false;
        }
    }
}
