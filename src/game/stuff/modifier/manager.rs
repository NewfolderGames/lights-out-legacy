use std::collections::hash_map::{ HashMap, Iter };

/// A modifier manager.
pub struct ModifierManager {

	modifiers: HashMap<String, f64>

}

impl ModifierManager {

	/// Creates a new modifier manager.
	pub fn new() -> Self {

		Self {

			modifiers: HashMap::new()

		}

	}

	/// Sets a modifier's value.
	pub fn add_value(&mut self, name: &str, value: f64) {

		self.modifiers
			.get_mut(name)
			.map(|m| *m += value);

	}

	/// Returns a modifier's value.
	pub fn get_value(&self, name: &str) -> Option<f64> {

		self.modifiers.get(name).copied()

	}

	/// Returns a modifier iterator.
	pub fn iter(&self) -> Iter<String, f64> {
		
		self.modifiers.iter()

	}

	/// Loads a modifier.
	pub fn load(&mut self, name: &str) {

		self.modifiers.insert(String::from(name), 0f64);

	}

	/// Resets modifier manager.
	pub fn reset(&mut self) {

		self.modifiers
			.iter_mut()
			.for_each(|(_, value)| *value = 0f64);

	}

	/// Sets a modifier's value.
	pub fn set_value(&mut self, name: &str, value: f64) {

		self.modifiers
			.get_mut(name)
			.map(|m| *m = value);

	}

}