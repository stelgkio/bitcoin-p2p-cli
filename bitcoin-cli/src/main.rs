use bitcoin::consensus::{encode, Decodable};
use bitcoin::network::message;
use peer::P2P;
use std::net::SocketAddr;
use std::process;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
mod config;
mod peer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an instance of the Config struct based on the command-line arguments
    let config = config::Config::new();
    println!("{:?}",config);
    
    let p2p= P2P::new(config);

/////
//  I did also the same solution with thread spawn and channel to read the data
//  std::tcpstream has a set_non_blocking so the behauvior is the like tokio but the code is more simple. 
////
    // let ( sender,  receiver) = mpsc::channel::<String>();
    // //Spawn a new thread to handle the tcp reqeust
    // let result =thread::spawn(move || {
    //     //on_connection(p2p,sender).unwrap();
    // });

    // //Wait until the thread is completed 
    // result.join().unwrap();
    on_connection(p2p).await?;
    //Print all messages
    // for res in receiver{
    //     println!("{:?}",res);
    // }
    Ok(())
}



async fn on_connection(p2p:P2P) -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr =  format!("{}:{}",p2p.get_ip(),p2p.get_port()).parse().unwrap_or_else(|error| {
        ("Error parsing address: {:?}", error);
        process::exit(1);
    });

    //Build version message for bitcoin
    let version_message = peer::build_version_message(address);

    let first_message = message::RawNetworkMessage {
        magic: p2p.get_network().magic(),
        payload: version_message,
    };

    let mut stream = TcpStream::connect(address).await.expect("Failed to open connection");
    stream.set_nodelay(true)?;
    
        // Send the message
        let _ = stream.write_all(encode::serialize(&first_message).as_slice()).await?;
        println!("Sent version message");

        // Setup buffer
        let mut buffer = [0; 1024];
        loop {
            let n = match stream.read(&mut buffer).await {
             //   Ok(n) if n == 0 => break, // Verack is 0 bytes long  
                Ok(n) => n,
                Err(e) => return Err(Box::new(e)),
            };

            let mut reader = &buffer[..n];
            let reply  =  message::RawNetworkMessage::consensus_decode(&mut reader).unwrap();
               match reply.payload {
                    message::NetworkMessage::Version(_) => {
                        println!("Received version message: {:?}", reply.payload);
    
                        let second_message = message::RawNetworkMessage {
                            magic: p2p.get_network().magic(),
                            payload: message::NetworkMessage::Verack,
                        };
                       // println!("{:?}",second_message);
                        let _ = stream.write_all(encode::serialize(&second_message).as_slice()).await?;
                        println!("Sent verack message");
                    }
                    message::NetworkMessage::Verack => {
                        println!("Received verack message: {:?}", reply.payload);
                        break;
                    }
                    _ => {
                        println!("Received unknown message: {:?}", reply.payload);
                        break;
                    }
                }
            
        }
          
        Ok(())
    }    
    

