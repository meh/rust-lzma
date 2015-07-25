use std::io::Read;
use std::fs::File;

extern crate lzma;

fn check(string: String) {
	let mut reader  = File::open("tests/assets/a.txt").unwrap();
	let mut control = String::new();

	reader.read_to_string(&mut control).unwrap();

	assert_eq!(string, control);
}

fn decode(path: &str) -> String {
	let mut reader = lzma::open(path).unwrap();
	let mut string = String::new();

	reader.read_to_string(&mut string).unwrap();

	string
}

#[test]
fn a() {
	check(decode("tests/assets/a.lzma"));
}

#[test]
fn a_eos() {
	check(decode("tests/assets/a_eos.lzma"));
}

#[test]
fn a_eos_and_size() {
	check(decode("tests/assets/a_eos_and_size.lzma"));
}

#[test]
#[should_panic]
fn bad_corrupted() {
	check(decode("tests/assets/bad_corrupted.lzma"));
}

#[test]
#[should_panic]
fn bad_incorrect_size() {
	check(decode("tests/assets/bad_incorrect_size.lzma"));
}

#[test]
#[should_panic]
fn bad_eos_incorrect_size() {
	check(decode("tests/assets/bad_eos_incorrect_size.lzma"));
}
