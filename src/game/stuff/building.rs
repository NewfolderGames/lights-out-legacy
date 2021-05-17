use std::rc::Rc;
use crate::game::asset::BuildingAsset;

pub struct Building {

	asset: Rc<BuildingAsset>,

	count: i32,
	modifiers: Vec<(String, f64)>,
	price: Vec<(String, f64)>,

	is_unlocked: bool

}

impl Building {

	pub fn new(asset: Rc<BuildingAsset>) -> Self {

		let price = asset.price.clone();
		let modifiers = asset.modifiers.as_ref()();

		Self {

			asset,
			count: 0,
			modifiers,
			price,
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

		self.modifiers = self.asset.modifiers.as_ref()();

		for modifier in self.modifiers.iter_mut() {

			modifier.1 *= (self.count + 1) as f64;

		}

	}

	pub fn calculate_price(&mut self) {

		self.price = self.asset.price.clone();

		for price in self.price.iter_mut() {

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

		&self.modifiers

	}

	pub fn get_price(&self) -> &Vec<(String, f64)> {

		&self.price

	}

	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}