use std::iter::AdditiveIterator;

static COINS: [uint, ..8] = [1, 2, 5, 10, 20, 50, 100, 200];

fn change_combs(change: uint, max_coin: uint) -> uint {
	COINS.iter().filter_map(|&coin| {
		if coin <= max_coin && coin <= change {
			Some(if coin == change { 1 }
			     else { change_combs(change - coin, coin) })
		} else {
			None
		}
	}).sum()
}

fn main() {
	println!("{}", change_combs(200, 200));
}
