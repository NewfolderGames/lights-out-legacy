use std::collections::hash_map::{ HashMap, Iter };
use super::{ Technology, TechnologyAsset };

/// A technology manager.
pub struct TechnologyManager {

	technologies: HashMap<String, Technology>

}

impl TechnologyManager {

	/// Creates a new technology manager.
	pub fn new() -> Self {
		
		Self {

			technologies: HashMap::new()

		}

	}

	/// Calculates technology prices.
	pub fn calculate(&mut self, modifiers: &HashMap<String, f64>) {

		self.technologies
			.iter_mut()
			.for_each(|(_, t)| t.calculate_price(modifiers));

	}

	/// Returns a reference to an technology.
	pub fn get(&self, name: &str) -> Option<&Technology> {

		self.technologies.get(name)

	}

	/// Returns `true` if the technology is researched.
	pub fn is_researched(&self, name: &str) -> bool {

		self.technologies
			.get(name)
			.map_or(false, |t| t.is_researched())

	}

	/// Returns a technology iterator.
	pub fn iter(&self) -> Iter<String, Technology> {
		
		self.technologies.iter()

	}

	/// Loads an asset in to the manager.
	pub fn load(&mut self, asset: TechnologyAsset) {
		
		self.technologies.insert(String::from(asset.name), Technology::new(asset));

	}

	/// Researches a technology.
	pub fn research(&mut self, name: &str) {

		self.technologies
			.get_mut(name)
			.map(|u| u.research());

	}

	/// Resets the building manager.
	pub fn reset(&mut self) {
		
		self.technologies
			.iter_mut()
			.for_each(|(_, r)| r.reset());

	}

	/// Unlocks a technology.
	pub fn unlock(&mut self, name: &str) {

		self.technologies
			.get_mut(name)
			.map(|t| t.unlock());

	}

}