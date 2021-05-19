use std::collections::HashMap;
use super::{ Building, BuildingAsset };

pub struct BuildingManager {

	buildings: HashMap<String, Building>,

}

impl BuildingManager {

	pub fn new() -> Self {

		Self {

			buildings: HashMap::new()

		}
		
	}

	pub fn get(&self, name: &str) -> Option<&Building> {

		self.buildings.get(name)

	}

	pub fn load(&mut self, asset: BuildingAsset) {

		let name = String::from(asset.name);
		let building = Building::new(asset);

		self.buildings.insert(name, building);

	}

	pub fn iter(&self) -> std::collections::hash_map::Iter<String, Building> {

		self.buildings.iter()

	}

}
