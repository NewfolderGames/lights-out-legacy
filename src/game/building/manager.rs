use std::collections::HashMap;
use std::rc::Rc;
use super::BuildingAsset;

pub struct BuildingManager {

	buildings: HashMap<String, Rc<BuildingAsset>>

}

impl BuildingManager {

	pub fn new() -> Self {

		Self {

			buildings: HashMap::new()

		}

	}

}
