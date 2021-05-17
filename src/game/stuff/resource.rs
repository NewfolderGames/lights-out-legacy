use std::rc::Rc;
use crate::game::asset::ResourceAsset;

pub struct Resource {

	asset: Rc<ResourceAsset>,

	capacity: f64,
	count: f64,
	production: f64,

	is_unlocked: bool

}

impl Resource {

	pub fn new(asset: Rc<ResourceAsset>) -> Self {

		let capacity = asset.capacity.as_ref()();
		let production = asset.production.as_ref()();

		Self {

			asset,
			capacity,
			count: 0f64,
			production,
			is_unlocked: false,

		}

	}
	
	pub fn add(&mut self, amount: f64) {

		self.count += amount;

		if self.count > self.capacity {

			self.count = self.capacity;

		}

	}

	pub fn calculate_capacity(&mut self) {

		self.capacity = self.asset.capacity.as_ref()();

	}

	pub fn calculate_production(&mut self) {

		self.production = self.asset.production.as_ref()();

	}

	pub fn get_asset(&self) -> Rc<ResourceAsset> {

		self.asset.clone()

	}

	pub fn get_capacity(&self) -> f64 {

		self.capacity

	}

	pub fn get_count(&self) -> f64 {

		self.count

	}

	pub fn get_production(&self) -> f64 {

		self.production

	}

	pub fn set_count(&mut self, amount: f64) {

		self.count = amount;

	}

	pub fn try_spend(&mut self, amount: f64) -> bool {

		if self.count < amount { return false }
		self.count -= amount;
		return true;

	}

	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}
