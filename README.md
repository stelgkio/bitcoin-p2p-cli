# bitcoin-p2p-cli
Rust cli app to test connection with bitcoin network


Example for running a node in regtest mode mapping JSON-RPC/REST (18443) and P2P (18444) ports:

docker run --rm -it \
  -p 18443:18443 \
  -p 18444:18444 \
  ruimarinho/bitcoin-core \
  -printtoconsole \
  -regtest=1 \
  -rpcallowip=172.17.0.0/16 \
  -rpcbind=0.0.0.0 \
  -rpcauth='foo:7d9ba5ae63c3d4dc30583ff4fe65a67e$9e3634e81c11659e3de036d0bf88f89cd169c1039e6e09607562d54765c649cc'

To test that mapping worked, you can send a JSON-RPC curl request to the host port:

curl --data-binary '{"jsonrpc":"1.0","id":"1","method":"getnetworkinfo","params":[]}' http://foo:qDDZdeQ5vw9XXFeVnXT4PZ--tGN2xNjjR4nrtyszZx0=@127.0.0.1:18443/

Mainnet

    JSON-RPC/REST: 8332
    P2P: 8333

Testnet

    Testnet JSON-RPC: 18332
    P2P: 18333

Regtest

    JSON-RPC/REST: 18443 (since 0.16+, otherwise 18332)
    P2P: 18444

Signet

    JSON-RPC/REST: 38332
    P2P: 38333




To run to the bitcoin-cli we have first to build it with 
    cargo build --release
Then we can navigate to the target folder and run two commands
    
    ./bitcoin-cli --ip 127.0.0.1 --port 18444 --network regtest 

    Config { ip: "127.0.0.1", port: "18444", network: "regtest" }
    Sent version message
    Received version message: Version(VersionMessage { version: 70016, services: ServiceFlags(1033), timestamp: 1679950273, receiver: Address {services: ServiceFlags(NONE), address: 0.0.0.0, port: 0}, sender: Address {services: ServiceFlags(NETWORK|WITNESS|NETWORK_LIMITED), address: 0.0.0.0, port: 0}, nonce: 10674837972924403938, user_agent: "/Satoshi:24.0.1/", start_height: 0, relay: true })
    Sent verack message
    Received verack message: Verack

or
    /bitcoin-cli --help

A CLI for testing bitcoin node connection

Usage: bitcoin-cli --ip <IP_ADDRESS> --port <PORT_NUMBER> --network <BITCOIN_NETWORK>

Options:
      --ip <IP_ADDRESS>            The ip address
      --port <PORT_NUMBER>         The port number
      --network <BITCOIN_NETWORK>  The Bitcoin network (e.g. regtest)
  -h, --help                       Print help
  -V, --version                    Print version
kami@kami1:~/Documents/Projects/Rust-Projects/bitcoin-
