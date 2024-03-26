//! # `hashmap`
//! 
//! This is a basic impl of the aymr DB traits for `std::collections::HashMap`.
//! 
//! Using a BTreeMap is optimal when:
//! 
//! - You want to associate arbitrary keys with an arbitrary value.
//! - You want a cache.
//! - You want a map, with no extra functionality.

//! Aymr HashMap does not flush to disk. Every change you have is exclusively going to be in-memory.

use std::collections::HashMap;

use crate::aymr_db::{
    error::Error,
    traits::{
        AymrDatabase,
        AymrOpenable,
        Batch,
        InlineArray,
    },
};

/// Struct implementing `AymrDatabase` and `AymrOpenable` over `HashMap`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AymrHashMap<K, V>
where
	K: Eq + std::hash::Hash,
{
    db: HashMap<K, V>,
}

impl<K, V> AymrDatabase<K, V> for AymrHashMap<K, V>
where
    K: AsRef<[u8]> + Ord + std::hash::Hash,
    V: AsRef<[u8]>,
{
    fn clear(&mut self) -> Result<(), Error> {
        self.db.clear();
        Ok(())
    }

    fn len(&self) -> usize {
        self.db.len()
    }

    fn is_empty(&self) -> Result<bool, Error> {
        Ok(self.db.is_empty())
    }

    fn get(&self, key: &K) -> Result<Option<InlineArray>, Error> {
        let rax = self.db.get(key).map(|v| v.as_ref().to_vec());
        Ok(rax)
    }

    fn insert(&mut self, key: K, value: V) -> Result<Option<InlineArray>, Error> {
        Ok(self.db.insert(key, value).map(|v| v.as_ref().to_vec()))
    }

    fn remove(&mut self, key: K) -> Result<Option<InlineArray>, Error> {
        Ok(self.db.remove(&key).map(|v| v.as_ref().to_vec()))
    }

    fn apply_batch<B: Batch>(&self, _batch: B) -> Result<(), Error> {
        // Implement apply_batch logic here
        todo!()
    }

    fn contains_key(&self, key: &K) -> Result<bool, Error> {
        Ok(self.db.contains_key(key))
    }
}

impl<K, V> AymrOpenable for AymrHashMap<K, V>
where
	K: Eq + std::hash::Hash,
{
    fn open() -> Self {
        let db = HashMap::new();
        AymrHashMap { db }
    }
}
