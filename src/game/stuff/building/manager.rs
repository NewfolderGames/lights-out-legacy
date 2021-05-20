use std::collections::{ HashMap, hash_map::Iter };
use super::{ Building, BuildingAsset };
use super::super::StuffManager;

pub struct BuildingManager {

	buildings: HashMap<&'static str, Building>,

	calculated_modifiers: HashMap<String, f64>,

	is_dirty: bool,

}

impl BuildingManager {

	pub fn new() -> Self {

		Self {

			buildings: HashMap::new(),
			calculated_modifiers: HashMap::new(),
			is_dirty: false,

		}
		
	}

	pub fn calculate(&mut self, stuff_manager: &StuffManager) {

		let mut modifiers: HashMap<&'static str, f64> = HashMap::new();

		for (_, building) in self.buildings.iter_mut() {

			if building.is_dirty() {

				building.calculate_modifiers(stuff_manager);
				building.calculate_price(stuff_manager);
				building.clear_dirty();

			}

			for (name, value) in building.get_modifiers() {

				if let Some(modifiers) = modifiers.get_mut(name) {

					*modifiers += value;

				} else {

					modifiers.insert(*name, *value);

				}

			}

		}

	}

	pub fn clear_dirty(&mut self) {

		self.is_dirty = false;

	}

	pub fn get(&self, name: &str) -> Option<&Building> {

		self.buildings.get(name)

	}

	pub fn get_mut(&mut self, name: &str) -> Option<&mut Building> {

		self.buildings.get_mut(name)

	}

	pub fn is_dirty(&self) -> bool {

		self.is_dirty

	}

	pub fn load(&mut self, asset: BuildingAsset) {

		let name = asset.name;
		let building = Building::new(asset);

		self.buildings.insert(name, building);

	}

	pub fn iter(&self) -> Iter<&'static str, Building> {

		self.buildings.iter()

	}

	pub fn unlock(&mut self, name: &str) {

		self.buildings
			.get_mut(name)
			.map(|u| u.unlock());

	}

}
