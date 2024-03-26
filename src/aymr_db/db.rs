//! `Aymr DB`
//!
//! The `AymrDb` type is a wrapper used to access databases that implement
//! the Aymr set of traits. Opening is achieved via calling the open fn which
//! will return a referance to the Db. Due to different databases having different
//! ways of being opened, this is managed by feature flags that correspond with your
//! desired database.
//!
//! The database cannot be dereferanced and may not be `Sync`. Aymr being `Sync`
//! depends on if the underlying database that's used is `Sync`. If you require
//! `Sync` on a non-`Sync` database backend, please wrap it in a `Mutex`, `RwLock`,
//! `UnsafeBox` or whatever `Sync`-able type you preffer. Please keep in mind the
//! semantics of said type.
//!
//! Aymr does not offer any additional data safety or guarantees not provided by
//! the underlying database.


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
