use std::collections::HashMap;

/// A text manager.
pub struct TextManager {

	strings: HashMap<String, &'static str>,

}

impl TextManager {

	/// Creates a new text manager.
	pub fn new() -> Self {

		Self {

			strings: HashMap::new()

		}

	}

	/// Loads a string into the manager.
	pub fn load_string(&mut self, name: &str, content: &'static str) {

		self.strings.insert(String::from(name), content);

	}

	/// Returns a string.
	pub fn get_string(&self, name: &str) -> Option<&str> {

		self.strings.get(name).copied()

	}

}