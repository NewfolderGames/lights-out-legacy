use std::collections::{ HashMap, hash_map::Iter };
use super::{ Resource, ResourceAsset };
use super::super::ModifierManager;

pub struct ResourceManager {

	resources: HashMap<&'static str, Resource>,

}

impl ResourceManager {

	pub fn new() -> Self {

		Self {

			resources: HashMap::new()

		}
		
	}

	pub fn calculate(&mut self, modifier_manager: &ModifierManager) {

		for (_, resource) in self.resources.iter_mut() {

			resource.calculate_capacity(modifier_manager);
			resource.calculate_production(modifier_manager);
			resource.produce();

		}

	}

	pub fn get(&self, name: &str) -> Option<&Resource> {

		self.resources.get(name)

	}

	pub fn get_mut(&mut self, name: &str) -> Option<&mut Resource> {

		self.resources.get_mut(name)

	}

	pub fn load(&mut self, asset: ResourceAsset) {

		let name = asset.name;
		let resource = Resource::new(asset);

		self.resources.insert(name, resource);

	}

	pub fn iter(&self) -> Iter<&'static str, Resource> {

		self.resources.iter()

	}

	pub fn unlock(&mut self, name: &str) {

		self.resources
			.get_mut(name)
			.map(|u| u.unlock());

	}

}