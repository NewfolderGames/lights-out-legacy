use std::collections::HashMap;
use std::rc::Rc;
use super::{ Resource, ResourceData };

/// A resource manager.
pub struct ResourceManager {

	resources: HashMap<String, Resource>,

}

// Constructor.

impl ResourceManager {

	/// Creates a new resource manager.
	pub fn new() -> Self {

		Self {

			resources: HashMap::new()

		}

	}

}

// Resource handling.

impl ResourceManager {

	/// Load a resource into the manager.
	pub fn load(&mut self, data: ResourceData) {

		let name = String::from(data.name);
		let resource = Resource::new(Rc::new(data));

		self.resources.insert(name, resource);

	}
	
	/// Returns a resource.
	pub fn get(&self, name: &str) -> Option<&Resource> {

		self.resources
			.get(name)

	}

	/// Returns a resource data.
	pub fn get_data(&self, name: &str) -> Option<Rc<ResourceData>> {

		self.resources
			.get(name)
			.and_then(|res| Some(res.data()))

	}

}