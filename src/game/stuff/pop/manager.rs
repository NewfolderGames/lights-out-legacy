use super::Pop;

/// A pop manager.
pub struct PopManager {

	pops: Vec<Pop>,

}

impl PopManager {

	/// Creates a new pop manager.
	pub fn new() -> Self {

		Self {

			pops: Vec::new()

		}

	}

}
