use std::result::Result;

use crate::aymr_db::error::Error;

/// The type representing an inline array of bytes.
pub type InlineArray = Vec<u8>;

/// Aymr database trait. Represents an easily implementable,
/// common API for various KV databases.
pub trait AymrDatabase {
    /// Clears the entire database, removing all values.
    fn clear(&self) -> Result<(), Error>;

    /// Returns the number of elements in the database.
    fn len(&self) -> usize;

    /// Returns true if the database is empty, false otherwise.
    fn is_empty(&self) -> Result<bool, Error>;

    /// Flushes all dirty IO buffers and fsyncs.
    fn flush(&self) -> Result<(), Error>;

    /// Retrieves a value from the database if it exists.
    fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<InlineArray>, Error>;

    /// Inserts a new key-value pair into the database, returning the old value if it was set.
    fn insert<K, V>(&self, key: K, value: V) -> Result<Option<InlineArray>, Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>;

    /// Removes a key-value pair from the database, returning the old value if it existed.
    fn remove<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<InlineArray>, Error>;

    /// Applies a batch of operations to the database atomically.
    fn apply_batch<B: Batch>(&self, batch: B) -> Result<(), Error>;

    /// Returns true if the database contains a value for the specified key.
    fn contains_key<K: AsRef<[u8]>>(&self, key: K) -> Result<bool, Error>;
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
