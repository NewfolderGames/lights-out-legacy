use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ Stuff, StuffAsset, StuffStorage };

/// A modifier data.
pub struct Modifier {

	asset: ModifierAsset,

	value: f64

}

impl Modifier {

	/// Returns the modifier's value.
	pub fn get_value(&self) -> f64 {

		self.value

	}

	/// Sets the modifier's value.
	pub fn set_value(&mut self, value: f64) {

		self.value = value;

	}

}

impl Stuff for Modifier {

	type Asset = ModifierAsset;
	type Storage = ModifierStorage;

	fn new(asset: ModifierAsset) -> Self {

		Self {

			asset,
			value: 0f64

		}

	}

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

	fn reset(&mut self) {
		
		self.value = 0f64;

	}

}

/// A modifier asset.
pub struct ModifierAsset {

	pub name: &'static str

}

impl ModifierAsset {

	/// Creates a new modifier asset.
	pub fn new(name: &'static str) -> Self {

		Self {

			name

		}

	}

}

impl StuffAsset for ModifierAsset {

	const NAME: &'static str = "asset_modifier";

}

/// A modifier storage.
pub struct ModifierStorage {

	modifiers: HashMap<String, Modifier>

}

impl ModifierStorage {

	/// Returns a modifier's value.
	pub fn get_value(&self, name: &str) -> Option<f64> {

		self.modifiers
			.get(name)
			.map(|m| m.get_value())

	}

	/// Returns a modifier's value.
	pub fn set_value(&self, name: &str) -> Option<f64> {

		self.modifiers
			.get(name)
			.map(|m| m.get_value())

	}

}

impl StuffStorage<Modifier> for ModifierStorage {

	fn new() -> Self {

		Self {

			modifiers: HashMap::new(),

		}

	}

	fn get(&self, name: &str) -> Option<&Modifier> {
		
		self.modifiers.get(name)

	}

	fn get_asset(&self, name: &str) -> Option<&ModifierAsset> {
		
		self.modifiers
			.get(name)
			.map(|b| b.get_asset())

	}

	fn get_mut(&mut self, name: &str) -> Option<&mut Modifier> {
		
		self.modifiers.get_mut(name)

	}

	fn iter(&self) -> Iter<String, Modifier> {
		
		self.modifiers.iter()

	}

	fn load(&mut self, asset: ModifierAsset) {
		
		self.modifiers.insert(String::from(asset.name), Modifier::new(asset));

	}

	fn reset(&mut self) {
		
		self.modifiers
			.iter_mut()
			.for_each(|(_, r)| r.reset());

	}

}