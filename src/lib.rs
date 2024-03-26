//! # Aymr - Database abstraction library for hot-swapping KV databases
//!
//! ***Aymr is a work in progress! It cannot be used in production (for now!)***
//!
//! Aymr is a wrapper library for simply switching between multiple KV-databases,
//! as well as accessing them over a network.  
//!
//! The main goal of this project is to unify multiple different KV DB APIs under
//! a single common API. This idea came about when migrating from sled `0.34` to `1.0-alpha`.
//! Both of them had the same underlying functionality, but unfortunately slightly different APIs.
//! Becasue of this, the idea of an intermediary common API unifiying them was born.
//!
//! The statement that most embedded KV databases function identically, with different
//! APIs being the key difference with how developers interact with them generally holds true.
//! And thus Aymr was spun into it's own project to support as many embedded DBs as it makes sense to support.
//!
//! It's important to note there's a lot of nuance between different KV databases, and
//! that Aymr will never be able to capture all of their various quirks and semantics under a single API.
//!
//! ## Use
//!
//! Aymr is a rust library and can be imported as such. In order to select the database
//! you want to use, please use the appropriate feature flag. For now we support the following:
//!
//! - [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
//!
//! Because database configs have little in common with each other, the config for every
//! Aymr Db will be different. Please consult the Aymr docs for more info in how to set up and run each DB.
//!
//! ## Examples
//!
//! Examples for using Aymr can be found under the `/examples` directory. You can use cargo to see Aymr in action:   
//!
//! ```bash
//! cargo run --example basic_db_usage
//! ```

pub mod aymr_db;
mod btreemap;
mod hashmap;
