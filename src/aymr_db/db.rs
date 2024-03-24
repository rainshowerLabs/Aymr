use crate::btreemap::db::AymrBtreeMap;

use super::traits::AymrDatabase;

#[derive(Debug)]
pub struct AymrDb<K,V> {
	#[feature(btreemap)]
	db: AymrBtreeMap<K, V>,
}

impl<K, V> AymrDatabase<K, V> for AymrDb<K, V>
where
    K: AsRef<[u8]> + Ord,
    V: AsRef<[u8]>,
{
    fn clear(&mut self) -> Result<(), super::error::Error> {
        self.db.clear()
    }

    fn len(&self) -> usize {
        self.db.len()
    }

    fn is_empty(&self) -> Result<bool, super::error::Error> {
        self.db.is_empty()
    }

    fn get(&self, key: &K) -> Result<Option<super::traits::InlineArray>, super::error::Error> {
        self.db.get(key)
    }

    fn insert(&mut self, key: K, value: V) -> Result<Option<super::traits::InlineArray>, super::error::Error> {
        self.db.insert(key, value)
    }

    fn remove(&mut self, key: K) -> Result<Option<super::traits::InlineArray>, super::error::Error> {
        self.db.remove(key)
    }

    fn apply_batch<B: super::traits::Batch>(&self, batch: B) -> Result<(), super::error::Error> {
        self.db.apply_batch(batch)
    }

    fn contains_key(&self, key: &K) -> Result<bool, super::error::Error> {
        self.db.contains_key(key)
    }
}
