use bitcoin::{
    BlockHash,
    Transaction,
};

use datum::Transaction;

pub type Datum = Transaction;

pub (crate) fn hash(datum: &Datum) -> BlockHash {
    datum.hash()
}
