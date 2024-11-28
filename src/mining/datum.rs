use bitcoin::{
    BlockHash,
    Transaction,
};

use datum::Transaction;

pub type Datum = Transaction;

pub (crate) fn hash(datum: &Datum) -> BlockHash {
    datum.hash()
}


pub (crate) fn size(datum: &Datum) -> usize {
    datum.size()
}

pub struct DatumIterator {
    datum: Datum,
    index: usize,
    txs: Vec<Transaction>,
}

pub fn new(datum: Datum) -> DatumIterator {
    DatumIterator {
        datum,
        index: 0,
        txs: Vec::new(),
    }
}