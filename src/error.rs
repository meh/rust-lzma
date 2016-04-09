use std::fmt;
use std::error;
use std::io;

/// Errors for `Reader` and `Writer`.
#[derive(Debug)]
pub enum Error {
	/// A downstream IO error.
	IO(io::Error),

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
