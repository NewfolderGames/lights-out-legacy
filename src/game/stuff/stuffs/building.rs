use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ ModifierStorage, ResourceStorage, Stuff, StuffAsset, StuffStorage };

// A building data.
pub struct Building {

	asset: BuildingAsset,
	
	calculated_modifiers: Vec<(String, f64)>,
	calculated_price: Vec<(String, f64)>,
	count: i32,

	is_active: bool,
	is_unlocked: bool,

}

impl Building {

	/// Adds building count.
	pub fn add_count(&mut self, amount: i32) {

		self.count += amount;
		if self.count < 0 { self.count = 0 }

	}

	/// Calculates building modifiers.
	pub fn calculate_modifiers(&mut self, modifier_storage: &ModifierStorage, resource_storage: &ResourceStorage) {

		self.calculated_modifiers.clear();
		self.asset
			.modifiers
			.as_ref()(modifier_storage, resource_storage)
			.iter()
			.for_each(|(m_name, m_value)| self.calculated_modifiers.push((String::from(*m_name), *m_value * self.count as f64)));

	}

	/// Calculates building price.
	pub fn calculate_price(&mut self, modifier_storage: &ModifierStorage, resource_storage: &ResourceStorage) {

		self.calculated_price.clear();
		self.asset
			.modifiers
			.as_ref()(modifier_storage, resource_storage)
			.iter()
			.for_each(|(r_name, r_price)| self.calculated_price.push((String::from(*r_name), r_price * self.asset.price_multiplier.powi(self.count + 1))))

	}

	/// Returns the building's count.
	pub fn get_count(&self) -> i32 {
	
		self.count

	}

	/// Returns the building's calculated modifiers.
	pub fn get_modifiers(&self) -> &Vec<(String, f64)> {

		&self.calculated_modifiers

	}

	/// Returns the building's calculated price.
	pub fn get_price(&self) -> &Vec<(String, f64)> {

		&self.calculated_price

	}

	/// Returns `true` if the building is unlocked.
	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	/// Disables or enables the building.
	pub fn set_active(&mut self, active: bool) {

		self.is_active = active;

	}

	/// Sets the building's count.
	pub fn set_count(&mut self, amount: i32) {

		self.count = amount;
		if self.count < 0 { self.count = 0 }

	}

	/// Unlocks the building.
	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}

impl Stuff for Building {

	type Asset = BuildingAsset;
	type Storage = BuildingStorage;

	fn new(asset: BuildingAsset) -> Self {

		Self {

			asset,
			calculated_modifiers: Vec::new(),
			calculated_price: Vec::new(),
			count: 0,
			is_active: true,
			is_unlocked: false,

		}

	}

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

	fn reset(&mut self) {
		
		self.is_unlocked = false;
		self.count = 0;

	}

}

/// A building asset.
pub struct BuildingAsset {

	pub name: &'static str,

	pub category: &'static str,
	pub modifiers: Box<dyn Fn(&ModifierStorage, &ResourceStorage) -> Vec<(&'static str, f64)>>,
	pub price: Box<dyn Fn(&ModifierStorage, &ResourceStorage) -> Vec<(&'static str, f64)>>,
	pub price_multiplier: f64,

}

impl BuildingAsset {

	/// Creates a new building asset.
	pub fn new(
		name: &'static str,
		category: &'static str,
		modifiers: Box<dyn Fn(&ModifierStorage, &ResourceStorage) -> Vec<(&'static str, f64)>>,
		price: Box<dyn Fn(&ModifierStorage, &ResourceStorage) -> Vec<(&'static str, f64)>>,
		price_multiplier: f64
	) -> Self {

		Self {

			name,
			category,
			modifiers,
			price,
			price_multiplier

		}
		
	}

}

impl StuffAsset for BuildingAsset {

	const NAME: &'static str = "asset_building";

}

/// A building storage.
pub struct BuildingStorage {

	buildings: HashMap<String, Building>,
	
	calculated_modifiers: HashMap<String, f64>,

}

impl BuildingStorage {

	/// Adds building count.
	pub fn add_count(&mut self, name: &str, amount: i32) {

		self.buildings
			.get_mut(name)
			.map(|b| b.add_count(amount));

	}

	/// Calculates building modifiers.
	pub fn calculate(&mut self, modifier_storage: &ModifierStorage, resource_storage: &ResourceStorage) {

		self.calculated_modifiers.clear();

		for (_, building) in self.buildings.iter_mut() {

			if !building.is_unlocked() || !building.get_count() == 0 || !building.is_active { continue; }

			// Calculate.

			building.calculate_modifiers(modifier_storage, resource_storage);
			building.calculate_price(modifier_storage, resource_storage);

			// Set modifiers.

			for (name, value) in building.get_modifiers() {

				if let Some(modifiers) = self.calculated_modifiers.get_mut(name) {

					*modifiers += value;

				} else {

					self.calculated_modifiers.insert(String::from(name), *value);

				}

			}

		}
			
	}

	/// Returns calculated modifiers.
	pub fn get_modifiers(&self) -> &HashMap<String, f64> {

		&self.calculated_modifiers

	}

	/// Sets a building's count.
	pub fn set_count(&mut self, name: &str, amount: i32) {

		self.buildings
			.get_mut(name)
			.map(|b| b.set_count(amount));

	}

	/// Unlocks a building.
	pub fn unlock(&mut self, name: &str) {

		self.buildings
			.get_mut(name)
			.map(|u| u.unlock());

	}

}

impl StuffStorage<Building> for BuildingStorage {

	fn new() -> Self {

		Self {

			buildings: HashMap::new(),
			calculated_modifiers: HashMap::new(),

		}

	}

	fn get(&self, name: &str) -> Option<&Building> {
		
		self.buildings.get(name)

	}

	fn get_asset(&self, name: &str) -> Option<&BuildingAsset> {
		
		self.buildings
			.get(name)
			.map(|b| b.get_asset())

	}

	fn get_mut(&mut self, name: &str) -> Option<&mut Building> {
		
		self.buildings.get_mut(name)

	}

	fn iter(&self) -> Iter<String, Building> {
		
		self.buildings.iter()

	}

	fn load(&mut self, asset: BuildingAsset) {
		
		self.buildings.insert(String::from(asset.name), Building::new(asset));

	}

	fn reset(&mut self) {
		
		self.buildings
			.iter_mut()
			.for_each(|(_, b)| b.reset());

	}

}