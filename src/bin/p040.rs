extern crate euler;

use euler::Digits;

// struct ChainedDigits

fn main() {
    let pows_ten = [1, 10, 100, 1000, 10000, 100000, 1000000];

    let product: u32 = (1..)
        .flat_map(|x| Digits::new(x).rev())
        .take(1000000)
        .enumerate()
        .filter(|&(i, _)| pows_ten.contains(&(i + 1)))
        .map(|(_, d)| d)
        .inspect(|&d| println!("{:?}", d))
        .product();

    println!("{:?}", product);
}
