use std::io::Read;
use std::env;

extern crate lzma;

fn main() {
	let mut decoder = lzma::open(&env::args().nth(1).expect("missing file")).unwrap();
	let mut string  = String::new();

	decoder.read_to_string(&mut string).unwrap();

	print!("{}", string);
}
