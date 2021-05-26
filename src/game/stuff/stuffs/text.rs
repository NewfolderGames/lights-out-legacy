use std::collections::hash_map::HashMap;

/// A text asset.
pub struct TextAsset {

	pub name: &'static str,
	pub text: &'static str

}

impl TextAsset {

	/// Creates a new text asset.
	pub fn new(name: &'static str, text: &'static str) -> Self {

		Self {

			name,
			text

		}

	}

}

/// A text storage.
pub struct TextStorage {

	texts: HashMap<String, TextAsset>

}

impl TextStorage {

	/// Creates a new text storage.
	pub fn new() -> Self {

		Self {

			texts: HashMap::new(),

		}

	}
	
	/// Returns a text.
	pub fn get(&self, name: &str) -> Option<&str> {

		self.texts
			.get(name)
			.map(|t| t.text)

	}

	/// Loads text into the storage.
	pub fn load(&mut self, asset: TextAsset) {

		self.texts.insert(String::from(asset.name), asset);

	}

}