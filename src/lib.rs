#![warn(missing_docs)]

//! This crate provides implementations of some common
//! [CRDT](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type)
mod gset;
mod pset;

pub use gset::GSet;
pub use pset::PSet;
