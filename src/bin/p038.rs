extern crate bit_set;
extern crate num;
extern crate euler;

use std::iter::repeat;
use std::ops::Range;
use bit_set::BitSet;
use num::pow;
use euler::Digits;

fn pandigital_concat_product(n: u32, muls: Range<u32>) -> Option<u32> {
    let digits = muls.rev()
        .map(|mul| n * mul)
        .flat_map(|prod| Digits::new(prod));

    let mut product = 0;
    let mut pow_ten = 1;
    let mut seen_digits = BitSet::with_capacity(10);
    for digit in digits {
        let newly_added = seen_digits.insert(digit as usize);
        // If we see a repeated digit, it's not pandigital
        if !newly_added {
            return None;
        }
        product += digit * pow_ten;
        pow_ten *= 10;
    }

    // If all digits 1..9 are set, it's pandigital
    if seen_digits.len() == 9 && !seen_digits.contains(0) {
        Some(product)
    } else {
        None
    }
}

fn main() {
    // Narrowing the search space: it'd be impossible to concat n numbers and
    // end up with 9 digits if every number has > 9/n digits
    let max = (2..10)
        .flat_map(|n| repeat(n).zip(1..pow(10, (9 / n) as usize)))
        .filter_map(|(n, i)| pandigital_concat_product(i, 1..n+1))
        .max().unwrap();

    println!("{:?}", max);
}
