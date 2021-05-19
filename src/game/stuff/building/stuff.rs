use std::collections::HashMap;
use super::BuildingAsset;
use super::super::StuffManager;

pub struct Building {

	asset: BuildingAsset,

	count: i32,
	modifiers: HashMap<String, f64>,
	price: HashMap<String, f64>,

	is_dirty: bool,
	is_unlocked: bool

}

impl Building {

	pub fn new(asset: BuildingAsset) -> Self {

		Self {

			asset,
			count: 0,
			modifiers: HashMap::new(),
			price: HashMap::new(),
			is_dirty: true,
			is_unlocked: false,

		}

	}

	pub fn build(&mut self, amount: i32) {

		self.is_dirty = true;
		self.count += amount;
		if self.count < 0 { self.count = 0 }

	}

	pub fn calculate_modifiers(&mut self, stuff_manger: &StuffManager) {

		self.modifiers = self.asset.modifiers.as_ref()(stuff_manger);

		for modifier in self.modifiers.iter_mut() {

			*modifier.1 *= (self.count + 1) as f64;

		}

	}

	pub fn calculate_price(&mut self, stuff_manger: &StuffManager) {

		self.price = self.asset.price.as_ref()(stuff_manger);

		for price in self.price.iter_mut() {

			*price.1 *= self.asset.price_multiplier.powi(self.count + 1);

		}

	}

	pub fn get_asset(&self) -> &BuildingAsset {

		&self.asset

	}

	pub fn get_count(&self) -> i32 {
	
		self.count

	}

	pub fn get_modifiers(&self) -> &HashMap<String, f64> {

		&self.modifiers

	}

	pub fn get_price(&self) -> &HashMap<String, f64> {

		&self.price

	}

	pub fn is_dirty(&self) -> bool {

		self.is_dirty

	}

	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}
