#![warn(missing_docs)]
#![feature(custom_derive)]
#![allow(unused_attributes)]

//! This crate provides implementations of some common
//! [CRDT](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type)

mod gset;
mod pset;

pub use gset::GSet;
pub use pset::PSet;
