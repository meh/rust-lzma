use std::ops::{Deref, Index};
use std::io::Write;
use byteorder::WriteBytesExt;

use {Error};

#[derive(Debug)]
pub struct Window {
	buffer: Vec<u8>,
	size:   u32,

	position: u32,
	total:    u32,

	full: bool,
}

impl Window {
	pub fn new(size: u32) -> Self {
		Window {
			buffer: Vec::with_capacity(size as usize),
			size:   size,

			position: 0,
			total:    0,

			full: false,
		}
	}

	pub fn size(&self) -> u32 {
		self.size
	}

	pub fn position(&self) -> u32 {
		self.position
	}

	pub fn total(&self) -> u32 {
		self.total
	}

	pub fn is_full(&self) -> bool {
		self.full
	}

	pub fn is_empty(&self) -> bool {
		self.position == 0 && !self.is_full()
	}

	pub fn push<T: Write>(&mut self, mut stream: T, byte: u8) -> Result<(), Error> {
		try!(stream.write_u8(byte));

		self.buffer.insert(self.position as usize, byte);

		self.position += 1;
		self.total    += 1;

		if self.position() == self.size() {
			self.position = 0;
			self.full     = true;
		}

		Ok(())
	}

	pub fn copy<T: Write>(&mut self, mut stream: T, distance: u32, length: usize) -> Result<(), Error> {
		for _ in 0 .. length {
			let b = self[distance];
			try!(self.push(stream.by_ref(), b));
		}

		Ok(())
	}

	pub fn check(&self, distance: u32) -> bool {
		distance <= self.position || self.full
	}
}

impl Index<u32> for Window {
	type Output = u8;

	fn index(&self, distance: u32) -> &u8 {
		if distance <= self.position {
			&self.buffer[(self.position - distance) as usize]
		}
		else {
			&self.buffer[(self.size - distance + self.position) as usize]
		}
	}
}

impl Deref for Window {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		&*self.buffer
	}
}
