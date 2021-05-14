use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::rc::Rc;
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

	pub fn add(&mut self, name: &str, amount: f64) {

		self.resources
			.get_mut(name)
			.and_then(|r| {
				r.add(amount);
				Some(amount)
			});

	}

	pub fn add_resource(&mut self, asset: Rc<ResourceAsset>) {

		self.resources
			.insert(String::from(asset.name), Resource::new(asset));

	}
	
	pub fn get(&self, name: &str) -> Option<&Resource> {

		self.resources
			.get(name)

	}

	pub fn iter(&self) -> Iter<String, Resource> {

		self.resources.iter()

	}

	pub fn try_subtract(&mut self, resources: &Vec<(String, f64)>) -> bool {

		for (name, amount) in resources.iter() {

			if let Some(res) = self.resources.get(name) {

				if res.get_count() < *amount { return false }

			} else { return false }

		}

		for (name, amount) in resources.iter() {

			self.resources
				.get_mut(name)
				.and_then(|r| {
					r.try_subtract(*amount);
					Some(amount)
				});

		}

		return true;

	}

}