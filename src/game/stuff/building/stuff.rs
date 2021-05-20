use std::collections::HashMap;
use super::BuildingAsset;
use super::super::StuffManager;

pub struct Building {

	asset: BuildingAsset,

	count: i32,
	modifiers: Vec<(&'static str, f64)>,
	price: Vec<(&'static str, f64)>,

	is_dirty: bool,
	is_unlocked: bool

}

impl Building {

	pub fn new(asset: BuildingAsset) -> Self {

		Self {

			asset,
			count: 0,
			modifiers: Vec::new(),
			price: Vec::new(),
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

		self.modifiers = self.asset.modifiers.clone();

		for (name, value) in self.modifiers.iter_mut() {

			*value *= self.count as f64;

		}

	}

	pub fn calculate_price(&mut self, stuff_manger: &StuffManager) {

		self.price = self.asset.price.clone();

		for (name, value) in self.price.iter_mut() {

			*value *= self.asset.price_multiplier.powi(self.count + 1);

		}

	}

	pub fn clear_dirty(&mut self) {

		self.is_dirty = false;

	}

	pub fn get_asset(&self) -> &BuildingAsset {

		&self.asset

	}

	pub fn get_count(&self) -> i32 {
	
		self.count

	}

	pub fn get_modifiers(&self) -> &Vec<(&'static str, f64)> {

		&self.modifiers

	}

	pub fn get_price(&self) -> &Vec<(&'static str, f64)> {

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
