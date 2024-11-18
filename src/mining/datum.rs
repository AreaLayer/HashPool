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