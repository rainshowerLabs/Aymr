# `hashmap`

This is a basic impl of the aymr DB traits for `std::collections::HashMap`.

Using a BTreeMap is optimal when:

- You want to associate arbitrary keys with an arbitrary value.
- You want a cache.
- You want a map, with no extra functionality.

Aymr HashMap does not flush to disk. Every change you have is exclusively going to be in-memory.
