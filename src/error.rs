use std::fmt;
use std::error;
use std::io;
use byteorder;

/// Errors for `Reader` and `Writer`.
#[derive(Debug)]
pub enum Error {
	/// A downstream IO error.
	IO(io::Error),

	/// A downstream byteorder error.
	ByteOrder(byteorder::Error),

	/// The stream is corrupted.
	Corrupted,

	/// Invalid model values.
	InvalidProperties,

	/// The EOS marker is missing.
	MissingMarker,

	/// The stream has more data but the uncompressed size has been reached.
	HasMoreData,

	/// The stream has finished but the uncompressed size has not been reached.
	NeedMoreData,

	/// The stream has finished unexpectedly with a marker.
	FinishedWithMarker,
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::IO(value)
	}
}

impl From<byteorder::Error> for Error {
	fn from(value: byteorder::Error) -> Self {
		Error::ByteOrder(value)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match self {
			&Error::IO(ref err) =>
				err.description(),

			&Error::ByteOrder(ref err) =>
				err.description(),

			&Error::Corrupted =>
				"The LZMA stream is corrupted.",

			&Error::InvalidProperties =>
				"Invalid model values.",

			&Error::MissingMarker =>
				"The EOS marker is missing.",

			&Error::HasMoreData =>
				"The stream has more data but the uncompressed size has been reached.",

			&Error::NeedMoreData =>
				"The stream has finished but the uncompressed size has not been reached.",

			&Error::FinishedWithMarker =>
				"The stream has finished unexpectedly with a marker.",
		}
	}
}
