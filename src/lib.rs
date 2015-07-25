//! LZMA handling library.

#![warn(missing_docs)]

extern crate byteorder;

mod consts;

mod error;
pub use error::Error;

mod properties;
pub use properties::Properties;

mod reader;
pub use reader::{Reader, open, read};
