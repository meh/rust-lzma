use std::io::{self, Write, Cursor};

#[derive(Debug)]
pub struct Cache<'a> {
	cursor: Cursor<&'a mut [u8]>,
	buffer: Option<Vec<u8>>,
}

impl<'a> Cache<'a> {
	pub fn new(cursor: Cursor<&mut [u8]>) -> Cache {
		Cache {
			cursor: cursor,
			buffer: None,
		}
	}

	pub fn into_inner(self) -> Option<Vec<u8>> {
		self.buffer
	}
}

impl<'a> Write for Cache<'a> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let mut written = try!(self.cursor.write(buf));

		if written != buf.len() {
			let mut buffer = Vec::with_capacity(buf.len() - written);
			written += try!(buffer.write(&buf[written..]));
			self.buffer = Some(buffer);
		}

		Ok(written)
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}
