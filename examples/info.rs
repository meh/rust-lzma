use std::io::Read;
use std::env;

extern crate lzma;

fn size_for(size: u64) -> String {
	format!("{} MB ({} bytes)", size / 1024 / 1024, size)
}

fn dictionary_for(size: u32) -> String {
	format!("{} MB (2^{} bytes)", size / 1024 / 1024, (size as f32).log2())
}

fn main() {
	let mut args = env::args();
	args.next();

	let mut file = args.next().expect("missing file");
	let     read = if file == "-read" {
		file = args.next().expect("missing file");
		true
	}
	else {
		false
	};

	let mut decoder    = lzma::open(&file).unwrap();
	let     properties = *decoder.properties();

	if let Some(size) = properties.uncompressed {
		println!("Uncompressed size:\t\t{}", size_for(size));
	}
	else {
		if read {
			let mut size = 0u64;
			let mut buf  = [0u8; 4096];

			loop {
				match decoder.read(&mut buf) {
					Ok(0) =>
						break,

					Ok(n) =>
						size += n as u64,

					Err(_) =>
						break,
				}
			}

			println!("Uncompressed size:\t\t{}", size_for(size));
		}
		else {
			println!("Uncompressed size:\t\tUnknown");
		}
	}

	println!("Dictionary size:\t\t{}", dictionary_for(properties.dictionary));
	println!("Literal context bits (lc):\t{}", properties.lc);
	println!("Literal pos bits (lp):\t\t{}", properties.lp);
	println!("Numebr of pos bits (pb):\t{}", properties.pb);
}
