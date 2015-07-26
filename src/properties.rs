use std::io::Read;
use std::u64;
use byteorder::{LittleEndian, ReadBytesExt};

use {Error};
use consts::MINIMUM_DICTIONARY_SIZE;

/// LZMA model properties.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Properties {
	/// Literal context bits.
	pub lc: u8,

	/// Literal position bits.
	pub lp: u8,

	/// Position bits.
	pub pb: u8,

	/// Dictionary size.
	pub dictionary: u32,

	/// Uncompressed size if present.
	pub uncompressed: Option<u64>,
}

/// Read the model properties from a stream.
pub fn read<T: Read>(mut stream: T) -> Result<Properties, Error> {
	let d = try!(stream.read_u8());

	if d >= (9 * 5 * 5) {
		return Err(Error::InvalidProperties);
	}

	let lc = d % 9;
	let d  = d / 9;
	let pb = d / 5;
	let lp = d % 5;

	let dictionary = match try!(stream.read_u32::<LittleEndian>()) {
		n if n < MINIMUM_DICTIONARY_SIZE =>
			MINIMUM_DICTIONARY_SIZE,

		n =>
			n
	};

	let uncompressed = match try!(stream.read_u64::<LittleEndian>()) {
		u64::MAX =>
			None,

		n =>
			Some(n)
	};

	Ok(Properties {
		lc: lc,
		lp: lp,
		pb: pb,

		dictionary:   dictionary,
		uncompressed: uncompressed,
	})
}
