# Aymr API referance

## Open/Create

Pretty much every kv DB has its own way to open/create a DB. For this reason Aymr just mirrors the underlying API for that DB via a feature flag.

## Using the DB

Once you open or create a DB you should be able to interact with it under a common API:


### `clear(&self) -> Result<()>`

Clears the entire DB, removing all values.

### `len(&self) -> usize` and `is_empty(&self) -> Result<bool>`

Returns the number of elements and if the DB is empty/

### `flush(&self) -> Result<()>`

Flushes all dirty IO buffers and fsyncs.

### `get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<InlineArray>>`

Retrieves a value from the Db if it exists.

### `insert<K, V>(&self, key: K, value: V) -> Result<Option<InlineArray>>`

Insert a new key to a new value, returning the last value if it was set.

### `remove<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<InlineArray>>`

Delete a value, returning the old value if it existed.

### `apply_batch(&self, batch: Batch) -> Result<()>`

Create a new batched update that is applied atomically. Readers will atomically see all updates at an atomic instant, and if the database crashes, either 0% or 100% of the full batch will be recovered, but never a partial batch. If a flush operation succeeds after this, it is guaranteed that 100% of the batch will be visible, unless later concurrent updates changed the values before the flush.

### `contains_key<K: AsRef<[u8]>>(&self, key: K) -> Result<bool>`

Returns `true` if the DB contains a value for the specified key.

## Batches

Batches are multiple DB operations rolled up into one Db update. For databases that support batched operations, their respective batching methods will be used. For databases that don't, we preform basic sequential operations for the batch.
