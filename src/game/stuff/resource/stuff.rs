use super::ResourceAsset;
use super::super::ModifierManager;

pub struct Resource {

	asset: ResourceAsset,

	capacity: f64,
	count: f64,
	production: f64,
	consumption: f64,

	is_depleted: bool,
	is_full: bool,

	is_unlocked: bool

}

impl Resource {

	pub fn new(asset: ResourceAsset) -> Self {

		Self {

			asset,
			capacity: 0f64,
			count: 0f64,
			production: 0f64,
			consumption: 0f64,
			is_depleted: false,
			is_full: false,
			is_unlocked: false,

		}

	}
	
	pub fn add(&mut self, amount: f64) {

		self.count += amount;

		if self.count > self.capacity {

			self.count = self.capacity;

		}

	}

	pub fn calculate_capacity(&mut self, modifier_manager: &ModifierManager) {

		self.capacity = self.asset.capacity;
		self.capacity +=
			modifier_manager.get_value(&["modifier_resource_", self.asset.name, "_storage_base"].join("")).unwrap_or(0f64) +
			modifier_manager.get_value(&["modifier_resource_", self.asset.category, "_storage_base"].join("")).unwrap_or(0f64);
		self.capacity *=
			modifier_manager.get_value(&["modifier_resource_", self.asset.name, "_storage_multiplier"].join("")).unwrap_or(1f64) +
			modifier_manager.get_value(&["modifier_resource_", self.asset.category, "_storage_multiplier"].join("")).unwrap_or(1f64);

	}

	pub fn calculate_production(&mut self, modifier_manager: &ModifierManager) {

		self.production = 0f64;
		self.production +=
			modifier_manager.get_value(&["modifier_resource_", self.asset.name, "_production_base"].join("")).unwrap_or(0f64) +
			modifier_manager.get_value(&["modifier_resource_", self.asset.category, "_production_base"].join("")).unwrap_or(0f64);
		self.production *=
			modifier_manager.get_value(&["modifier_resource_", self.asset.name, "_production_multiplier"].join("")).unwrap_or(1f64) +
			modifier_manager.get_value(&["modifier_resource_", self.asset.category, "_production_multiplier"].join("")).unwrap_or(1f64);
		
	}

	pub fn calculate_consumption(&mut self, modifier_manager: &ModifierManager) {

		self.consumption = 0f64;
		self.consumption +=
			modifier_manager.get_value(&["modifier_resource_", self.asset.name, "_consumption_base"].join("")).unwrap_or(0f64) +
			modifier_manager.get_value(&["modifier_resource_", self.asset.category, "_consumption_base"].join("")).unwrap_or(0f64);
		self.consumption *=
			modifier_manager.get_value(&["modifier_resource_", self.asset.name, "_consumption_multiplier"].join("")).unwrap_or(1f64) +
			modifier_manager.get_value(&["modifier_resource_", self.asset.category, "_consumption_multiplier"].join("")).unwrap_or(1f64);
		
	}

	pub fn get_asset(&self) -> &ResourceAsset {

		&self.asset

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

	pub fn tick(&mut self) {

		self.count += self.production;
		self.count -= self.consumption;

		if self.count <= 0f64 {

			self.count = 0f64;
			self.is_depleted = true;

		} else if self.count > self.capacity {
			
			self.count = self.capacity;
			self.is_full = false;
	
		} else {

			self.is_depleted = false;
			self.is_full = false;

		}

	}

	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}
