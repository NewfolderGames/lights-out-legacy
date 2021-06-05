use std::collections::HashMap;
use crate::game::stuff::resource::ResourceManager;
use super::BuildingAsset;

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

	/// Creats a new building.
	pub fn new(asset: BuildingAsset) -> Self {

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

	/// Adds building count.
	pub fn add_count(&mut self, amount: i32) {

		self.count += amount;
		if self.count < 0 { self.count = 0 }

	}

	/// Calculates the building's modifiers.
	pub fn calculate_modifiers(&mut self, modifiers: &HashMap<String, f64>) {

		self.calculated_modifiers.clear();
		self.asset
			.modifiers
			.as_ref()(modifiers)
			.iter()
			.for_each(|(m_name, m_value)| { 
				self.base_modifier.push((String::from(*m_name), *m_value));
				self.calculated_modifiers.push((String::from(*m_name), *m_value * self.count as f64));
			});

	}

	/// Calculates the building's price.
	pub fn calculate_price(&mut self, modifiers: &HashMap<String, f64>) {

		self.calculated_price.clear();
		self.asset
			.price
			.to_owned()
			.iter()
			.for_each(|(r_name, r_price)| self.calculated_price.push((String::from(*r_name), r_price * self.asset.price_multiplier.powi(self.count))))

	}

	/// Checks resource deficit and then activates / deactives the building.
	pub fn check_deficit(&mut self, resource_manager: &ResourceManager) {

		if self.is_active {

			self.is_active = !self
				.asset
				.deficit
				.as_ref()(resource_manager);

		}

	}

	/// Returns the building's base modifiers.
	pub fn get_base_modifiers(&self) -> &Vec<(String, f64)> {

		&self.base_modifier

	}

	/// Returns the building's category.
	pub fn get_category(&self) -> &str {

		self.asset.category

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

	/// Resets the building.
	pub fn reset(&mut self) {
		
		self.base_modifier.clear();
		self.calculated_modifiers.clear();
		self.calculated_price.clear();
		self.count = 0;
		self.is_active = true;
		self.is_unlocked = false;

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
