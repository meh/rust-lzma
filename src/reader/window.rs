use std::ops::{Deref, Index};
use std::io::Write;
use byteorder::WriteBytesExt;

use {Error};

/// A sliding window implementation.
#[derive(Debug)]
pub struct Window {
	buffer: Vec<u8>,
	size:   u32,

	position: u32,
	total:    u32,

	full: bool,
}

impl Window {
	/// Creates a sliding window with the given size.
	pub fn new(size: u32) -> Self {
		Window {
			buffer: Vec::with_capacity(size as usize),
			size:   size,

			position: 0,
			total:    0,

			full: false,
		}
	}

	/// Gets the size.
	pub fn size(&self) -> u32 {
		self.size
	}

	/// Gets the current position.
	pub fn position(&self) -> u32 {
		self.position
	}

	/// Gets the total position.
	pub fn total(&self) -> u32 {
		self.total
	}

	/// Checks if the window is full.
	pub fn is_full(&self) -> bool {
		self.full
	}

	/// Checks if the window is empty.
	pub fn is_empty(&self) -> bool {
		self.position == 0 && !self.is_full()
	}

	#[doc(hidden)]
	pub unsafe fn reset(&mut self) {
		self.position = 0;
		self.total    = 0;
		self.full     = false;
	}

	/// Pushes a byte to the window and the given writer.
	pub fn push<W: Write>(&mut self, mut stream: W, byte: u8) -> Result<(), Error> {
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

	/// Pushes `length` bytes from the given distance into the window and the given writer.
	pub fn copy<W: Write>(&mut self, mut stream: W, distance: u32, length: usize) -> Result<(), Error> {
		for _ in 0 .. length {
			let b = self[distance];
			try!(self.push(stream.by_ref(), b));
		}

		Ok(())
	}

	/// Checks if the distance is valid.
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
