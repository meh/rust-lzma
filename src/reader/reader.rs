use std::io::{self, Write, Read, BufReader, Cursor};

use {Error, Properties};
use consts::{LENGTH_TO_POSITION_STATES, ALIGN_BITS, END_POSITION_MODEL_INDEX};
use consts::{FULL_DISTANCES, STATES, POSITION_BITS_MAX, MATCH_MINIMUM_LENGTH};
use super::{Range, Window, Length, Probabilities, BitTree, State, Cache};

#[derive(Debug)]
pub struct Reader<T: Read> {
	stream:  BufReader<T>,
	decoded: u64,

	properties: Properties,

	// using an optional buffer and offset for leftovers so we avoid useless
	// allocations and reallocations
	buffer: Option<Vec<u8>>,
	offset: usize,

	range:  Range,
	window: Window,

	literal:  Probabilities,
	position: Probabilities,

	length: Length,
	repeat: Length,

	slot:  Vec<BitTree>,
	align: BitTree,

	state: u32,
	rep:   [u32; 4],

	is_match:     Probabilities,
	is_rep:       Probabilities,
	is_rep_g0:    Probabilities,
	is_rep_g1:    Probabilities,
	is_rep_g2:    Probabilities,
	is_rep0_long: Probabilities,
}

impl<T: Read> Reader<T> {
	pub fn new(properties: Properties, stream: T) -> Result<Reader<T>, Error> {
		let mut stream = BufReader::new(stream);

		let range  = try!(Range::from(stream.by_ref()));
		let window = Window::new(properties.dictionary);

		let literal = Probabilities::new(0x300 << (properties.lc + properties.lp));

		Ok(Reader {
			stream:  stream,
			decoded: 0,

			properties: properties,

			buffer: None,
			offset: 0,

			range:  range,
			window: window,

			literal:  literal,
			position: Probabilities::new(1 + FULL_DISTANCES - END_POSITION_MODEL_INDEX),

			length: Length::new(),
			repeat: Length::new(),

			slot:  vec![BitTree::new(6); LENGTH_TO_POSITION_STATES],
			align: BitTree::new(ALIGN_BITS),

			state: 0,
			rep:   [0; 4],

			is_match:     Probabilities::new(STATES << POSITION_BITS_MAX),
			is_rep:       Probabilities::new(STATES),
			is_rep_g0:    Probabilities::new(STATES),
			is_rep_g1:    Probabilities::new(STATES),
			is_rep_g2:    Probabilities::new(STATES),
			is_rep0_long: Probabilities::new(STATES << POSITION_BITS_MAX),
		})
	}

	pub fn from(mut stream: T) -> Result<Reader<T>, Error> {
		Reader::new(try!(Properties::from(stream.by_ref())), stream)
	}

	fn distance(&mut self, length: usize) -> Result<usize, Error> {
		let state = if length > LENGTH_TO_POSITION_STATES - 1 {
			LENGTH_TO_POSITION_STATES - 1
		}
		else {
			length
		};

		let slot = try!(self.slot[state].decode(self.stream.by_ref(), &mut self.range));

		if slot < 4 {
			return Ok(slot);
		}

		let     direct   = (slot >> 1) - 1;
		let mut distance = (2 | (slot & 1)) << direct;

		if slot < END_POSITION_MODEL_INDEX {
			distance += try!(super::probabilities::reverse(self.stream.by_ref(),
				&mut self.position[distance - slot ..], direct, &mut self.range));
		}
		else {
			distance += try!(self.range.direct(self.stream.by_ref(), direct - ALIGN_BITS)) << ALIGN_BITS;
			distance += try!(self.align.reverse(self.stream.by_ref(), &mut self.range));
		}

		Ok(distance as usize)
	}

	fn literal(&mut self, writer: &mut Cache, state: usize, rep0: u32) -> Result<(), Error> {
		let prev = if !self.window.is_empty() {
			self.window[1] as u32
		}
		else {
			0
		};

		// it will contain the final byte with an additional 9th control bit
		let mut byte = 1u32;

		let lit = ((self.window.total() & ((1 << self.properties.lp) - 1)) << self.properties.lc)
			+ (prev >> (8 - self.properties.lc as u32));

		let probs = &mut self.literal[0x300 * lit as usize ..];

		// we have to use the distance
		if state >= 7 {
			let mut match_byte = self.window[rep0 + 1];

			while byte < 0b1_0000_0000 {
				let match_bit = (match_byte >> 7) & 1;
				match_byte <<= 1;

				let bit = try!(self.range.probabilistic(self.stream.by_ref(),
					&mut probs[(((1 + match_bit as u32) << 8) + byte) as usize]));

				byte <<= 1;
				byte  |= if bit { 1 } else { 0 };

				if match_bit != if bit { 1 } else { 0 } {
					break;
				}
			}
		}

		while byte < 0b1_0000_0000 {
			let bit = try!(self.range.probabilistic(self.stream.by_ref(), &mut probs[byte as usize]));

			byte <<= 1;
			byte  |= if bit { 1 } else { 0 };
		}

		self.window.push(writer, byte as u8)
	}

