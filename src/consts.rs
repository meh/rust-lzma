/// Minimum size for the dictionary.
pub const MINIMUM_DICTIONARY_SIZE: u32 = 1 << 12;

/// Minimum value for the range.
pub const TOP_VALUE: u32 = 1 << 24;

/// The number of bits for probabilities.
pub const MODEL_TOTAL_BITS: u16 = 11;

/// Number of bits to move.
pub const MOVE_BITS: u16 = 5;

/// The initial probability value for 0.5 probability.
pub const PROBABILITY_INITIAL_VALUE: u16 = (1 << MODEL_TOTAL_BITS) / 2;

///
pub const POSITION_BITS_MAX: usize = 4;

///
pub const STATES: usize = 12;

///
pub const END_POSITION_MODEL_INDEX: usize = 14;

///
pub const FULL_DISTANCES: usize = 1 << (END_POSITION_MODEL_INDEX >> 1);

///
pub const ALIGN_BITS: usize = 4;

///
pub const LENGTH_TO_POSITION_STATES: usize = 4;

///
pub const MATCH_MINIMUM_LENGTH: usize = 2;
