
use bitcoin::network::{address, constants, message, message_network};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::config::Config;


#[derive(Debug)]
pub struct P2P{
    config:Config,    
    network:constants::Network

}
impl P2P {
    pub fn new(conf:Config)->P2P{
        P2P {config:conf.clone(),network:set_network(&conf)}
    }
    pub fn get_ip(&self) -> &str {
        &self.config.get_ip()
    }
    
    pub fn get_port(&self) -> &str {
        &self.config.get_port()
    }
   
    pub fn get_network(&self) -> &constants::Network {
        &self.network
    }  
}
   

 fn set_network(cnf:&Config)-> constants::Network {
    let value =cnf.get_network();
    let network= match  value  {
        "regtest"=>constants::Network::Regtest,
        "bitcoin"=>constants::Network::Bitcoin,
        "testnet"=>constants::Network::Testnet,
        _=>constants::Network::Regtest
        
    };
    network
 }  


pub fn build_version_message(address: SocketAddr) -> message::NetworkMessage {
    // Building version message, see https://en.bitcoin.it/wiki/Protocol_documentation#version
    
    let my_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

    // "bitfield of features to be enabled for this connection"
    let services = constants::ServiceFlags::NONE;

    // "standard UNIX timestamp in seconds"
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time error").as_secs();

    // "The network address of the node receiving this message"
    let addr_recv = address::Address::new(&address, constants::ServiceFlags::NONE);

    // "The network address of the node emitting this message"
    let addr_from = address::Address::new(&my_address, constants::ServiceFlags::NONE);

    // "Node random nonce, randomly generated every time a version packet is sent. This nonce is used to detect connections to self."
    let nonce: u64 = 0;

    // "User Agent (0x00 if string is 0 bytes long)"
    let user_agent = String::from("rust-example");

    // "The last block received by the emitting node"
    let start_height: i32 = 10;

    // Construct the message
    message::NetworkMessage::Version(message_network::VersionMessage::new(
        services,
        timestamp as i64,
        addr_recv,
        addr_from,
        nonce,
        user_agent,
        start_height,
    ))
}