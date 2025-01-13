use crate::block::Block;
use crate::transaction::Transaction;
pub struct BitAxe {
    pub blocks: Vec<Block>,
    pub transactions: Vec<Transaction>,
}