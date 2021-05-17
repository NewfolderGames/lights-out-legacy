use std::rc::Rc;
use crate::game::asset::BuildingAsset;

pub struct Building {

	asset: Rc<BuildingAsset>,

	count: i32,
	calculated_price: Vec<(String, f64)>,
	calculated_modifiers: Vec<(String, f64)>,

	is_unlocked: bool

}

impl Building {

	pub fn new(asset: Rc<BuildingAsset>) -> Self {

		let price = asset.price.clone();

		Self {

			asset,
			count: 0,
			calculated_price: price,
			calculated_modifiers: Vec::new(),
			is_unlocked: false,

		}

	}

	pub fn build(&mut self, amount: i32) {

		self.count += amount;
		if self.count < 0 { self.count = 0 }

		self.calculate_modifiers();
		self.calculate_price();

	}

	pub fn calculate_modifiers(&mut self) {

		self.calculated_modifiers = self.asset.modifiers.as_ref()();

		for modifier in self.calculated_modifiers.iter_mut() {

			modifier.1 *= (self.count + 1) as f64;

		}

	}

	pub fn calculate_price(&mut self) {

		self.calculated_price = self.asset.price.clone();

		for price in self.calculated_price.iter_mut() {

			price.1 *= self.asset.price_multiplier.powi(self.count + 1);

		}

	}

	pub fn get_asset(&self) -> Rc<BuildingAsset> {

		self.asset.clone()

	}

	pub fn get_count(&self) -> i32 {
	
		self.count

	}

	pub fn get_modifiers(&self) -> &Vec<(String, f64)> {

		&self.calculated_modifiers

	}

	pub fn get_price(&self) -> &Vec<(String, f64)> {

		&self.calculated_price

	}

	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}