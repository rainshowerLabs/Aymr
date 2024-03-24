//! `btreemap`
//! 
//! This is a basic impl of the aymr DB traits for `std::collections::BTreeMap`.
//! 
//! Using a BTreeMap is optimal when:
//! 
//! - You want a map sorted by its keys.
//! - You want to be able to get a range of entries on-demand.
//! - You’re interested in what the smallest or largest key-value pair is.
//! - You want to find the largest or smallest key that is smaller or larger than something.
//! - Are doing more reads than writes.
//! 
//! In addition, the design `sled` uses is based off of a BTreeMap impl.
//! 
//! Aymr btreemap does not flush to disk. Every change you have is exclusively going to be in-memory.

use std::{
    collections::BTreeMap,
};

use crate::aymr_db::{
    traits::{
        AymrDatabase,
        AymrOpenable,
        InlineArray,
        Batch,
    },
    error::Error,
};

/// Struct implementing `AymrDatabase` and `AymrOpenable` over `BTreeMap`.
#[derive(Debug)]
pub struct AymrBtreeMap<K, V> {
    db: BTreeMap<K, V>,
}

impl<K, V> AymrDatabase<K, V> for AymrBtreeMap<K, V>
where
    K: AsRef<[u8]> + Ord,
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
        let rax = self.db.get(&key).map(|v| v.as_ref().to_vec());
        Ok(rax)
    }

    fn insert(&mut self, key: K, value: V) -> Result<Option<InlineArray>, Error> {
        Ok(self.db.insert(key, value).map(|v| v.as_ref().to_vec()))
    }

    fn remove(&mut self, key: K) -> Result<Option<InlineArray>, Error> {
        Ok(self.db.remove(&key).map(|v| v.as_ref().to_vec()))
    }

    fn apply_batch<B: Batch>(&self, batch: B) -> Result<(), Error> {
        // Implement apply_batch logic here
        todo!()
    }

    fn contains_key(&self, key: &K) -> Result<bool, Error> {
        Ok(self.db.contains_key(&key))
    }
}

impl<K, V> AymrOpenable for AymrBtreeMap<K, V> {}
