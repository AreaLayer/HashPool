extern crate bdk;
extern crate bitcoin_hashes;

use bdk::blockchain::noop_progress;
use bdk::bitcoin::network::constants::Network;
use bdk::bitcoin::util::address::Address;
use bdk::bitcoin::util::key::PrivateKey;
use bdk::Wallet;
use bitcoin_hashes::Sha256;
use std::str::FromStr;

fn main() {
    // Set up the BDK wallet
    let descriptor = "wpkh(tprv8ZgxMBicQKsPdY5HfY4Upd3h3tqDnxDaJkV5mbD66FhUzFJjGSV9aG7rJ3mDnAtwqX8J4T5Hf5X1sgeTrmFa6cav8JCYeNX1LG9HJWx7gZU/0/*)";
    let network = Network::Testnet;
    let wallet = Wallet::new_offline(descriptor, None, network, None).unwrap();

    // Generate a new private key
    let private_key = wallet.keychain(network).derive_private_key(0).unwrap();
    println!("Private Key: {}", private_key);

    // Convert the private key to a byte array
    let private_key_bytes = private_key.private_key.to_bytes();

    // Calculate SHA256 hash
    let hash = sha256::Hash::hash(&private_key_bytes);
    println!("SHA256 Hash: {:?}", hash);
}

