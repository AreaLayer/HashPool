use std::collections::HashMap;
use std::sync::Arc;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

fn main () {
    // Create a new cache.
    let cache = new_cache();
}
// Define the Cache type as an Arc (atomic reference counting pointer) 
// around a Mutex-protected HashMap.
pub type Cache = Arc<Mutex<HashMap<String, String>>>;

// Define the CacheGuard type as a MutexGuard around a HashMap, with the 
// same lifetime as the referenced Cache.
pub type CacheGuard<'a> = MutexGuard<'a, HashMap<String, String>>;

// Function to create a new empty Cache.
pub fn new_cache() -> Cache {
    Arc::new(Mutex::new(HashMap::new()))
}

// Function to obtain a CacheGuard, locking the cache for access.
pub fn get_cache_guard<'a>(cache: &'a Cache) -> CacheGuard<'a> {
    cache.lock().unwrap()
}

// Function to retrieve a value from the cache. 
// Returns an Option containing the value, or None if the key is not present.
pub fn get_cache_value(cache: &Cache, key: &str) -> Option<String> {
    let guard = get_cache_guard(cache);
    guard.get(key).cloned()
}

// Function to insert a value into the cache.
pub fn set_cache_value(cache: &Cache, key: &str, value: &str) {
    let mut guard = get_cache_guard(cache);
    guard.insert(key.to_string(), value.to_string());
}
