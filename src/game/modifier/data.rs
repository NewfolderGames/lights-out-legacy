/// A loadable modifier data.
pub struct ModifierData {

	pub name: &'static str,
	pub title: &'static str,

	pub default_value: f64,

}

// Constructor.

impl ModifierData {

	/// Creates a new modifier data.
	pub fn new(name: &'static str, title: &'static str, default_value: f64) -> Self {

		Self {

			name,
			title,
			default_value

		}

	}

}