use crate::Block;

extern crate bdk;
extern crate bitcoin;
extern crate bitcoin_hashes;

use bdk::bitcoin::network::constants::Network;
use bitcoin::blockdata::block::Block;
use bitcoin::network::serialize::BitcoinHash;
use bitcoin::network::serialize::RawDecoder;
use bitcoin_hashes::sha256d;
use std::str::FromStr;

fn main() {
    // Hexadecimal representation of a Bitcoin block
    let block_hex = "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c";

    // Parse the block from hexadecimal
    let block_bytes = hex::decode(block_hex).expect("Failed to decode hex");
    let mut decoder = RawDecoder::new(&block_bytes);
    let block: Block = BitcoinHash::deserialize(&mut decoder).expect("Failed to parse block");

    // Display basic block information
    println!("Block Hash: {}", block.header.bitcoin_hash());
    println!("Version: {}", block.header.version);
    println!("Previous Block Hash: {}", block.header.prev_blockhash);
    println!("Merkle Root: {}", block.header.merkle_root);
    println!("Timestamp: {}", block.header.time);
    println!("Difficulty Target: {}", block.header.bits);
    println!("Nonce: {}", block.header.nonce);

    // Verify block header using BDK
    let network = Network::Bitcoin;
    let block_header = block.header;
    let mut verifier = bdk::blockchain::BlockVerifier::new(network);
    let result = verifier.verify(&block_header);
    println!("Block Header Verification Result: {:?}", result);
}

