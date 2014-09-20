
## Overview

Quicksilver is a library of approximate algorithm and sketches

The algorithms contained within this library are designed to calculate common metrics and statistics
(such as cardinality, frequency, etc) in an approximate fashion.  In exchange for decreased accuracy,
these algorithms typically have minimal memory overhead or are very fast.  Or both.

Build status with Rust Nightly: [![Build Status](https://travis-ci.org/polyfractal/quicksilver.svg?branch=master)](https://travis-ci.org/polyfractal/quicksilver)

##Documentation

API Docs are auto-generated after every commit: http://www.rust-ci.org/polyfractal/quicksilver/doc/quicksilver/

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

#### Precision vs Memory chart

<table><tr><td style="width:100px; font-weight: bold">Precision</td><td style="width:150px; font-weight: bold">Size</td><td style="width:300px; font-weight: bold">Worst Case Estimate Accuracy (+/-)</td>
<tr><td>4</td><td>48 bytes</td><td>39.00%</td></tr>
<tr><td>5</td><td>60 bytes</td><td>27.50%</td></tr>
<tr><td>6</td><td>84 bytes</td><td>19.50%</td></tr>
<tr><td>7</td><td>136 bytes</td><td>13.78%</td></tr>
<tr><td>8</td><td>240 bytes</td><td>9.75%</td></tr>
<tr><td>9</td><td>444 bytes</td><td>6.89%</td></tr>
<tr><td>10</td><td>852 bytes</td><td>4.8%</td></tr>
<tr><td>11</td><td>1672 bytes</td><td>3.44%</td></tr>
<tr><td>12</td><td>3312 bytes</td><td>2.43%</td></tr>
<tr><td>13</td><td>6588 bytes</td><td>1.72%</td></tr>
<tr><td>14</td><td>13140 bytes</td><td>1.21%</td></tr>
<tr><td>15</td><td>26248 bytes</td><td>0.86%</td></tr>
<tr><td>16</td><td>52464 bytes</td><td>0.60%</td></tr>
<tr><td>17</td><td>104892 bytes</td><td>0.43%</td></tr>
<tr><td>18</td><td>209748 bytes</td><td>0.30%</td></tr>
</table>