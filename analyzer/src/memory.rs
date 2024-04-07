use crate::indexer::{IndexData, IndexStore};

pub struct InMemoryIndexStore {
    inner: std::collections::HashMap<String, IndexData>,
}

impl InMemoryIndexStore {
    pub fn new() -> Self {
        InMemoryIndexStore {
            inner: std::collections::HashMap::new(),
        }
    }
}

impl IndexStore for InMemoryIndexStore {
    fn set(&mut self, key: &str, value: IndexData) {
        self.inner.insert(key.to_string(), value);
    }

    fn get(&self, key: &str) -> String {
        self.inner.get(key).unwrap().value.clone()
    }
}
