use clap::{arg,Command};

// Define a struct to hold the IP, port, and network values
#[derive(Debug,Clone)]
pub struct Config {
    ip: String,
    port: String,
    network: String,
}

impl Config {
    // Define a new function to create a Config instance from the command-line arguments
   pub fn new() -> Self {
        let matches = Command::new("Bitcoin test connection CLI")
            .version("1.0")
            .author("Steliano Gjoka")
            .about("A CLI for testing bitcoin node connection")
            .arg(arg!(--ip <IP_ADDRESS>).help("The ip address").required(true).long("ip"))
            .arg(arg!(--port <PORT_NUMBER>).help("The port number").required(true).long("port"))
            .arg(arg!(--network <BITCOIN_NETWORK>).help("The Bitcoin network (e.g. regtest)").required(true).long("network"))
             .get_matches();

           
        Config {
            ip: matches.get_one::<String>("ip").unwrap().to_string(),
            port: matches.get_one::<String>("port").unwrap().to_string(),
            network: matches.get_one::<String>("network").unwrap().to_string(),
        }
    }
    
    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    pub fn get_port(&self) -> &str {
        &self.port
    }

    pub fn get_network(&self) -> &str {
        &self.network
    }

    
}


