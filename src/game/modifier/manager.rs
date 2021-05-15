use std::collections::HashMap;
use std::rc::Rc;
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

	pub fn add_modifier(&mut self, asset: Rc<ModifierAsset>) {

		self.modifiers
			.insert(String::from(asset.name), Modifier::new(asset));

	}

	pub fn get(&self, name: &str) -> Option<&Modifier> {

		self.modifiers.get(name)

	}

	pub fn get_value_unchecked(&self, name: &str) -> f64 {

		self.get(name)
			.and_then(|m| Some(m.get_value()))
			.unwrap_or(0f64)

	}

}
