use std::sync::{Arc, Mutex};
use std::collections::HashMap;
pub struct ASIC {
    pub id: usize,
    pub hash_rate: usize,
    pub hash_rate_lock: Arc<Mutex<usize>>,
    pub shares: Arc<Mutex<HashMap<usize, usize>>>,
    pub difficulty: u64,
}

impl ASIC {
    pub fn new(id: usize, hash_rate: usize) -> Self {
        ASIC {
            id,
            hash_rate,
            hash_rate_lock: Arc::new(Mutex::new(0)),
            shares: Arc::new(Mutex::new(HashMap::new())),
            difficulty: 1,
        }
    }
    pub fn hash_rate(&self) -> u64 {
        *self.hash_rate_lock.lock().unwrap() as u64
    }

    pub fn mine(&self, nonce: usize) -> bool {
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
    }
}
