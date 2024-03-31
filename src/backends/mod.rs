//! # Aymr backends
//! 
//! This mod contains various backends that can be used with Aymr.
//! 
//! Each mod inside contains the necessary bindings for each backend.
//! Currently the supported backends are:
//! 
//! - [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
//! - [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

pub mod btreemap;
pub mod hashmap;
