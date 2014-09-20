
use std::int;
use std::num;

static phi: f64 = 0.77351f64;
static kappa: f64 = 1.75f64;



///
/// Implements the Probabilistic Counting with Stochastic Averaging
/// counter.  PCSA provides an approximate estimate of cardinality with
/// bounded error.
///
/// Relative error is 0.78 / sqrt(m)
///
/// See this [article](http://blog.aggregateknowledge.com/2013/04/02/sketch-of-the-day-probabilistic-counting-with-stochastic-averaging-pcsa/) for a layman's explanation of PCSA
///
/// Original paper can be found [here](http://www.mathcs.emory.edu/~cheung/papers/StreamDB/Probab/1985-Flajolet-Probabilistic-counting.pdf)
///
pub struct PCSA {
    m: u32,
    b: u32,
    buckets: Vec<u32>,
    indexMask: u64
}

impl PCSA {

    /// Construct a new PCSA counter
    ///
    /// b: number of bits to use as bucket index mask, larger
    ///    values means more buckets, better accuracy, more space
    ///    b must be between 4..16 inclusive
    ///
    ///    m = 2^b
    ///
    /// Complete PCSA Memory chart for reference (including per-object overhead, minus static overheads):
    ///
    /// <table><tr><td style="width:100px; font-weight: bold">Precision</td><td style="width:150px; font-weight: bold">Size</td><td style="width:300px; font-weight: bold">Error on 1m Cardinality Test</td><td style="width:300px; font-weight: bold">Worst Case Estimate Error (+/-)</td>
    /// <tr><td>4</td><td>80 bytes</td> <td>0.09%</td> <td>39.00%</td></tr>
    /// <tr><td>5</td><td>144 bytes</td> <td>4.14%</td> <td>34.88%</td></tr>
    /// <tr><td>6</td><td>272 bytes</td> <td>1.18%</td> <td>31.84%</td></tr>
    /// <tr><td>7</td><td>528 bytes</td> <td>3.62%</td> <td>29.48%</td></tr>
    /// <tr><td>8</td><td>1040 bytes</td> <td>0.71%</td> <td>27.57%</td></tr>
    /// <tr><td>9</td><td>2064 bytes</td> <td>1.51%</td> <td>26.00%</td></tr>
    /// <tr><td>10</td><td>4112 bytes</td> <td>0.77%</td> <td>24.66%</td></tr>
    /// <tr><td>11</td><td>8208 bytes</td> <td>1.15%</td> <td>23.51%</td></tr>
    /// <tr><td>12</td><td>16400 bytes</td> <td>1.10%</td> <td>22.51%</td></tr>
    /// <tr><td>13</td><td>32784 bytes</td> <td>0.96%</td> <td>21.63%</td></tr>
    /// <tr><td>14</td><td>65552 bytes</td> <td>0.20%</td> <td>20.84%</td></tr>
    /// <tr><td>15</td><td>131088 bytes</td> <td>0.18%</td> <td>20.13%</td></tr>
    /// </table>
    /// ### Usage
    ///
    ///```
    ///let mut pcsa = PCSA::new(10);
    ///```
    #[experimental]
    pub fn new(b: u32) -> PCSA {

        let m: u32;

        match b {
            4..16 => {  m = num::pow(2, b as uint) },
            _ => fail!("b must be 4 <= b <= 16")
        }

        let mut buckets: Vec<u32> = Vec::from_elem(m as uint, 0u32);
        let indexMask: u64 = ((1u << b as uint) - 1) as u64;

        PCSA {m: m, b: b, buckets: buckets, indexMask: indexMask}
    }

    /// Offer a hashed u64 value to the PCSA algorithm.  If this is a new, distinct value, the PCSA algo will update it's
    /// internal buckets.
    ///
    ///### Usage
    ///
    ///```
    /// let mut pcsa = PCSA::new(10);
    ///
    /// // Hash the value with std::hash SipHash 2-4. Any 64-bit hash will work
    /// // You should probably use something *other* than SipHash, since it is cryptographic
    /// // and slow
    /// let hash: u64 = hash::hash(&19u);
    /// pcsa.offer_hashed(hash);
    ///
    ///```
    #[experimental]
    pub fn offer_hashed(&mut self, hash: &u64) {
        let index: u32 = (hash & self.indexMask) as u32;
        *self.buckets.get_mut(index as uint) |= 1 << (hash >> self.b as uint).trailing_zeros() as uint;
    }


    /// Returns the current cardinality estimate
    #[experimental]
    pub fn cardinality(&self) -> u32 {
        let mut counter = 0u;
        let mut sum = 0u;


        while counter < self.m as uint {
            // We want first "significant zero", so invert first
            sum += (!(self.buckets[counter])).trailing_zeros() as uint;
            counter += 1;
        }

        // pcsaCardinality: m / phi * Math.pow(2, sum / m)
        let m: f64 = self.m as f64;
        (m / phi * 2f64.powf(sum as f64 / m)) as u32
    }


