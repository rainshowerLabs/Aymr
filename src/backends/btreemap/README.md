# `btreemap`

This is a basic impl of the aymr DB traits for `std::collections::BTreeMap`.

Using a BTreeMap is optimal when:

- You want a map sorted by its keys.
- You want to be able to get a range of entries on-demand.
- You’re interested in what the smallest or largest key-value pair is.
- You want to find the largest or smallest key that is smaller or larger than something.
- Are doing more reads than writes.

In addition, the design `sled` uses is based off of a BTreeMap impl.

Aymr btreemap does not flush to disk. Every change you have is exclusively going to be in-memory.
