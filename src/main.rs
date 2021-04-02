extern crate log;
extern crate simple_logger;

use log::{trace,info};
use bitcoin_client::BitcoinClient;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting Bitcoin Client Daemon...");
    let DEV = "http://localhost:18443";
    let TEST = "http://localhost:18332";
    let PROD = "http://localhost:8332";
    let mut bitcoin_client = BitcoinClient::new_authenticated(DEV.to_string(),"ra".to_string(), "1234".to_string());
    bitcoin_client.init();
    trace!("Bitcoin Client Daemon Stopped.");
}
