use std::collections::{ HashMap, hash_map::Iter };
use super::{ Modifier, ModifierAsset };

pub struct ModifierManager {

	modifiers: HashMap<String, Modifier>,

}

impl ModifierManager {

	pub fn new() -> Self {

		Self {

			modifiers: HashMap::new()

		}
		
	}

	pub fn get(&self, name: &str) -> Option<&Modifier> {
		
		self.modifiers.get(name)
		
	}
	
	pub fn get_mut(&mut self, name: &str) -> Option<&mut Modifier> {
		
		self.modifiers.get_mut(name)
		
	}
	
	pub fn get_value(&self, name: &str) -> Option<f64> {

		self.modifiers
			.get(name)
			.map(|m| m.get_value())

	}

	pub fn iter(&self) -> Iter<String, Modifier> {

		self.modifiers.iter()

	}

	pub fn load(&mut self, asset: ModifierAsset) {

		let name = String::from(asset.name);
		let modifier = Modifier::new(asset);

		self.modifiers.insert(name, modifier);

	}

	pub fn set_value(&mut self, name: &str, value: f64) {

		self.modifiers
			.get_mut(name)
			.map(|m| m.set_value(value));

	}

}