use std::result::Result;

use crate::aymr_db::error::Error;

/// The type representing an inline array of bytes.
pub type InlineArray = Vec<u8>;

/// Trait for marking that the database we're opening is compatible with Aymr.
pub trait AymrOpenable {
    fn open() -> Self;
}

/// Aymr database trait. Represents an easily implementable,
/// common API for various KV databases.
pub trait AymrDatabase<K,V>
where
    K: AsRef<[u8]>,
    V: AsRef<[u8]>
{
    /// Clears the entire database, removing all values.
    fn clear(&mut self) -> Result<(), Error>;

    /// Returns the number of elements in the database.
    fn len(&self) -> usize;

    /// Returns true if the database is empty, false otherwise.
    fn is_empty(&self) -> Result<bool, Error>;

    /// Retrieves a value from the database if it exists.
    fn get(&self, key: &K) -> Result<Option<InlineArray>, Error>;

    /// Inserts a new key-value pair into the database, returning the old value if it was set.
    fn insert(&mut self, key: K, value: V) -> Result<Option<InlineArray>, Error>;

    /// Removes a key-value pair from the database, returning the old value if it existed.
    fn remove(&mut self, key: K) -> Result<Option<InlineArray>, Error>;

    /// Applies a batch of operations to the database atomically.
    fn apply_batch<B: Batch>(&self, batch: B) -> Result<(), Error>;

    /// Returns true if the database contains a value for the specified key.
    fn contains_key(&self, key: &K) -> Result<bool, Error>;
}

/// A trait representing a batch of operations to be applied to a database.
pub trait Batch {
    /// Adds a clear operation to the batch.
    fn clear(&mut self);

    /// Adds an insert operation to the batch.
    fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>;

    /// Adds a remove operation to the batch.
    fn remove<K: AsRef<[u8]>>(&mut self, key: K);
}

/// Trait implemented by databases that can flush to disk.
pub trait AymrFlush {
    /// Flushes all dirty IO buffers and fsyncs.
    fn flush(&self) -> Result<(), Error>;
}

/// Marker trait to denote that a type is designed to work
/// with Aymr.
pub trait AymrConfig {}

