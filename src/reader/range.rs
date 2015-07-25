use std::io::Read;
use byteorder::ReadBytesExt;

use Error;
use consts::{MODEL_TOTAL_BITS, TOP_VALUE, MOVE_BITS};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Range {
	range: u32,
	code:  u32,
}

impl Range {
	pub fn new(range: u32, code: u32) -> Range {
		Range {
			range: range,
			code:  code,
		}
	}

	pub fn from<T: Read>(mut stream: T) -> Result<Range, Error> {
		let     range = 0xffffffff;
		let mut code  = 0;

		let control = try!(stream.read_u8());

		for _ in 0 .. 4 {
			code = (code << 8) | try!(stream.read_u8()) as u32;
		}

		if control != 0 || code == range {
			Err(Error::Corrupted)
		}
		else {
			Ok(Range::new(range, code))
		}
	}

	fn normalize<T: Read>(&mut self, mut stream: T) -> Result<(), Error> {
		if self.range < TOP_VALUE {
			self.range <<= 8;
			self.code    = (self.code << 8) | try!(stream.read_u8()) as u32;
		}

		Ok(())
	}

	pub fn direct<T: Read>(&mut self, mut stream: T, bits: usize) -> Result<usize, Error> {
		let mut result = 0usize;

		for _ in 0 .. bits {
			self.range >>= 1;
			self.code    = self.code.wrapping_sub(self.range);

			let t = 0u32.wrapping_sub(self.code >> 31);
			self.code = self.code.wrapping_add(self.range & t);

			if self.code == self.range {
				return Err(Error::Corrupted);
			}

			try!(self.normalize(stream.by_ref()));

			result <<= 1;
			result  += t.wrapping_add(1) as usize;
		}

		Ok(result)
	}

	pub fn probabilistic<T: Read>(&mut self, stream: T, prob: &mut u16) -> Result<bool, Error> {
		let mut v     = *prob;
		let     bound = (self.range >> MODEL_TOTAL_BITS) * v as u32;

		let bit = if self.code < bound {
			v          += ((1 << MODEL_TOTAL_BITS) - v) >> MOVE_BITS;
			self.range  = bound;

			false
		}
		else {
			v          -= v >> MOVE_BITS;
			self.code  -= bound;
			self.range -= bound;

			true
		};

		try!(self.normalize(stream));

		*prob = v;

		Ok(bit)
	}

	pub fn is_finished(&self) -> bool {
		self.code == 0
	}
}
