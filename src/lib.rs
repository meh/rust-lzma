//! LZMA handling library.

#![warn(missing_docs)]

extern crate byteorder;

mod consts;

mod error;
pub use error::Error;

/// Model property related functions.
pub mod properties;
pub use properties::Properties;

#[doc(hidden)]
pub mod reader;
pub use reader::{Reader, open, read};
