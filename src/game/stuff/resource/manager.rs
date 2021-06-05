use std::collections::hash_map::{ HashMap, Iter };
use crate::game::stuff::modifier::ModifierManager;
use super::{ Resource, ResourceAsset };

/// A resource manager.
pub struct ResourceManager {

	resources: HashMap<String, Resource>

}

impl ResourceManager {
	
	/// Creates a new resource manager.
	pub fn new() -> Self {
		
		Self {

			resources: HashMap::new()

		}

	}

	/// Adds a resource's count.
	pub fn add_count(&mut self, name: &str, amount: f64) {

		self.resources
			.get_mut(name)
			.map(|r| r.add_count(amount));

	}

	/// Calculates resources.
	pub fn calculate(&mut self, modifier_manager: &ModifierManager) {

		self.resources
			.iter_mut()
			.for_each(|(_, r)| {

				r.calculate_production(&modifier_manager);
				r.calculate_consumption(&modifier_manager);
				r.calculate_capacity(&modifier_manager);

				r.add_count(r.get_production() - r.get_consumption());

			})

	}

	/// Returns a reference to a resource.
	pub fn get(&self, name: &str) -> Option<&Resource> {
		
		self.resources.get(name)

	}

	/// Returns a resource iterator.
	pub fn iter(&self) -> Iter<String, Resource> {
		
		self.resources.iter()

	}

	/// Loads an asset in to the manager.
	pub fn load(&mut self, asset: ResourceAsset) {
		
		self.resources.insert(String::from(asset.name), Resource::new(asset));

	}

	/// Resets the resource manager.
	pub fn reset(&mut self) {
		
		self.resources
			.iter_mut()
			.for_each(|(_, r)| r.reset());

	}

	/// Sets a resource's count.
	pub fn set_count(&mut self, name: &str, amount: f64) {

		self.resources
			.get_mut(name)
			.map(|r| r.set_count(amount));

	}

	/// Spends resources.
	/// Returns `false` if failed.
	pub fn spend_resources(&mut self, resources: &Vec<(String, f64)>) -> bool {

		for (name, count) in resources.iter() {

			if let Some(r) = self.resources.get(name) {

				if r.get_count() < *count { return false; }

			} else { return false; }

		}

		for (name, count) in resources.iter() {

			self.resources
				.get_mut(name)
				.map(|r| r.add_count(-count));

		}

		return true;

	}

	/// Unlocks a resource.
	pub fn unlock(&mut self, name: &str) {

		self.resources
			.get_mut(name)
			.map(|r| r.unlock());

	}

}
