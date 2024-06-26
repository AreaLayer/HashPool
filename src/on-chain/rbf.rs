use std::io::Read;
use std::io::Write;
use std::io::BufReader;


use bitcoin::BumpFee;
use bitcoin::RBF;
use bitcoin::consensus::encode::deserialize;
use bitcoin::consensus::decode::deserialize;
use bitcoin::network::constants::Network;
use bitcoin::Transaction;

fn BumpFee {
    let mut file = File::open("transaction.bin").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let tx:::Transaction = deserialize(&buf).unwrap();
    let mut tx = tx;
    tx.rbf = true;
    tx.bump_fee(1000).unwrap();
    let mut file = File::create("transaction.bin").unwrap();
    file.write_all(&serialize(&tx)).unwrap();
    println!("Transaction updated");
    println!("Transaction hash: {}", tx.txid());

}

impl BumpFee = {
    fn bump_fee(&mut self, fee_rate: u64) -> Result<Transaction, Error> {
        let mut rbf = RBF::new(self);
        rbf.set_fee_rate(fee_rate);
        rbf.bump_fee()
        Ok(rbf.transaction)
}
}
