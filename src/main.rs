use clap::{Arg, Command};
use std::fs::File;

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

    let configfile = matches.value_of("config").unwrap_or("config.yml");
    let file = match File::open(configfile) {
        Ok(t) => t,
        Err(e) => {
            println!("{}: {:?}", "Err open config", e);
            return;
        }
    };
}
