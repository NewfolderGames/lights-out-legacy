use std::collections::{ HashMap, hash_map::Iter };
use super::{ Modifier, ModifierAsset };

pub struct ModifierManager {

	modifiers: HashMap<&'static str, Modifier>,

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

	pub fn load(&mut self, asset: ModifierAsset) {

		let name = asset.name;
		let modifier = Modifier::new(asset);

		self.modifiers.insert(name, modifier);

	}

	pub fn iter(&self) -> Iter<&'static str, Modifier> {

		self.modifiers.iter()

	}

}