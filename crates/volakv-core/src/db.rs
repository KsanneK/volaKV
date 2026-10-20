use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub struct Db {
    map: Arc<RwLock<HashMap<String, bytes::Bytes>>>
}

impl Db {
    pub fn new() -> Self {
        Self { map: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn set(&self, key: String, value: bytes::Bytes) {
        self.map.write().unwrap().insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<bytes::Bytes> {
        self.map.read().unwrap().get(key).cloned()
    }

    pub fn del(&self, key: &str) -> bool {
        self.map.write().unwrap().remove(key).is_some()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.map.read().unwrap().contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.map.read().unwrap().len()
    }
}
