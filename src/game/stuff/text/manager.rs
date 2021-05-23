use std::collections::HashMap;
use super::TextAsset;

pub struct TextManager {

	texts: HashMap<String, TextAsset>,

}

impl TextManager {

	pub fn new() -> Self {

		Self {

			texts: HashMap::new(),

		}

	}
	
	pub fn get(&self, name: &str) -> Option<&str> {

		self.texts
			.get(name)
			.map(|t| t.text)

	}

	pub fn load(&mut self, asset: TextAsset) {

		self.texts.insert(String::from(asset.name), asset);

	}

}