LZMA
====
[![Build Status](https://travis-ci.org/meh/rust-lzma.svg?branch=master)](https://travis-ci.org/meh/rust-lzma)

LZMA handling library.

```toml
[dependencies]
lzma = "*"
```

Example
-------
This example will decode and print an LZMA compressed UTF-8 text file.

```rust
use std::io::Read;
use std::env;

extern crate lzma;

fn main() {
	let mut decoder = lzma::open(&env::args().nth(1).expect("missing file")).unwrap();
	let mut string  = String::new();

	decoder.read_to_string(&mut string).unwrap();

	print!("{}", string);
}
```
