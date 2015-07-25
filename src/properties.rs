use std::io::Read;
use std::u64;

use byteorder::{LittleEndian, ReadBytesExt};
use {Error};
use consts::MINIMUM_DICTIONARY_SIZE;

#[derive(Debug)]
pub struct Properties {
	pub lc: u8,
	pub lp: u8,
	pub pb: u8,

	pub dictionary:   u32,
	pub uncompressed: Option<u64>,
}

impl Properties {
	pub fn from<T: Read>(mut stream: T) -> Result<Properties, Error> {
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
}