    /// Returns the current cardinality estimatewith small-cardinality correction enabled
    #[experimental]
    pub fn smCardinality(&self) -> u32 {
        let mut counter = 0u;
        let mut sum = 0u;


        while counter < self.m as uint {
            // We want first "significant zero", so invert first
            sum += (!(self.buckets[counter])).trailing_zeros() as uint;
            counter += 1;
        }

        // pcsaCorCardinality: m / phi * (Math.pow(2, sum / m) - Math.pow(2, -kappa * sum / m)),
        let m: f64 = self.m as f64;
        ((m / phi * 2f64.powf(sum as f64 / m)) - 2f64.powf(-kappa * sum as f64 / m)) as u32
    }


    /// Returns the amount of memory (in bytes) used by this data structure
    #[experimental]
    pub fn ram_bytes_used(&self) -> u32 {
        16 + (self.m * 4)    // m + b + buckets[u32, ..m]
    }


    /// Merge another counter into this counter
    #[experimental]
    pub fn merge(&mut self, p2: &PCSA) {
        let mut counter = 0u;

        while counter < self.m as uint {
            *self.buckets.get_mut(counter) |= p2.buckets[counter];
            counter += 1;
        }

    }
}

#[cfg(test)]
mod test {

    use super::PCSA;
    use std::hash;
    use std::hash::Hash;
    use std::task;

    use time::precise_time_ns;

    #[test]
    pub fn test_pcsa_merge() {

        let mut count: int = 0;

        let mut p1 = PCSA::new(5);
        let mut p2 = PCSA::new(5);
        while count < 1000 {
            let hash = hash::hash(&count.to_string());
            p1.offer_hashed(&hash);

            let hash = hash::hash(&(count + 1000).to_string());
            p2.offer_hashed(&hash);
            count += 1;
        }

        error!("   P1: {}", p1.cardinality());
        error!("   P2: {}", p2.cardinality());

        p1.merge(&p2);
        error!("   P1 + P2: {}",  p1.cardinality());

        let estimate = p1.cardinality() as int;
        let error = (estimate - 2000i).abs() as f64 / 2000f64;
        let theoreticalError = 0.78 / 5f64.sqrt();

        error!("   Error: {}", error);
        error!("   Expected Error: {}", theoreticalError);

        assert!(error < theoreticalError);
    }

    #[test]
    pub fn test_pcsa_bad_constructor() {

        let mut m = 0;

        while m < 20u32 {
            let tm = m;
            let result = task::try(proc() {
                let mut pcsa = PCSA::new(tm);
            });

            match m {
                4..16 => {if result.is_err() {fail!("4..16 range threw error")}},
                _     => {if !result.is_err() {fail!("_ range did not throw error")}}
            }

            m += 1u32;
        }
    }

    #[test]
    pub fn test_pcsa_100000() {

        let mut m: f64 = 4f64;
        let iterations = 100000;

        while m < 16f64 {
            let mut pcsa = PCSA::new(m as u32);

            let mut count: int = 0;
            let start = precise_time_ns();

            while count < iterations {
                let hash = hash::hash(&count.to_string());  //TODO move this outside the loop, horrible for benchmarking

                pcsa.offer_hashed(&hash);
                count += 1;
            }
            let time = (precise_time_ns() - start) / 1000000;
            let qps = iterations as f64 / time as f64;

            let estimate = pcsa.cardinality() as int;
            let error = (estimate - count).abs() as f64 / count as f64;
            let theoreticalError: f64 = 0.78f64 / m.sqrt();

            error!("   m: {}", m);
            error!("   Milliseconds: {}", time);
            error!("   Counts/ms: {}", qps);
            error!("   Estimate: {}", estimate);
            error!("   Error: {}", error);
            error!("   Expected Error: {}", theoreticalError);
            error!("   Ram Usage: {}", pcsa.ram_bytes_used());

            assert!(error < theoreticalError);

            m += 1f64;
        }
    }

    #[test]
    pub fn test_pcsa_1000000() {

        let mut m: f64 = 4f64;
        let iterations = 1000000;

        while m < 16f64 {
            let mut pcsa = PCSA::new(m as u32);

            let mut count: int = 0;
            let start = precise_time_ns();

            while count < iterations {
                let hash = hash::hash(&count.to_string());   //TODO move this outside the loop, horrible for benchmarking

                pcsa.offer_hashed(&hash);
                count += 1;
            }
            let time = (precise_time_ns() - start) / 1000000;
            let qps = iterations as f64 / time as f64;

            let estimate = pcsa.cardinality() as int;
            let error = (estimate - count).abs() as f64 / count as f64;
            let theoreticalError = 0.78f64 / m.sqrt();

            error!("   m: {}",  m);
            error!("   Milliseconds: {}", time);
            error!("   Counts/ms: {}", qps);
            error!("   Estimate: {}", estimate);
            error!("   Error: {}", error);
            error!("   Expected Error: {}", theoreticalError);
            error!("   Ram Usage: {}", pcsa.ram_bytes_used());
            assert!(error < theoreticalError);

            m += 1f64;
        }
    }

}