use super::{ ResourceAsset, ResourceManager};
use super::super::{ Stuff, StuffManager };

pub struct Resource {

	asset: ResourceAsset,

	capacity: f64,
	count: f64,
	production: f64,

	is_unlocked: bool

}

impl Resource {

	pub fn new(asset: ResourceAsset) -> Self {

		Self {

			asset,
			capacity: 0f64,
			count: 0f64,
			production: 0f64,
			is_unlocked: false,

		}

	}
	
	pub fn add(&mut self, amount: f64) {

		self.count += amount;

		if self.count > self.capacity {

			self.count = self.capacity;

		}

	}

	pub fn calculate_capacity(&mut self, stuff_manager: &StuffManager) {

		self.capacity = self.asset.capacity.as_ref()(stuff_manager);

	}

	pub fn calculate_production(&mut self, stuff_manager: &StuffManager) {

		self.production = self.asset.production.as_ref()(stuff_manager);

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

	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

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

impl Stuff for Resource {

	type Asset = ResourceAsset;
	type Manager = ResourceManager;

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

}