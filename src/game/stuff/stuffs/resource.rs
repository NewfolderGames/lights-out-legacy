use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ ModifierStorage, Stuff, StuffAsset, StuffStorage };

/// A resource data.
pub struct Resource {

	asset: ResourceAsset,

	capacity: f64,
	consumption: f64,
	count: f64,
	production: f64,

	is_unlocked: bool

}

impl Resource {

	/// Adds resource count.
	pub fn add_count(&mut self, amount: f64) {

		self.count += amount;

		if self.count > self.capacity { self.count = self.capacity; }
		if self.count < 0f64 { self.count = 0f64; }

	}

	/// Calculates resource capacity.
	pub fn calculate_capacity(&mut self, modifier_storage: &ModifierStorage) {

		self.capacity = self.asset.capacity;
		self.capacity +=
			modifier_storage.get_value(&["modifier_resource_", self.asset.name, "_storage_base"].join("")).unwrap_or(0f64) +
			modifier_storage.get_value(&["modifier_resource_", self.asset.category, "_storage_base"].join("")).unwrap_or(0f64);
		self.capacity *= 1f64 +
			modifier_storage.get_value(&["modifier_resource_", self.asset.name, "_storage_multiplier"].join("")).unwrap_or(0f64) +
			modifier_storage.get_value(&["modifier_resource_", self.asset.category, "_storage_multiplier"].join("")).unwrap_or(0f64);

	}

	/// Calculates resource production.
	pub fn calculate_production(&mut self, modifier_storage: &ModifierStorage) {

		self.production = 0f64;
		self.production +=
			modifier_storage.get_value(&["modifier_resource_", self.asset.name, "_production_base"].join("")).unwrap_or(0f64) +
			modifier_storage.get_value(&["modifier_resource_", self.asset.category, "_production_base"].join("")).unwrap_or(0f64);
		self.production *= 1f64 +
			modifier_storage.get_value(&["modifier_resource_", self.asset.name, "_production_multiplier"].join("")).unwrap_or(0f64) +
			modifier_storage.get_value(&["modifier_resource_", self.asset.category, "_production_multiplier"].join("")).unwrap_or(0f64);
		
	}

	/// Calculates resource consumption.
	pub fn calculate_consumption(&mut self, modifier_storage: &ModifierStorage) {

		self.consumption = 0f64;
		self.consumption +=
			modifier_storage.get_value(&["modifier_resource_", self.asset.name, "_consumption_base"].join("")).unwrap_or(0f64) +
			modifier_storage.get_value(&["modifier_resource_", self.asset.category, "_consumption_base"].join("")).unwrap_or(0f64);
		self.consumption *= 1f64 +
			modifier_storage.get_value(&["modifier_resource_", self.asset.name, "_consumption_multiplier"].join("")).unwrap_or(0f64) +
			modifier_storage.get_value(&["modifier_resource_", self.asset.category, "_consumption_multiplier"].join("")).unwrap_or(0f64);
		
	}

	/// Returns the resource's capacity.
	pub fn get_capacity(&self) -> f64 {

		self.capacity

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

	/// Sets the resouce's count.
	pub fn set_count(&mut self, amount: f64) {

		self.count = amount;

	}

	/// Unlocks the resource.
	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}

impl Stuff for Resource {

	type Asset = ResourceAsset;
	type Storage = ResourceStorage;

	fn new(asset: ResourceAsset) -> Self {

		Self {

			capacity: asset.capacity,
			consumption: 0f64,
			count: 0f64,
			production: 0f64,
			is_unlocked: false,
			asset,

		}

	}

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

	fn reset(&mut self) {
		
		self.is_unlocked = false;
		self.count = 0f64;

	}

}

/// A resource asset.
pub struct ResourceAsset {

	pub name: &'static str,

	pub category: &'static str,
	pub capacity: f64,

}

impl ResourceAsset {

	/// Creates a new resource asset.
	pub fn new(name: &'static str, category: &'static str, capacity: f64) -> Self {

		Self {

			name,
			category,
			capacity

		}

	}

}

impl StuffAsset for ResourceAsset {

	const NAME: &'static str = "asset_resource";

}

/// A resource storage.
pub struct ResourceStorage {

	resources: HashMap<String, Resource>

}

impl ResourceStorage {
	
	/// Adds a resource's count.
	pub fn add_count(&mut self, name: &str, amount: f64) {

		self.resources
			.get_mut(name)
			.map(|r| r.add_count(amount));

	}

	/// Calculates resources.
	pub fn calculate(&mut self, modifier_storage: &ModifierStorage) {

		self.resources
			.iter_mut()
			.for_each(|(_, r)| {

				r.calculate_production(&modifier_storage);
				r.calculate_consumption(&modifier_storage);
				r.calculate_capacity(&modifier_storage);

				r.add_count(r.get_production() - r.get_consumption());

			})

	}

	/// Sets a resource's count.
	pub fn set_count(&mut self, name: &str, amount: f64) {

		self.resources
			.get_mut(name)
			.map(|r| r.set_count(amount));

	}

	/// Unlocks a resource.
	pub fn unlock(&mut self, name: &str) {

		self.resources
			.get_mut(name)
			.map(|r| r.unlock());

	}

}

impl StuffStorage<Resource> for ResourceStorage {

	fn new() -> Self {

		Self {

			resources: HashMap::new(),

		}

	}

	fn get(&self, name: &str) -> Option<&Resource> {
		
		self.resources.get(name)

	}

	fn get_asset(&self, name: &str) -> Option<&ResourceAsset> {
		
		self.resources
			.get(name)
			.map(|b| b.get_asset())

	}

	fn get_mut(&mut self, name: &str) -> Option<&mut Resource> {
		
		self.resources.get_mut(name)

	}

	fn iter(&self) -> Iter<String, Resource> {
		
		self.resources.iter()

	}

	fn load(&mut self, asset: ResourceAsset) {
		
		self.resources.insert(String::from(asset.name), Resource::new(asset));

	}

	fn reset(&mut self) {
		
		self.resources
			.iter_mut()
			.for_each(|(_, r)| r.reset());

	}

}