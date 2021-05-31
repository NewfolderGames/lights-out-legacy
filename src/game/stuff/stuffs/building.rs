use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ ModifierStorage, ResourceStorage, Stuff, StuffAsset, StuffStorage };

// A building data.
pub struct Building {

	asset: BuildingAsset,
	
	base_modifier: Vec<(String, f64)>,
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

	/// Calculates the building's modifiers.
	pub fn calculate_modifiers(&mut self, modifier_storage: &ModifierStorage) {

		self.base_modifier.clear();
		self.calculated_modifiers.clear();
		self.asset
			.modifiers
			.as_ref()(modifier_storage)
			.iter()
			.for_each(|(m_name, m_value)| { 
				self.base_modifier.push((String::from(*m_name), *m_value));
				self.calculated_modifiers.push((String::from(*m_name), *m_value * self.count as f64));
			});

	}

	/// Calculates the building's price.
	pub fn calculate_price(&mut self, modifier_storage: &ModifierStorage) {

		self.calculated_price.clear();
		self.asset
			.price
			.to_owned()
			.iter()
			.for_each(|(r_name, r_price)| self.calculated_price.push((String::from(*r_name), r_price * self.asset.price_multiplier.powi(self.count))))

	}

	/// Checks resource deficit and activates / deactives the building.
	pub fn check_deficit(&mut self, resource_storage: &ResourceStorage) {

		self.is_active = !self
			.asset
			.deficit
			.as_ref()(resource_storage);

	}

	/// Returns the building's base modifiers.
	pub fn get_base_modifiers(&self) -> &Vec<(String, f64)> {

		&self.base_modifier

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

	/// Returns `ture` if the building is enabled.
	pub fn is_active(&self) -> bool {

		self.is_active

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

	/// Togles the building.
	pub fn toggle(&mut self) {

		self.is_active = !self.is_active;

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
			base_modifier: Vec::new(),
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
		self.is_active = true;
		self.count = 0;

	}

}

/// A building asset.
pub struct BuildingAsset {

	pub name: &'static str,

	pub category: &'static str,
	pub modifiers: Box<dyn Fn(&ModifierStorage) -> Vec<(&'static str, f64)>>,
	pub deficit: Box<dyn Fn(&ResourceStorage) -> bool>,
	pub price: Vec<(&'static str, f64)>,
	pub price_multiplier: f64,

}

impl BuildingAsset {

	/// Creates a new building asset.
	pub fn new(
		name: &'static str,
		category: &'static str,
		modifiers: Box<dyn Fn(&ModifierStorage) -> Vec<(&'static str, f64)>>,
		deficit: Box<dyn Fn(&ResourceStorage) -> bool>,
		price: Vec<(&'static str, f64)>,
		price_multiplier: f64
	) -> Self {

		Self {

			name,
			category,
			modifiers,
			deficit,
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

			// Calculate.

			building.calculate_modifiers(modifier_storage);
			building.calculate_price(modifier_storage);
			building.check_deficit(resource_storage);

			if !building.is_unlocked() || building.count == 0 { continue; }

			// Set modifiers.

			for (name, value) in building.get_modifiers() {

				let value = if building.is_active() { *value } else { 0f64 };

				if let Some(modifiers) = self.calculated_modifiers.get_mut(name) {

					*modifiers += value;

				} else {

					self.calculated_modifiers.insert(String::from(name), value);

				}

			}

		}
			
	}

	/// Checks resource deficit and activates / deactivates buildings.
	pub fn check_deficit(&mut self, resource_storage: &ResourceStorage) {

		for (_, building) in self.buildings.iter_mut() {

			building.check_deficit(resource_storage);

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