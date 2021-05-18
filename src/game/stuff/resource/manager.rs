use std::collections::HashMap;
use super::{ Resource, ResourceAsset };

pub struct ResourceManager {

	resources: HashMap<String, Resource>,

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

	pub fn load(&mut self, asset: ResourceAsset) {

		let name = String::from(asset.name);
		let resource = Resource::new(asset);

		self.resources.insert(name, resource);

	}

}