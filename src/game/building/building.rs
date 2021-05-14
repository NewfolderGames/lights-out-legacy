use std::rc::Rc;
use super::BuildingAsset;

pub struct Building {

	asset: Rc<BuildingAsset>,

	count: i32,
	calculated_price: Vec<(String, f64)>,
	calculated_modifiers: Vec<(String, f64)>,

	is_dirty: bool,

}

impl Building {

	pub fn new(asset: Rc<BuildingAsset>) -> Self {

		let price = asset.price.clone();
		let modifiers = asset.modifiers.clone();

		Self {

			asset,
			count: 0,
			calculated_price: price,
			calculated_modifiers: modifiers,
			is_dirty: true,

		}

	}

	pub fn add(&mut self, amount: i32) {

		self.is_dirty = true;
		self.count += amount;

		if self.count < 0 { self.count = 0 }

		self.update_price();
		self.update_modifiers();

	}

	pub fn update_modifiers(&mut self) {

		self.is_dirty = true;
		self.calculated_modifiers = self.asset.modifiers.clone();

		for modifier in self.calculated_modifiers.iter_mut() {

			modifier.1 *= self.count as f64;

		}

	}

	pub fn update_price(&mut self) {

		self.is_dirty = true;
		self.calculated_price = self.asset.price.clone();

		for price in self.calculated_price.iter_mut() {

			price.1 *= self.asset.price_multiplier.powi(self.count + 1);

		}

	}

}