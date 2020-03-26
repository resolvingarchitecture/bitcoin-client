use log::{info,warn};

use ra_common::models::{Packet};
use bitcoincore_rpc::{Auth, Client, RpcApi};

pub struct BitcoinClient {
    rpc: Client,
    local_node_running: bool
}

impl BitcoinClient {
    pub fn new() -> BitcoinClient {
        BitcoinClient {
            rpc: Client::new("http://localhost:8332".to_string(),
                             Auth::UserPass(String::from(""),
                                            String::from(""))).unwrap(),
            local_node_running: false
        }
    }
    pub fn new_authenticated(username: String, passphrase: String) -> BitcoinClient {
        BitcoinClient {
            rpc: Client::new("http://localhost:8332".to_string(),
                             Auth::UserPass(username.to_string(),
                                            passphrase.to_string())).unwrap(),
            local_node_running: false
        }
    }
    pub fn init(&mut self) {
        info!("{}","Initializing Bitcoin Client...");
        match self.rpc.get_block_count() {
            Ok(n) => {
                self.local_node_running = true;
                info!("block count: {}", n);
            },
            Err(e) => warn!("error: {}",e)
        }
    }
    pub fn handle(&mut self, packet: &mut Packet) {
        if !self.local_node_running {
            warn!("Local Bitcoin instance not running");
        }
        info!("Handling incoming packet id={}", packet.id);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
