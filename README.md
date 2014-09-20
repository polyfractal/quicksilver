
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
  hll.offer_hashed(&hash);
}

let cardinality = hll.cardinality();
```

#### Precision vs Memory chart

<table><tr><td style="width:100px; font-weight: bold">Precision</td><td style="width:150px; font-weight: bold">Size</td><td style="width:300px; font-weight: bold">Worst Case Estimate Error (+/-)</td>
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


### PCSA - Probabilistic Counting with Stochastic Averaging

Implements the Probabilistic Counting with Stochastic Averaging counter.  PCSA provides an approximate estimate of cardinality with
bounded error.  Relative error is 0.78 / sqrt(m)

See this [article](http://blog.aggregateknowledge.com/2013/04/02/sketch-of-the-day-probabilistic-counting-with-stochastic-averaging-pcsa/) for a layman's explanation of PCSA

Original paper can be found [here](http://www.mathcs.emory.edu/~cheung/papers/StreamDB/Probab/1985-Flajolet-Probabilistic-counting.pdf)

*Note: PCSA is generally inferior to HLL in estimation accuracy, memory usage and performance*

#### Usage

```rust
use quicksilver::pcsa::PCSA;
use std::hash::Hash;

let mut hll = PCSA::new(10);

for i in range(0u, 1000000) {
  let hash = hash::hash(&i);
  pcsa.offer_hashed(hash);
}

let cardinality = pcsa.cardinality();
```

#### Precision vs Memory chart

<table><tr><td style="width:100px; font-weight: bold">Precision</td><td style="width:150px; font-weight: bold">Size</td><td style="width:300px; font-weight: bold">Error on 1m Cardinality Test</td><td style="width:300px; font-weight: bold">Worst Case Estimate Error (+/-)</td>
<tr><td>4</td><td>80 bytes</td> <td>0.09%</td> <td>39.00%</td></tr>
<tr><td>5</td><td>144 bytes</td> <td>4.14%</td> <td>34.88%</td></tr>
<tr><td>6</td><td>272 bytes</td> <td>1.18%</td> <td>31.84%</td></tr>
<tr><td>7</td><td>528 bytes</td> <td>3.62%</td> <td>29.48%</td></tr>
<tr><td>8</td><td>1040 bytes</td> <td>0.71%</td> <td>27.57%</td></tr>
<tr><td>9</td><td>2064 bytes</td> <td>1.51%</td> <td>26.00%</td></tr>
<tr><td>10</td><td>4112 bytes</td> <td>0.77%</td> <td>24.66%</td></tr>
<tr><td>11</td><td>8208 bytes</td> <td>1.15%</td> <td>23.51%</td></tr>
<tr><td>12</td><td>16400 bytes</td> <td>1.10%</td> <td>22.51%</td></tr>
<tr><td>13</td><td>32784 bytes</td> <td>0.96%</td> <td>21.63%</td></tr>
<tr><td>14</td><td>65552 bytes</td> <td>0.20%</td> <td>20.84%</td></tr>
<tr><td>15</td><td>131088 bytes</td> <td>0.18%</td> <td>20.13%</td></tr>
</table>
