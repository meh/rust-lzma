/// The decoder state.
pub enum State {
	///
	Literal(u32),

	///
	Match(u32),

	///
	Repetition(u32),

	///
	ShortRepetition(u32),
}

impl State {
	/// Update the state.
	pub fn update(self) -> u32 {
		match self {
			State::Literal(value) if value < 4  => 0,
			State::Literal(value) if value < 10 => value - 3,
			State::Literal(value)               => value - 6,

			State::Match(value) if value < 7 => 7,
			State::Match(_)                  => 10,

			State::Repetition(value) if value < 7 => 8,
			State::Repetition(_)                  => 11,

			State::ShortRepetition(value) if value < 7 => 9,
			State::ShortRepetition(_)                  => 11,
		}
	}
}
