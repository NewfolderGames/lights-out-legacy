use crate::game::stuff::modifier::ModifierManager;
use super::ResourceAsset;

/// A resource data.
pub struct Resource {

	asset: ResourceAsset,

	capacity: f64,
	consumption: f64,
	count: f64,
	production: f64,

	is_deficit: bool,
	is_unlocked: bool,

}

impl Resource {
	
	/// Creates a new resource.
	pub fn new(asset: ResourceAsset) -> Self {

		Self {

			capacity: asset.capacity,
			consumption: 0f64,
			count: 0f64,
			production: 0f64,
			is_deficit: false,
			is_unlocked: false,
			asset,

		}

	}

	/// Adds resource count.
	pub fn add_count(&mut self, amount: f64) {

		self.is_deficit = false;

		if amount > 0f64 {

			if self.count >= self.capacity { return }

			self.count += amount;
			if self.count >= self.capacity { self.count = self.capacity }

		} else {

			self.count += amount;
			if self.count < 0f64 {
			
				self.count = 0f64;
				self.is_deficit = true;
			
			}

		}

	}

	/// Calculates the resource capacity.
	pub fn calculate_capacity(&mut self, modifier_manager: &ModifierManager) {

		self.capacity = self.asset.capacity;
		self.capacity +=
			modifier_manager.get_value(&format!("resource_{}_capacity_base", self.asset.name)).unwrap_or(1f64) +
			modifier_manager.get_value(&format!("resource_category_{}_capacity_base", self.asset.category)).unwrap_or(0f64);
		self.capacity *= 1f64 +
			modifier_manager.get_value(&format!("resource_{}_capacity_multiplier", self.asset.name)).unwrap_or(0f64) +
			modifier_manager.get_value(&format!("resource_category_{}_capacity_multiplier", self.asset.category)).unwrap_or(0f64);

	}

	/// Calculates the resource production.
	pub fn calculate_production(&mut self, modifier_manager: &ModifierManager) {

		self.production = 0f64;
		self.production +=
			modifier_manager.get_value(&format!("resource_{}_production_base", self.asset.name)).unwrap_or(0f64) +
			modifier_manager.get_value(&format!("resource_category_{}_production_base", self.asset.category)).unwrap_or(0f64);
		self.production *= 1f64 +
			modifier_manager.get_value(&format!("resource_{}_production_mutiplier", self.asset.name)).unwrap_or(0f64) +
			modifier_manager.get_value(&format!("resource_category_{}_production_mutiplier", self.asset.category)).unwrap_or(0f64);
		
	}

	/// Calculates the resource consumption.
	pub fn calculate_consumption(&mut self, modifier_manager: &ModifierManager) {

		self.consumption = 0f64;
		self.consumption +=
			modifier_manager.get_value(&format!("resource_{}_consumption_base", self.asset.name)).unwrap_or(0f64) +
			modifier_manager.get_value(&format!("resource_category_{}_consumption_base", self.asset.category)).unwrap_or(0f64);
		self.consumption *= 1f64 +
			modifier_manager.get_value(&format!("resource_{}_consumption_mutiplier", self.asset.name)).unwrap_or(0f64) +
			modifier_manager.get_value(&format!("resource_category_{}_consumption_mutiplier", self.asset.category)).unwrap_or(0f64);
		
	}

	/// Returns the resource's capacity.
	pub fn get_capacity(&self) -> f64 {

		self.capacity

	}

	/// Returns the resource's category.
	pub fn get_category(&self) -> &str {

		self.asset.category

	}

	/// Returns the resource's consumption.
	pub fn get_consumption(&self) -> f64 {

		self.consumption

	}

	/// Returns the resource's count.
	pub fn get_count(&self) -> f64 {

		self.count

	}

	/// Returns the resource's production.
	pub fn get_production(&self) -> f64 {

		self.production

	}

	/// Returns `true` if the resource is in deficit.
	pub fn is_deficit(&self) -> bool {

		self.is_deficit

	}

	/// Returns `true` if the resource's count equals its capacity.
	pub fn is_full(&self) -> bool {

		self.count == self.capacity

	}

	/// Returns `true` if the resource's count equals 0.
	pub fn is_empty(&self) -> bool {

		self.count == 0f64

	}

	/// Returns `true` if the resource is unlocked.
	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	/// Resets the resource.
	pub fn reset(&mut self) {
		
		self.count = 0f64;
		self.is_unlocked = false;

	}

	/// Sets the resouce's count.
	pub fn set_count(&mut self, amount: f64) {

		self.count = amount;

	}

	/// Unlocks the resource.
	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}