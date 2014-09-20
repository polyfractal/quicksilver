#![crate_name = "quicksilver"]
#![desc = "Quicksilver - a library of approximate algorithms and sketches"]
#![license = "MIT/ASL2"]
#![crate_type = "rlib"]
#![feature(phase)]

//#![deny(missing_doc)]


//! Quicksilver is a library of approximate algorithm and sketches
//!
//! The algorithms contained within this library are designed to calculate common metrics and statistics
//! (such as cardinality, frequency, etc) in an approximate fashion.  In exchange for decreased accuracy,
//! these algorithms typically have minimal memory overhead or are very fast.  Or both
//!

#[phase(plugin, link)]
#[allow(experimental)]
extern crate log;
extern crate core;
extern crate time;
extern crate test;



pub use hll::{HLL};
pub use pcsa::PCSA;

/// HyperLogLog - Approximates cardinality estimation with minimal memory overhead
pub mod hll;

/// Probalistic Counter with Stochastic Averaging - Approximate cardinality estimation
pub mod pcsa;

