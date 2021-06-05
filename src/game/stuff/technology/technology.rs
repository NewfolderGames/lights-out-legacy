use std::collections::HashMap;
use super::TechnologyAsset;

/// A technology data.
pub struct Technology {

	asset: TechnologyAsset,

	calculated_price: Vec<(String, f64)>,

	is_researched: bool,
	is_unlocked: bool

}

impl Technology {

	/// Creates a new technology.
	pub fn new(asset: TechnologyAsset) -> Self {

		Self {

			asset,
			calculated_price: Vec::new(),
			is_researched: false,
			is_unlocked: false,

		}

	}

	/// Calculates the technology's price.
	pub fn calculate_price(&mut self, modifiers: &HashMap<String, f64>) {

		self.calculated_price.clear();
		self.asset
			.price
			.to_owned()
			.into_iter()
			.for_each(|(name, value)| self.calculated_price.push((String::from(name), value)));

	}

	/// Returns calculated price.
	pub fn get_price(&self) -> &Vec<(String, f64)> {

		&self.calculated_price

	}

	/// Returns `true` if the technology is unlocked.
	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	/// Returns `true` if the technology is researched.
	pub fn is_researched(&self) -> bool {

		self.is_researched

	}

	/// Researches the technology.
	pub fn research(&mut self) {

		self.is_researched = true;

	}

	/// Resets the technology.
	pub fn reset(&mut self) {
		
		self.calculated_price.clear();
		self.is_researched = false;
		self.is_unlocked = false;

	}

	/// Unlocks the technology.
	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}