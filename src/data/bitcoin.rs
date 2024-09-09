extern crate bitcoin;
extern crate hex;

use bitcoin::network::constants::Network;
use bitcoin::BlockExplorer;
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use bitcoin::network::address::Address as NetAddress;
use bitcoin::network::constants::Network::Testnet;
use bitcoin::network::constants::Network::Bitcoin;
use bitcoin::network::constants::Network::Regtest;
use bitcoin::network::constants::Network::Signet;
use std::str::FromStr;

fn main() {
    // Define a transaction hash
    let tx_hash_hex = "29cc75b63e75ec0db2067c3bcb21e95f258c02b967c3f79119a1dd5d8be1a318";
    println!("Transaction Hash: {}", tx_hash_hex);

    // Parse the transaction hash
    let tx_hash_bytes = hex::decode(tx_hash_hex).expect("Failed to decode hex");
    let tx_hash = bitcoin::BlockHash::from_slice(&tx_hash_bytes).expect("Invalid hash length");

    // Fetch transaction data from a block explorer
    let network = Network::Testnet; // Change to the desired network
    let explorer = BlockExplorer::new(network);
    let tx_data = explorer.transaction(&tx_hash).expect("Failed to fetch transaction data");

    // Display transaction data
    println!("Transaction Hash: {}", tx_data.txid);
    println!("Block Hash: {}", tx_data.block_hash);
    println!("Confirmations: {}", tx_data.confirmations);
    println!("Transaction Size: {} bytes", tx_data.size);
    println!("Inputs:");
    for input in tx_data.inputs {
        println!("  {}", input.previous_output);
    }
    println!("Outputs:");
    for output in tx_data.outputs {
        println!("  {}", output);
    }
}


 
