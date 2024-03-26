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

#[cfg(feature = "btreemap")]
use crate::btreemap::db::AymrBtreeMap;

#[cfg(feature = "hashmap")]
use crate::hashmap::db::AymrHashMap;

use super::{
    error::Error,
    traits::{
        AymrDatabase,
        AymrOpenable,
        InlineArray,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AymrDb<K, V>
where
    K: Eq + std::hash::Hash,
{
    #[cfg(feature = "btreemap")]
    db: AymrBtreeMap<K, V>,

    #[cfg(feature = "hashmap")]
    db: AymrHashMap<K, V>,
}

impl<K, V> AymrDb<K, V>
where
    K: Eq + std::hash::Hash,
{
    #[cfg(feature = "btreemap")]
    #[allow(dead_code)]
    pub fn open() -> Self {
        let rax = AymrDb {
            db: AymrBtreeMap::open(),
        };

        rax
    }

    #[cfg(feature = "hashmap")]
    #[allow(dead_code)]
    pub fn open() -> Self {

        let rax = AymrDb {
            db: AymrHashMap::open(),
        };

        rax
    }
}

impl<K, V> AymrDatabase<K, V> for AymrDb<K, V>
where
    K: AsRef<[u8]> + Ord + Eq + std::hash::Hash,
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

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create an instance of AymrDb for testing
    fn create_test_db<K: AsRef<[u8]> + Ord + std::hash::Hash, V: AsRef<[u8]>>() -> AymrDb<K, V> {
        AymrDb {
            #[cfg(feature = "btreemap")]
            db: AymrBtreeMap::open(),

            #[cfg(feature = "hashmap")]
            db: AymrHashMap::open(),
        }
    }

    #[test]
    fn test_clear() {
        let mut db = create_test_db::<Vec<u8>, Vec<u8>>();
        assert!(db.clear().is_ok());
    }

    #[test]
    fn test_len() {
        let db = create_test_db::<Vec<u8>, Vec<u8>>();
        assert_eq!(db.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let db = create_test_db::<Vec<u8>, Vec<u8>>();
        assert!(db.is_empty().unwrap());
    }

    #[test]
    fn test_get() {
        let db = create_test_db::<Vec<u8>, Vec<u8>>();
        assert!(db.get(&vec![1, 2, 3]).unwrap().is_none());
    }

    #[test]
    fn test_insert_and_get() {
        let mut db = create_test_db::<Vec<u8>, Vec<u8>>();
        let key = vec![1, 2, 3];
        let value = vec![4, 5, 6];
        assert!(db.insert(key.clone(), value).is_ok());
        assert!(db.get(&key).unwrap().is_some());
    }

    #[test]
    fn test_remove() {
        let mut db = create_test_db::<Vec<u8>, Vec<u8>>();
        let key = vec![1, 2, 3];
        let value = vec![4, 5, 6];
        db.insert(key.clone(), value).unwrap();
        assert_eq!(db.len(), 1);
        assert!(db.remove(key).unwrap().is_some());
        assert_eq!(db.len(), 0);
    }

    #[test]
    fn test_contains_key() {
        let mut db = create_test_db::<Vec<u8>, Vec<u8>>();
        let key = vec![1, 2, 3];
        let value = vec![4, 5, 6];
        db.insert(key.clone(), value).unwrap();
        assert!(db.contains_key(&key).unwrap());
    }

    #[test]
    fn test_clear_after_insert() {
        let mut db = create_test_db::<Vec<u8>, Vec<u8>>();

        let _ = db.insert("key".into(), "value".into());
        assert_eq!(db.len(), 1);

        assert!(db.clear().is_ok());
        assert_eq!(db.len(), 0);
        assert!(db.is_empty().unwrap());
    }
}
