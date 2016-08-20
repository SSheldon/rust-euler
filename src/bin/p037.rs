extern crate num;
extern crate euler;

use num::pow;
use euler::primes;

struct RightTruncation {
    n: u32,
}

impl RightTruncation {
    fn new(n: u32) -> RightTruncation {
        RightTruncation { n: n }
    }
}

impl Iterator for RightTruncation {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.n > 0 {
            let result = Some(self.n);
            self.n /= 10;
            result
        } else {
            None
        }
    }
}

struct LeftTruncation {
    n: u32,
    pow: i32,
}

impl LeftTruncation {
    fn new(n: u32) -> LeftTruncation {
        let pow = (n as f64).log10() as i32;
        LeftTruncation { n: n, pow: pow }
    }
}

impl Iterator for LeftTruncation {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.pow >= 0 {
            let result = Some(self.n);
            let pow_ten = pow(10, self.pow as usize);
            self.pow -= 1;
            self.n %= pow_ten;
            result
        } else {
            None
        }
    }
}

fn main() {
    // Upper bound chosen empirically
    let primes = primes(1000000);

    let truncatable_prime = |n: u32| -> bool {
        RightTruncation::new(n).all(|i| primes.contains(i as usize)) &&
            LeftTruncation::new(n).all(|i| primes.contains(i as usize))
    };

    let sum: u32 = primes.iter()
        .map(|n| n as u32)
        .skip_while(|&n| n < 10) // Single digits don't count
        .filter(|&n| truncatable_prime(n))
        .sum();
    println!("{:?}", sum);
}
