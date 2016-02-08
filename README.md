crdt-rs
============
This is a library to work with Context Free Replicated Data Types (CRDT)[1] in rust

Installation
=============
This crate works with Cargo. Assuming you have Rust and Cargo installed, simply check out the source and run tests:
```
git clone https://github.com/achanda/crdt-rs
cd crdt-rs
cargo test
```

You can also add `crdt_rs` as a dependency to your project's `Cargo.toml`:
```
[dependencies]
crdt_rs = "*"
```

TODO
===============
1. Tests for all types
2. Documentation
3. More utility functions for all types
4. More types

[1]: https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type
