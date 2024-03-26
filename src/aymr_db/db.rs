use std::sync::Arc;

use crate::btreemap::db::AymrBtreeMap;

use super::{
    error::Error,
    traits::{
        AymrDatabase,
        AymrOpenable,
        InlineArray,
    },
};

#[derive(Debug)]
pub struct AymrDb<K, V> {
    #[feature(btreemap)]
    db: AymrBtreeMap<K, V>,
}

impl<K, V> AymrDb<K, V> {
    #[feature(btreemap)]
    fn open() -> Arc<Self> {
        
        let rax = AymrDb {
            db: AymrBtreeMap::open(),
        };

        Arc::new(rax)
    }
}

impl<K, V> AymrDatabase<K, V> for AymrDb<K, V>
where
    K: AsRef<[u8]> + Ord,
    V: AsRef<[u8]>,
{
    fn clear(&mut self) -> Result<(), Error> {
        self.db.clear()
    }

    fn len(&self) -> usize {
        self.db.len()
    }

    fn is_empty(&self) -> Result<bool, Error> {
        self.db.is_empty()
    }

    fn get(&self, key: &K) -> Result<Option<InlineArray>, Error> {
        self.db.get(key)
    }

    fn insert(&mut self, key: K, value: V) -> Result<Option<InlineArray>, Error> {
        self.db.insert(key, value)
    }

    fn remove(&mut self, key: K) -> Result<Option<InlineArray>, Error> {
        self.db.remove(key)
    }

    fn apply_batch<B: super::traits::Batch>(&self, batch: B) -> Result<(), Error> {
        self.db.apply_batch(batch)
    }

    fn contains_key(&self, key: &K) -> Result<bool, Error> {
        self.db.contains_key(key)
    }
}
