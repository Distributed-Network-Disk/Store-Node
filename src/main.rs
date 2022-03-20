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

use clap::{Arg, Command};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matches = Command::new("store-node")
        .version("0.1.0")
        .author("totoroyyw <totoro@yyw.moe>")
        .about("Distributed NetDisk StorNode")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .takes_value(true)
                .help("Config file"),
        )
        .get_matches();

    println!("client start");

    let configfile = matches.value_of("config").unwrap_or("config.ini");
    // TODO: use flag parser and conf
    let file = match File::open(configfile) {
        Ok(t) => t,
        Err(e) => {
            println!("{}: {:?}", "Err open config", e);
            return;
        }
    };
    let mut config = BufReader::new(file);
    let mut line = String::new();
    config.read_line(&mut line).unwrap();
    let serverip = String::from(line.trim());
    // TODO
    line.clear();
    config.read_line(&mut line).unwrap();
    let server_control_port = line.trim().parse::<i32>().unwrap();

    line.clear();
    config.read_line(&mut line).unwrap();
    let self_data_port = line.trim().parse::<i32>().unwrap();

    line.clear();
    config.read_line(&mut line).unwrap();
    let client_id = line.trim().parse::<i32>().unwrap();

    line.clear();
    config.read_line(&mut line).unwrap();
    let fragment_folder = String::from(line.trim());
}
