mod probabilities;
pub use self::probabilities::Probabilities;

mod bit_tree;
pub use self::bit_tree::BitTree;

mod window;
pub use self::window::Window;

mod range;
pub use self::range::Range;

mod length;
pub use self::length::Length;

mod state;
pub use self::state::State;

mod cache;
pub use self::cache::Cache;

mod reader;
pub use self::reader::Reader;

use std::io::Read;
use std::fs::File;
use std::path::Path;

use Error;

pub fn open<T: AsRef<Path>>(path: T) -> Result<Reader<File>, Error> {
	read(try!(File::open(path)))
}

pub fn read<T: Read>(stream: T) -> Result<Reader<T>, Error> {
	Reader::from(stream)
}
