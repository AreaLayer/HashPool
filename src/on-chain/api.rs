extern crate bdk;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use bdk::bitcoin::network::constants::Network;
use bdk::Wallet;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// Blockchain API response struct
#[derive(Debug, Serialize, Deserialize)]
struct BlockApiResponse {
    hash: String,
    version: u32,
    prev_blockhash: String,
    merkle_root: String,
    time: u64,
    bits: u32,
    nonce: u32,
}

fn main() {
    // Set up the BDK wallet
    let descriptor = "wpkh(tprv8ZgxMBicQKsPdY5HfY4Upd3h3tqDnxDaJkV5mbD66FhUzFJjGSV9aG7rJ3mDnAtwqX8J4T5Hf5X1sgeTrmFa6cav8JCYeNX1LG9HJWx7gZU/0/*)";
    let network = Network::Testnet;
    let wallet = Wallet::new_offline(descriptor, None, network, None).unwrap();

    // Generate a new private key
    let private_key = wallet.keychain(network).derive_private_key(0).unwrap();

    // Convert the private key to a byte array
    let private_key_bytes = private_key.private_key.to_bytes();

    // Calculate the SHA256 hash
    let sha256_hash = sha256::Hash::hash(&private_key_bytes);
    println!("SHA256 Hash: {:?}", sha256_hash);

    // Fetch block data from a blockchain API
    let block_hash = "0000000000000000000000000000000000000000000000000000000000000000";
    let api_url = format!("https://blockchain.info/rawblock/{}", block_hash);
    let client = Client::new();
    let response = client.get(&api_url).send().expect("Failed to fetch data");

    // Deserialize the JSON response
    let block_data: BlockApiResponse = serde_json::from_str(&response.text().expect("Failed to read response")).expect("Failed to deserialize JSON");

    // Display block data
    println!("Block Hash: {}", block_data.hash);
    println!("Version: {}", block_data.version);
    println!("Previous Block Hash: {}", block_data.prev_blockhash);
    println!("Merkle Root: {}", block_data.merkle_root);
    println!("Timestamp: {}", block_data.time);
    println!("Difficulty Target: {}", block_data.bits);
    println!("Nonce: {}", block_data.nonce);
}

