use std::{
    collections::BTreeMap,
};

use crate::aymr_db::{
    db::{
        AymrDatabase,
        AymrOpenable,
        InlineArray,
        Batch,
    },
    error::Error,
};

#[derive(Debug)]
struct AymrBtreeMap<K, V> {
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
