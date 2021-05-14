use std::collections::HashMap;
use std::rc::Rc;
use super::BuildingData;

/// A building manager.
pub struct BuildingManager {

	buildings: HashMap<String, Rc<BuildingData>>

}

// Constructor.

impl BuildingManager {

	/// Creates a new building manager.
	pub fn new() -> Self {

		Self {

			buildings: HashMap::new()

		}

	}

}

// Building handling.

impl BuildingManager {

	/// Loads a building into the manager.
	pub fn load(&mut self, data: BuildingData) {

		let name = String::from(data.name);
		let building = Rc::new(data);

		self.buildings.insert(name, building);

	}

}