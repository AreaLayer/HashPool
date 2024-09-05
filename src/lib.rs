extern crate crypto;
extern crate sha2;

// Removed unused import
use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};



pub (crate) struct Miner {
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

    pub (crate) fn hash_rate(&self) -> u64 {
        self.hash_rate.load(Ordering::SeqCst) as u64
    }

    pub (crate) fn mine(&self, nonce: usize) -> bool {
        // Example placeholder implementation of mining logic
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(format!("{}", nonce).as_bytes());
        let hash_result = format!("{:x}", hasher.finalize());

        // Convert hash_result from hexadecimal string to u64
        let hash_as_u64 = u64::from_str_radix(&hash_result[..16], 16).unwrap_or(0);

        // Check if hash_as_u64 is divisible by self.difficulty
        let success = hash_as_u64 % self.difficulty == 0;

        success
    }}


