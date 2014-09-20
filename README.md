
## Overview

Quicksilver is a library of approximate algorithm and sketches

The algorithms contained within this library are designed to calculate common metrics and statistics
(such as cardinality, frequency, etc) in an approximate fashion.  In exchange for decreased accuracy,
these algorithms typically have minimal memory overhead or are very fast.  Or both.

Build status with Rust Nightly: [![Build Status](https://travis-ci.org/polyfractal/quicksilver.svg?branch=master)](https://travis-ci.org/polyfractal/quicksilver)

## Public modules

### HLL - HyperLogLog

Approximates cardinality estimation with minimal memory overhead

This implements [HyperLogLog](http://algo.inria.fr/flajolet/Publications/FlFuGaMe07.pdf), an algorithm
which provides a reasonably accurate estimation of cardinality.  It is very fast (200m op/s on my Macbook Air)
and requires minimal memory.  It can estimate the cardinality of sets that contain billions of entries.

See [this article](http://research.neustar.biz/2012/10/25/sketch-of-the-day-hyperloglog-cornerstone-of-a-big-data-infrastructure/)
for a good laymen explanation of HyperLogLog.

#### Usage

```rust
use quicksilver::hll::HLL;
use std::hash::Hash;

let mut hll = HLL::new(12);

for i in range(0u, 1000000) {
  let hash = hash::hash(&i);
  hll.offer_hashed(hash);
}

let cardinality = hll.cardinality();
```