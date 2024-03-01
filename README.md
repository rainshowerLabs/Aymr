# Aymr - Database abstraction library for hot-swapping KV databases

Aymr is a wrapper library for simply switching between multiple KV-databases, as well as accessing them over a network.  

The main goal of this project is to unify multiple different KV DB APIs under a single common API. This idea came about when migrating from sled `0.34` to `1.0-alpha`. Both of them had the same underlying functionality, but unfortunately slightly different APIs. Becasue of this, the idea of an intermediary common API unifiying them was born.

The statement that most embedded KV databases function identically, with different APIs being the key difference with how developers interact with them generally holds true. And thus Aymr was spun into it's own project to support as many embedded DBs as it makes sense to support.

It's important to note there's a lot of nuance between different KV databases, and that Aymr will never be able to capture all of their various quirks and semantics under a single API.
