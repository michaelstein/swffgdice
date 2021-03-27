#![allow(non_snake_case)]

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut result = swffgdice::roll(&args);
	result.balance();
	println!("{}", result.to_string());
}