	fn decode(&mut self, writer: &mut Cache) -> Result<usize, Error> {
		if let Some(size) = self.properties.uncompressed {
			if self.decoded == size {
				return Ok(0);
			}
		}
		else {
			if self.range.is_finished() {
				return Err(Error::MissingMarker);
			}
		}

		let pos = self.window.total() & ((1 << self.properties.pb) - 1);

		if !try!(self.range.probabilistic(self.stream.by_ref(), &mut self.is_match[((pos << POSITION_BITS_MAX) + self.state) as usize])) {
			// check if there's more data to read
			if let Some(size) = self.properties.uncompressed {
				if self.decoded == size {
					return Err(Error::HasMoreData);
				}
			}

			let rep   = self.rep[0];
			let state = self.state;
			try!(self.literal(writer, state as usize, rep));

			self.state    = State::Literal(self.state).update();
			self.decoded += 1;

			return Ok(1);
		}

		let mut length;

		if try!(self.range.probabilistic(self.stream.by_ref(), &mut self.is_rep[self.state as usize])) {
			// check if there's more data to read
			if let Some(size) = self.properties.uncompressed {
				if self.decoded == size {
					return Err(Error::HasMoreData);
				}
			}

			if self.window.is_empty() {
				return Err(Error::HasMoreData);
			}

			if !try!(self.range.probabilistic(self.stream.by_ref(), &mut self.is_rep_g0[self.state as usize])) {
				if !try!(self.range.probabilistic(self.stream.by_ref(), &mut self.is_rep0_long[((self.state << POSITION_BITS_MAX) + pos) as usize])) {
					let byte = self.window[self.rep[0] + 1];
					try!(self.window.push(writer, byte));

					self.state    = State::ShortRepetition(self.state).update();
					self.decoded += 1;

					return Ok(1);
				}
			}
			else {
				let mut distance;
				
				if !try!(self.range.probabilistic(self.stream.by_ref(), &mut self.is_rep_g1[self.state as usize])) {
					distance = self.rep[1];
				}
				else {
					if !try!(self.range.probabilistic(self.stream.by_ref(), &mut self.is_rep_g2[self.state as usize])) {
						distance = self.rep[2];
					}
					else {
						distance    = self.rep[3];
						self.rep[3] = self.rep[2];
					}

					self.rep[2] = self.rep[1];
				}

				self.rep[1] = self.rep[0];
				self.rep[0] = distance;
			}

			length = try!(self.repeat.decode(self.stream.by_ref(), &mut self.range, pos as usize));

			self.state = State::Repetition(self.state).update();
		}
		else {
			length = try!(self.length.decode(self.stream.by_ref(), &mut self.range, pos as usize));

			self.rep[3] = self.rep[2];
			self.rep[2] = self.rep[1];
			self.rep[1] = self.rep[0];
			self.rep[0] = try!(self.distance(length)) as u32;

			// EOS marker found
			if self.rep[0] == 0xffffffff {
				// if the range finished correctly
				if self.range.is_finished() {
					// return error if EOS when the uncompressed size is defined
					if let Some(size) = self.properties.uncompressed {
						if self.decoded != size {
							return Err(Error::NeedMoreData);
						}
					}

					// return EOF
					return Ok(0);
				}
				else {
					return Err(Error::NeedMoreData);
				}
			}

			if self.rep[0] >= self.properties.dictionary || !self.window.check(self.rep[0]) {
				return Err(Error::Corrupted);
			}

			self.state = State::Match(self.state).update();
		}

		length += MATCH_MINIMUM_LENGTH;

		if let Some(size) = self.properties.uncompressed {
			if self.decoded + length as u64 > size {
				return Err(Error::HasMoreData);
			}
		}

		try!(self.window.copy(writer, self.rep[0] + 1, length));
		self.decoded += length as u64;

		Ok(length)
	}
}

impl<T: Read> Read for Reader<T> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		if buf.len() == 0 {
			return Ok(0);
		}

		let     length = buf.len();
		let mut target = Cursor::new(buf);

		// we have some leftovers from the previous decode, try to flush those
		if let Some(buffer) = self.buffer.take() {
			let written  = try!(target.write(&buffer[self.offset..]));
			self.offset += written;

			if self.offset == buffer.len() {
				self.buffer = None;
			}
			else {
				self.buffer = Some(buffer);
			}

			return Ok(written);
		}

		let mut cache = Cache::new(target);

		match self.decode(&mut cache) {
			Err(Error::IO(err)) =>
				Err(err),

			Err(err) =>
				Err(io::Error::new(io::ErrorKind::Other, err)),

			Ok(0) =>
				Ok(0),

			Ok(written) => {
				if let Some(cache) = cache.into_inner() {
					self.buffer = Some(cache);
					self.offset = 0;

					Ok(length)
				}
				else {
					Ok(written)
				}
			}
		}
	}
}
