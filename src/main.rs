extern crate log;
extern crate simple_logger;

use log::{trace,info};
use bitcoin_client::BitcoinClient;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting Bitcoin Client Daemon...");
    let mut bitcoin_client = BitcoinClient::new_authenticated("ra".to_string(), "1234".to_string());
    bitcoin_client.init();
    trace!("Bitcoin Client Daemon Stopped.");
}
