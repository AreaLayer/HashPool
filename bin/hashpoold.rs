use std::collections::HashMap;
use std::sync::Arc;

pub type Cache = Arc<Mutex<HashMap<String, String>>>;
pub type CacheGuard<'a> = MutexGuard<'a, HashMap<String, String>>;

pub fn new_cache() -> Cache {
    Arc::new(Mutex::new(HashMap::new()))
}

pub fn get_cache_guard(cache: &Cache) -> CacheGuard {
    cache.lock().unwrap()
}

pub fn get_cache_value(cache: &Cache, key: &str) -> Option<String> {
    let guard = get_cache_guard(cache);
    guard.get(key).map(|v| v.clone())
}

pub fn set_cache_value(cache: &Cache, key: &str, value: &str) {
    let mut guard = get_cache_guard(cache);

    guard.insert(key.to_string(), value.to_string());
}