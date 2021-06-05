use std::collections::HashMap;
use super::UpgradeAsset;

/// An upgrade data.
pub struct Upgrade {

	asset: UpgradeAsset,

	calculated_modifiers: Vec<(String, f64)>,
	calculated_price: Vec<(String, f64)>,

	is_researched: bool,
	is_unlocked: bool

}

impl Upgrade {

	/// Creates a new upgrade.
	pub fn new(asset: UpgradeAsset) -> Self {

		Self {

			asset,
			calculated_modifiers: Vec::new(),
			calculated_price: Vec::new(),
			is_researched: false,
			is_unlocked: false,

		}

	}

	/// Calculates the upgrades's modifiers.
	pub fn calculate_modifiers(&mut self, modifiers: &mut HashMap<String, f64>) {

		self.calculated_modifiers.clear();
		self.asset
			.modifiers
			.to_owned()
			.into_iter()
			.for_each(|(m_name, m_value)| self.calculated_modifiers.push((String::from(m_name), m_value)));

	}

	/// Calculates the upgrade's price.
	pub fn calculate_price(&mut self, modifiers: &mut HashMap<String, f64>) {

		self.calculated_price.clear();
		self.asset
			.price
			.to_owned()
			.into_iter()
			.for_each(|(r_name, r_price)| self.calculated_price.push((String::from(r_name), r_price)));

	}

	/// Returns the upgrade's calculated modifiers.
	pub fn get_modifiers(&self) -> &Vec<(String, f64)> {

		&self.calculated_modifiers

	}

	/// Returns calculated price.
	pub fn get_price(&self) -> &Vec<(String, f64)> {

		&self.calculated_price

	}

	/// Returns `true` if the upgrade is unlocked.
	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	/// Returns `true` if the upgrade is researched.
	pub fn is_researched(&self) -> bool {

		self.is_researched

	}

	/// Researches the upgrade.
	pub fn research(&mut self) {

		self.is_researched = true;

	}

	/// Resets the upgrade.
	pub fn reset(&mut self) {
		
		self.calculated_modifiers.clear();
		self.calculated_price.clear();
		self.is_researched = false;
		self.is_unlocked = false;

	}

	/// Unlocks the upgrade.
	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}
