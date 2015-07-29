use std::io::{self, Write, Cursor};

/// A wrapper around a `Cursor`.
///
/// It will cache the contents that go over the size of the cursor so they can
/// be extracted later and saved.
#[derive(Debug)]
pub struct Cache<'a> {
	cursor: Cursor<&'a mut [u8]>,
	buffer: Option<Vec<u8>>,
}

impl<'a> Cache<'a> {
	/// Creates a new cache.
	pub fn new(cursor: Cursor<&mut [u8]>) -> Cache {
		Cache {
			cursor: cursor,
			buffer: None,
		}
	}

	/// Returns the underlying buffer.
	pub fn into_inner(self) -> Option<Vec<u8>> {
		self.buffer
	}
}

impl<'a> Write for Cache<'a> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let mut written = try!(self.cursor.write(buf));

		if written != buf.len() {
			if self.buffer.is_none() {
				self.buffer = Some(Vec::with_capacity(buf.len() - written));
			}

			written += try!(self.buffer.as_mut().unwrap().write(&buf[written..]));
		}

		Ok(written)
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}
