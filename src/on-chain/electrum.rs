extern crate bdk_electrum;
extern crate bitcoin;

fn main() {
    // Create an Electrum client
    let client = bdk_electrum::Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    let block_height = client.get_block_height().unwrap();
    println!("Block Height: {}", block_height);
    let block_hash = client.get_block_hash(block_height).unwrap();
    println!("Block Hash: {}", block_hash);
    let block = client.get_block(block_hash).unwrap();
    println!("Block: {}", block);
    let tx = client.get_transaction("transaction_hash").unwrap();
    println!("Transaction: {}", tx);
    let address = client.get_address("address").unwrap();
    println!("Address: {}", address);
}