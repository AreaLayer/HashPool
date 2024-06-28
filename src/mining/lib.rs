use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Miner {
    pub id: usize,
    pub hash_power: usize,
    pub hash_rate: AtomicUsize,
    pub shares: Arc<HashMap<usize, usize>>,
    pub difficulty: u64,
}

impl Miner {
    pub fn new(id: usize, hash_power: usize) -> Self {
        Miner {
            id,
            hash_power,
            hash_rate: AtomicUsize::new(0),
            shares: Arc::new(HashMap::new()),
            difficulty: 1,
        }
    }

    pub fn hash_rate(&self) -> usize {
        self.hash_rate.load(Ordering::SeqCst)
    }

    pub fn mine(&self, nonce: usize) -> bool {
        let hash = nonce; // Example: the actual hashing algorithm should replace this
        if hash < self.difficulty as usize {
            self.hash_rate.store(hash, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
}
