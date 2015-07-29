use std::io::Read;

use {Error};
use consts::{PROBABILITY_INITIAL_VALUE, POSITION_BITS_MAX};
use super::{BitTree, Range};

/// A length decoder.
#[derive(Clone, Debug)]
pub struct Length {
	choice: [u16; 2],

	low: Vec<BitTree>,
	mid: Vec<BitTree>,
	hig: BitTree,
}

impl Length {
	/// Creates a new length decoder.
	pub fn new() -> Self {
		Length {
			choice: [PROBABILITY_INITIAL_VALUE; 2],

			low: vec![BitTree::new(3); 1 << POSITION_BITS_MAX],
			mid: vec![BitTree::new(3); 1 << POSITION_BITS_MAX],
			hig: BitTree::new(8),
		}
	}

	/// Resets the decoder.
	///
	/// Note that resetting might corrupt the decoding.
	pub unsafe fn reset(&mut self) {
		self.choice = [PROBABILITY_INITIAL_VALUE; 2];

		for bt in &mut self.low {
			bt.reset();
		}

		for bt in &mut self.mid {
			bt.reset();
		}

		self.hig.reset();
	}

	/// Decode a length.
	pub fn decode<T: Read>(&mut self, mut stream: T, range: &mut Range, state: usize) -> Result<usize, Error> {
		if !try!(range.probabilistic(stream.by_ref(), &mut self.choice[0])) {
			Ok(try!(self.low[state].decode(stream.by_ref(), range)))
		}
		else if !try!(range.probabilistic(stream.by_ref(), &mut self.choice[1])) {
			Ok(8 + try!(self.mid[state].decode(stream.by_ref(), range)))
		}
		else {
			Ok(16 + try!(self.hig.decode(stream.by_ref(), range)))
		}
	}
}
