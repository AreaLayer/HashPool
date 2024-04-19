use crate::Hash;


extern crate bitcoin;
extern crate bitcoin_hashes;

use bitcoin::blockdata::block::BlockHeader;
use bitcoin::network::serialize::RawDecoder;
use bitcoin::network::serialize::BitcoinHash;
use bitcoin_hashes::sha256d;
use std::str::FromStr;

fn main() {
    // Hexadecimal representation of a Bitcoin block header
    let block_header_hex = "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c";

    // Parse the block header from hexadecimal
    let block_header_bytes = hex::decode(block_header_hex).expect("Failed to decode hex");
    let mut decoder = RawDecoder::new(&block_header_bytes);
    let block_header: BlockHeader = BitcoinHash::deserialize(&mut decoder).expect("Failed to parse block header");

    // Calculate the hash of the block header
    let hash = sha256d::Hash::hash(&block_header.serialize());
    println!("Block Header Hash: {}", hash);
}

