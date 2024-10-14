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

impl ElectrumClient {
    pub fn new(url: &str) -> Result<Self, Error> {
        let client = Client::new(url)?;
        Ok(Self { client })
    }
    pub fn get_block_height(&self) -> Result<u64, Error> {
        let block_height = self.client.get_block_height()?;
        Ok(block_height)
    }
    pub fn get_block_hash(&self, block_height: u64) -> Result<String, Error> {
        let block_hash = self.client.get_block_hash(block_height)?;
        Ok(block_hash)
    }
    pub fn get_block(&self, block_hash: &str) -> Result<String, Error> {
        let block = self.client.get_block(block_hash)?;
        Ok(block)
    }
    pub fn get_transaction(&self, tx_hash: &str) -> Result<String, Error> {
        let tx = self.client.get_transaction(tx_hash)?;
        Ok(tx)
    }
    pub fn get_address(&self, address: &str) -> Result<String, Error> {
        let address = self.client.get_address(address)?;
        Ok(address)
    }
    pub fn get_balance(&self, address: &str) -> Result<u64, Error> {
        let balance = self.client.get_balance(address)?;
        Ok(balance)
    }
    pub fn get_utxos(&self, address: &str) -> Result<Vec<Utxo>, Error> {
        let utxos = self.client.get_utxos(address)?;
        Ok(utxos)
    }
}