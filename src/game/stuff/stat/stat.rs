/// A stat data.
pub struct Stat {

	category: String,
	value: f64,

}

impl Stat {

	/// Creates a new stat.
	pub fn new(category: &str) -> Self {

		Self {

			category: String::from(category),
			value: 0f64

		}

	}

	/// Adds amount to the stat's value.
	pub fn add_value(&mut self, amount: f64) {

		self.value += amount;

	}

	/// Returns a reference to the stat's category.
	pub fn get_category(&self) -> &String {

		&self.category

	}

	/// Returns the stat's value.
	pub fn get_value(&self) -> f64 {

		self.value

	}

	/// Resets the stat's value.
	pub fn reset(&mut self) {

		self.value = 0f64;

	}

	/// Sets the stat's value.
	pub fn set_value(&mut self, value: f64) {

		self.value = value;

	}

}