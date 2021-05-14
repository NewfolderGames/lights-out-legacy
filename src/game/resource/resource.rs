use std::rc::Rc;
use super::ResourceAsset;

pub struct Resource {

	asset: Rc<ResourceAsset>,

	is_unlocked: bool,
	is_hidden: bool,

	capacity: f64,
	count: f64,
	production: f64,

	is_dirty: bool,

}

impl Resource {

	pub fn new(asset: Rc<ResourceAsset>) -> Self {

		let is_unlocked = asset.is_unlocked;
		let is_hidden = asset.is_hidden;
		let capacity = asset.default_capacity;

		Self {

			asset,
			is_unlocked,
			is_hidden,
			capacity,
			count: 0f64,
			production: 0f64,
			is_dirty: true,

		}

	}

}

// Numbers.

impl Resource {

	pub fn add(&mut self, amount: f64) {

		self.count += amount;
		self.is_dirty = true;

		if self.count > self.capacity {

			self.count = self.capacity;

		}

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

	pub fn set_capacity(&mut self, amount: f64) {

		self.capacity = amount;
		self.is_dirty = true;

	}

	pub fn set_count(&mut self, amount: f64) {

		self.count = amount;
		self.is_dirty = true;

	}

	pub fn set_production(&mut self, amount: f64) {

		self.production = amount;
		self.is_dirty = true;

	}

}

// Other.

impl Resource {

	pub fn asset(&self) -> Rc<ResourceAsset> {

		self.asset.clone()

	}

	pub fn is_dirty(&self) -> bool {

		self.is_dirty

	}

	pub fn clear_dirty(&mut self) {

		self.is_dirty = false;

	}

}