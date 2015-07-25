use std::io::{self, Read, Write};
use std::env;
use std::process;

extern crate lzma;

fn main() {
	let mut decoder = lzma::open(&env::args().nth(1).expect("missing file")).unwrap();
	let mut buffer  = [0u8; 4096];
	let mut stdout  = io::stdout();

	loop {
		match decoder.read(&mut buffer) {
			Ok(0) =>
				break,

			Ok(n) =>
				stdout.write_all(&buffer[0..n]).unwrap(),

			Err(_) =>
				process::exit(1),
		}
	}
}
