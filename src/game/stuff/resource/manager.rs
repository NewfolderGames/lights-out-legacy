use std::collections::{ HashMap, hash_map::Iter };
use super::{ Resource, ResourceAsset };

pub struct ResourceManager {

	resources: HashMap<&'static str, Resource>,

}

impl ResourceManager {

	pub fn new() -> Self {

		Self {

			resources: HashMap::new()

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