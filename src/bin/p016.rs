extern crate euler;
extern crate num;

use num::{pow, ToPrimitive};
use num::bigint::BigUint;
use euler::Digits;

fn main() {
	let pow = pow(BigUint::from(2u32), 1000);
	let digit_sum: u32 = Digits::new(pow).map(|d| d.to_u32().unwrap()).sum();
	println!("{}", digit_sum);
}
