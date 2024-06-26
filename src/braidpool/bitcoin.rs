use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use bitcoin::RPC;
use bitcoin::network::constants::Network;

fn main() {
    let (tx, rx): (Sender<RPC>, Receiver<RPC>) = std::sync::mpsc::channel();

}
impl RPC {
    pub fn new(network: Network, url: String, user: String, pass: String) -> RPC {
        RPC {
            network: network,
            url: url,
            user: user,
            pass: pass,
        }
    }
}

struct BitcoinRPC {
    rpc: RPC,
    tx: Sender<RPC>,
    rx: Receiver<RPC>,
}

impl BitcoinRPC {
    pub fn new(network: Network, url: String, user: String, pass: String) -> BitcoinRPC {
        let (tx, rx): (Sender<RPC>, Receiver<RPC>) = std::sync::mpsc::channel();
        BitcoinRPC {
            rpc: RPC::new(network, url, user, pass),
            tx: tx,
            rx: rx,
        }
    }
}

impl BitcoinRPC {
    pub fn get_balance(&self) -> i64 {
        let mut rpc = self.rpc.clone();
        let mut balance = 0;
        loop {
            let balance = rpc.get_balance();
            if balance > 0 {
                return balance;
            }
        }
    }
}

impl BitcoinRPC {
    pub fn get_new_address(&self) -> String {
        let mut rpc = self.rpc.clone();
        let mut address = String::new();
        loop {
            let address = rpc.get_new_address();
            if address.len() > 0 {
                return address;
            }
        }
    }
}

impl BitcoinRPC {
    pub fn send_to_address(&self, address: String, amount: i64) -> bool {
        let mut rpc = self.rpc.clone();
        let mut sent = false;
        loop {
            let sent = rpc.send_to_address(address, amount);
            if sent {
                return sent;
            }
        }
    }
}

impl BitcoinRPC {
    pub fn get_transaction_count(&self) -> i64 {
        let mut rpc = self.rpc.clone();
        let mut tx_count = 0;
        loop {
            let tx_count = rpc.get_transaction_count();
            if tx_count > 0 {
                return tx_count;
            }
        }
    }
}

impl BitcoinRPC {
    pub fn get_transaction(&self, tx_id: String) -> String {
        let mut rpc = self.rpc.clone();
        let mut tx = String::new();
        loop {
            let tx = rpc.get_transaction(tx_id);
            if tx.len() > 0 {
                return tx;
            }
        }
    }
}

impl BitcoinRPC {
    pub fn get_block_count(&self) -> i64 {
        let mut rpc = self.rpc.clone();
        let mut block_count = 0;
        loop {
            let block_count = rpc.get_block_count();
            if block_count > 0 {
                return block_count;
            }
        }
    }
}