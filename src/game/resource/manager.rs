use std::collections::HashMap;
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

}

impl ResourceManager {
	
	pub fn get(&self, name: &str) -> Option<&Resource> {

		self.resources
			.get(name)

	}

}