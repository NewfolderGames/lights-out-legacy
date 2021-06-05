use std::collections::hash_map::{ HashMap, Iter };
use crate::game::stuff::resource::ResourceManager;
use super::{ Building, BuildingAsset };

/// A building manager.
pub struct BuildingManager {

	buildings: HashMap<String, Building>,

}

impl BuildingManager {

	/// Creats a new building manager.
	pub fn new() -> Self {

		Self {

			buildings: HashMap::new(),

		}

	}

	/// Adds building count.
	pub fn add_count(&mut self, name: &str, amount: i32) {

		self.buildings
			.get_mut(name)
			.map(|b| b.add_count(amount));

	}

	/// Calculates building modifiers.
	pub fn calculate(&mut self, modifiers: &mut HashMap<String, f64>, resource_manager: &ResourceManager) {
		
		for (_, building) in self.buildings.iter_mut() {

			// Calculate.

			building.calculate_modifiers(modifiers);
			building.calculate_price(modifiers);
			building.check_deficit(resource_manager);

			if !building.is_unlocked() || building.get_count() == 0 { continue; }

			// Set modifiers.

			for (name, value) in building.get_modifiers() {

				let value = if building.is_active() { *value } else { 0f64 };

				*modifiers
					.entry(name.clone())
					.or_insert(0f64) += value;

			}

		}
			
	}

	/// Returns a reference to a building.
	pub fn get(&self, name: &str) -> Option<&Building> {

		self.buildings.get(name)

	}

	/// Returns a building iterator.
	pub fn iter(&self) -> Iter<String, Building> {
		
		self.buildings.iter()

	}

	/// Loads an asset in to the manager.
	pub fn load(&mut self, asset: BuildingAsset) {
		
		self.buildings.insert(String::from(asset.name), Building::new(asset));

	}

	/// Resets the building manager.
	pub fn reset(&mut self) {
		
		self.buildings
			.iter_mut()
			.for_each(|(_, b)| b.reset());

	}

	/// Sets a building's count.
	pub fn set_count(&mut self, name: &str, amount: i32) {

		self.buildings
			.get_mut(name)
			.map(|b| b.set_count(amount));

	}

	/// Toggles a building.
	pub fn toggle(&mut self, name: &str) {

		self.buildings
			.get_mut(name)
			.map(|b| b.toggle());

	}

	/// Unlocks a building.
	pub fn unlock(&mut self, name: &str) {

		self.buildings
			.get_mut(name)
			.map(|u| u.unlock());

	}

}
