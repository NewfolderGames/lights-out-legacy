use std::rc::Rc;
use super::ResourceData;

/// A resource.
pub struct Resource {

	data: Rc<ResourceData>,

	is_unlocked: bool,
	is_hidden: bool,

	count: f64,
	capacity: f64,
	production: f64,

	is_count_dirty: bool,
	is_capacity_dirty: bool,
	is_production_dirty: bool,

}

// Constructor.

impl Resource {

	/// Creates a new resource.
	pub fn new(data: Rc<ResourceData>) -> Self {

		let is_unlocked = data.is_unlocked;
		let is_hidden = data.is_hidden;
		let capacity = data.default_capacity;

		Self {

			data,
			is_unlocked,
			is_hidden,
			count: 0f64,
			capacity,
			production: 0f64,
			is_count_dirty: true,
			is_capacity_dirty: true,
			is_production_dirty: true,

		}

	}

}

// Numbers.

impl Resource {

	/// Adds amount to count.
	pub fn add(&mut self, amount: f64) {

		self.count += amount;
		self.is_count_dirty = true;

		if self.count > self.capacity {

			self.count = self.capacity;

		}

	}

	/// Returns resource count.
	pub fn count(&self) -> f64 {

		self.count

	}

	/// Returns resource capacity.
	pub fn capacity(&self) -> f64 {

		self.capacity

	}

	/// Returns resource production.
	pub fn production(&self) -> f64 {

		self.production

	}

	/// Sets count.
	pub fn set_count(&mut self, amount: f64) {

		self.count = amount;
		self.is_count_dirty = true;

	}

	/// Sets capacity.
	pub fn set_capacity(&mut self, amount: f64) {

		self.capacity = amount;
		self.is_capacity_dirty = true;

	}

	/// Sets production.
	pub fn set_production(&mut self, amount: f64) {

		self.production = amount;
		self.is_production_dirty = true;

	}

}

// Other.

impl Resource {

	/// Returns resource data.
	pub fn data(&self) -> Rc<ResourceData> {

		self.data.clone()

	}

	/// Checks whether count or capacity changed or not.
	pub fn is_dirty(&self) -> bool {

		self.is_count_dirty || self.is_capacity_dirty || self.is_production_dirty

	}

	/// Clears dirty state.
	pub fn clear_dirty(&mut self) {

		self.is_count_dirty = false;
		self.is_capacity_dirty = false;
		self.is_production_dirty = false;

	}

}