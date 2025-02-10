use crate::block::Block;
use crate::transaction::Transaction;

pub struct BitAxe {
    pub blocks: Vec<Block>,
    pub transactions: Vec<Transaction>,
}

pub fn new() -> BitAxe {
    BitAxe {
        blocks: Vec::new(),
        transactions: Vec::new(),
    }
}

impl BitAxe {
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

pub fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }
pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

impl Default for BitAxe {
    fn default() -> Self {
        Self::new()
    }
}