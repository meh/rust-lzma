use std::io::Read;

use {Error};
use super::{Range, Probabilities};

/// A bit tree decoder.
#[derive(Clone, Debug)]
pub struct BitTree {
	probabilities: Probabilities,
	bits:          usize,
}

impl BitTree {
	/// Creates a new bit tree of the given size.
	pub fn new(bits: usize) -> BitTree {
		BitTree {
			probabilities: Probabilities::new(1 << bits),
			bits:          bits,
		}
	}

	/// Resets the bit tree decoder.
	///
	/// Note thet resetting might corrupt the decoding.
	pub unsafe fn reset(&mut self) {
		self.probabilities.reset();
	}

	/// Gets the number of bits in the tree.
	pub fn bits(&self) -> usize {
		self.bits
	}

	/// Decodes bits.
	pub fn decode<T: Read>(&mut self, mut stream: T, range: &mut Range) -> Result<usize, Error> {
		let mut m = 1usize;

		for _ in 0 .. self.bits() {
			if try!(range.probabilistic(stream.by_ref(), &mut self.probabilities[m])) {
				m <<= 1;
				m  += 1;
			}
			else {
				m <<= 1;
			}
		}

		Ok(m - (1 << self.bits()))
	}

	/// Decodes bits in reverse order.
	pub fn reverse<T: Read>(&mut self, stream: T, range: &mut Range) -> Result<usize, Error> {
		super::probabilities::reverse(stream, &mut self.probabilities, self.bits, range)
	}
}
