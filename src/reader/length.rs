use std::io::Read;

use {Error};
use consts::{PROBABILITY_INITIAL_VALUE, POSITION_BITS_MAX};
use super::{BitTree, Range};

#[derive(Clone, Debug)]
pub struct Length {
	choice:  u16,
	choice2: u16,

	low: Vec<BitTree>,
	mid: Vec<BitTree>,
	hig: BitTree,
}

impl Length {
	pub fn new() -> Self {
		Length {
			choice:  PROBABILITY_INITIAL_VALUE,
			choice2: PROBABILITY_INITIAL_VALUE,

			low: vec![BitTree::new(3); 1 << POSITION_BITS_MAX],
			mid: vec![BitTree::new(3); 1 << POSITION_BITS_MAX],
			hig: BitTree::new(8),
		}
	}

	pub fn decode<T: Read>(&mut self, mut stream: T, range: &mut Range, state: usize) -> Result<usize, Error> {
		if !try!(range.probabilistic(stream.by_ref(), &mut self.choice)) {
			Ok(try!(self.low[state].decode(stream.by_ref(), range)))
		}
		else if !try!(range.probabilistic(stream.by_ref(), &mut self.choice2)) {
			Ok(8 + try!(self.mid[state].decode(stream.by_ref(), range)))
		}
		else {
			Ok(16 + try!(self.hig.decode(stream.by_ref(), range)))
		}
	}
}
