use std::ops::{Deref, DerefMut};
use std::io::Read;

use Error;
use consts::PROBABILITY_INITIAL_VALUE;
use super::Range;

#[derive(Clone, Debug)]
pub struct Probabilities {
	buffer: Vec<u16>,
	size:   usize,
}

impl Probabilities {
	pub fn new(size: usize) -> Self {
		Probabilities {
			buffer: vec![PROBABILITY_INITIAL_VALUE; size],
			size:   size,
		}
	}
}

impl Deref for Probabilities {
	type Target = Vec<u16>;

	fn deref(&self) -> &Self::Target {
		&self.buffer
	}
}

impl DerefMut for Probabilities {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.buffer
	}
}

pub fn reverse<T: Read>(mut stream: T, probs: &mut [u16], bits: usize, range: &mut Range) -> Result<usize, Error> {
	let mut m        = 1;
	let mut distance = 0;

	for i in 0 .. bits {
		let bit = try!(range.probabilistic(stream.by_ref(), &mut probs[m]));

		m <<= 1;

		if bit {
			m        += 1;
			distance |= 1 << i;
		}
	}

	Ok(distance)
}
