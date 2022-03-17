use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;
use std::time::Duration;

use fileattr::fileattr;

pub struct fileuploader {
    serverip: String,
    serverport: u16,
    fragmentfolder: std::path::PathBuf,
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
            fragmentfolder: unsafe { std::path::PathBuf::from(sta_fragmentfolder.clone()) },
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

        if let Ok(serverconnection) = std::net::TcpStream::connect((&self.serverIP[..], self.server_port)) {
            serverconnection
                .set_read_timeout(Some(Duration::new(5, 0)))
                .expect("set_read_timeout call failed");
            serverconnection
                .set_write_timeout(Some(Duration::new(5, 0)))
                .expect("set_read_timeout call failed");
            self.to_server = Some(serverconnection);
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