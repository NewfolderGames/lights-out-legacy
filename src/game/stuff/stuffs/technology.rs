use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ ModifierStorage, Stuff, StuffAsset, StuffStorage };

/// A technology data.
pub struct Technology {

	asset: TechnologyAsset,

	is_researched: bool,
	is_unlocked: bool

}

impl Technology {

	/// Returns `true` if the technology is unlocked.
	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	/// Returns `true` if the technology is researched.
	pub fn is_researched(&self) -> bool {

		self.is_researched

	}

	/// Researches the technology.
	pub fn research(&mut self) {

		self.is_researched = true;

	}

	/// Unlocks the technology.
	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}

impl Stuff for Technology {

	type Asset = TechnologyAsset;
	type Storage = TechnologyStorage;

	fn new(asset: TechnologyAsset) -> Self {

		Self {

			asset,
			is_researched: false,
			is_unlocked: false,

		}

	}

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

	fn reset(&mut self) {
		
		self.is_researched = false;
		self.is_unlocked = false;

	}

}

/// A technology asset.
pub struct TechnologyAsset {

	pub name: &'static str,

	pub price: Box<dyn Fn(&ModifierStorage) -> Vec<(&'static str, f64)>>,
	pub unlock: &'static str,

}

impl TechnologyAsset {

	/// Creates a new technology asset.
	pub fn new(
		name: &'static str,
		price: Box<dyn Fn(&ModifierStorage) -> Vec<(&'static str, f64)>>,
		unlock: &'static str
	) -> Self {

		Self {

			name,
			price,
			unlock

		}

	}

}

impl StuffAsset for TechnologyAsset {

	const NAME: &'static str = "asset_technology";

}

/// A technology storage.
pub struct TechnologyStorage {

	technologies: HashMap<String, Technology>

}

impl TechnologyStorage {

	/// Unlocks a technology.
	pub fn unlock(&mut self, name: &str) {

		self.technologies
			.get_mut(name)
			.map(|t| t.unlock());

	}

}

impl StuffStorage<Technology> for TechnologyStorage {

	fn new() -> Self {

		Self {

			technologies: HashMap::new(),

		}

	}

	fn get(&self, name: &str) -> Option<&Technology> {
		
		self.technologies.get(name)

	}

	fn get_asset(&self, name: &str) -> Option<&TechnologyAsset> {
		
		self.technologies
			.get(name)
			.map(|b| b.get_asset())

	}

	fn get_mut(&mut self, name: &str) -> Option<&mut Technology> {
		
		self.technologies.get_mut(name)

	}

	fn iter(&self) -> Iter<String, Technology> {
		
		self.technologies.iter()

	}

	fn load(&mut self, asset: TechnologyAsset) {
		
		self.technologies.insert(String::from(asset.name), Technology::new(asset));

	}

	fn reset(&mut self) {
		
		self.technologies
			.iter_mut()
			.for_each(|(_, r)| r.reset());

	}

}