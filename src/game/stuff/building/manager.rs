use std::collections::{ HashMap, hash_map::Iter };
use super::{ Building, BuildingAsset };
use super::super::ModifierManager;

pub struct BuildingManager {

	buildings: HashMap<String, Building>,

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

	pub fn calculate(&mut self, modifier_manager: &ModifierManager) {

		self.calculated_modifiers.clear();

		for (_, building) in self.buildings.iter_mut() {

			if !building.is_unlocked() || building.get_count() == 0 { continue; }

			if building.is_dirty() {

				building.calculate_modifiers(modifier_manager);
				building.calculate_price(modifier_manager);
				building.clear_dirty();

			}

			for (name, value) in building.get_modifiers() {

				if let Some(modifiers) = self.calculated_modifiers.get_mut(name) {

					*modifiers += value;

				} else {

					self.calculated_modifiers.insert(String::from(name), *value);

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

	pub fn get_modifiers(&self) -> &HashMap<String, f64> {

		&self.calculated_modifiers

	}

	pub fn get_mut(&mut self, name: &str) -> Option<&mut Building> {

		self.buildings.get_mut(name)

	}

	pub fn is_dirty(&self) -> bool {

		self.is_dirty

	}

	pub fn iter(&self) -> Iter<String, Building> {

		self.buildings.iter()

	}

	pub fn load(&mut self, asset: BuildingAsset) {

		let name = String::from(asset.name);
		let building = Building::new(asset);

		self.buildings.insert(name, building);

	}

	pub fn reset(&mut self) {

		self.is_dirty = true;
		self.buildings
			.iter_mut()
			.for_each(|(_, b)| b.reset());

	}

	pub fn set_count(&mut self, name: &str, amount: i32) {

		self.is_dirty = true;
		self.buildings
			.get_mut(name)
			.map(|b| b.set_count(amount));

	}

	pub fn unlock(&mut self, name: &str) {

		self.buildings
			.get_mut(name)
			.map(|u| u.unlock());

	}

}
