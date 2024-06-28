use std::collections::HashMap;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

 pub struct Miner {
    pub id: usize,
 pub struct Miner {
     pub hash_power: usize,
     pub hash_rate: AtomicUsize,
     pub shares: Arc<HashMap<usize, usize>>,
   pub difficulty: usize,
   pub difficulty: u64,
 }
}

 impl Miner {

    pub fn new(id: usize, hash_power: usize) -> Self {
 impl Miner {
         difficulty: 1,
     }
    }
}

     pub fn hash_rate(&self) -> usize {
-       self.hash_rate.load(Ordering::SeqCst)
       self.hash_rate.load(Ordering::SeqCst) as u64
     }

     pub fn mine(&self, nonce: usize) -> bool {

        let mut hash = nonce;
 impl Miner {
         self.hash_rate.store(hash_rate, Ordering::SeqCst);
              true

         } else {
           false
           false
         }

        }
