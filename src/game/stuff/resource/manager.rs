use std::collections::{ HashMap, hash_map::Iter };
use super::{ Resource, ResourceAsset };
use super::super::ModifierManager;

pub struct ResourceManager {

	resources: HashMap<String, Resource>,

}

impl ResourceManager {

	pub fn new() -> Self {

		Self {

			resources: HashMap::new()

		}
		
	}

	pub fn add(&mut self, name: &str, amount: f64) {

		self.resources
			.get_mut(name)
			.map(|r| r.add(amount));

	}

	pub fn calculate(&mut self, modifier_manager: &ModifierManager) {

		for (_, resource) in self.resources.iter_mut() {

			resource.calculate_capacity(modifier_manager);
			resource.calculate_production(modifier_manager);
			resource.calculate_consumption(modifier_manager);
			resource.tick();

		}

	}

	pub fn get(&self, name: &str) -> Option<&Resource> {

		self.resources.get(name)

	}

	pub fn get_mut(&mut self, name: &str) -> Option<&mut Resource> {

		self.resources.get_mut(name)

	}

	pub fn load(&mut self, asset: ResourceAsset) {

		let name = String::from(asset.name);
		let resource = Resource::new(asset);

		self.resources.insert(name, resource);

	}

	pub fn iter(&self) -> Iter<String, Resource> {

		self.resources.iter()

	}

	pub fn set_unlock(&mut self, name: &str, unlock: bool) {

		self.resources
			.get_mut(name)
			.map(|u| u.set_unlock(unlock));

	}

}