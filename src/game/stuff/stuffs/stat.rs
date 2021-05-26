use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ Stuff, StuffAsset, StuffStorage };

/// A stat data.
pub struct Stat {

	asset: StatAsset,

	value: f64

}

impl Stat {

	/// Adds the stat's value.
	pub fn add_value(&mut self, amount: f64) {

		self.value += amount;
		if self.value < 0f64 { self.value = 0f64; }

	}

	/// Returns the stat's value.
	pub fn get_value(&self) -> f64 {

		self.value

	}

	/// Sets the stat's value.
	pub fn set_value(&mut self, value: f64) {

		self.value = value;

	}

}

impl Stuff for Stat {

	type Asset = StatAsset;
	type Storage = StatStorage;

	fn new(asset: StatAsset) -> Self {

		Self {

			asset,
			value: 0f64,

		}

	}

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

	fn reset(&mut self) {
		
		self.value = 0f64;

	}

}

/// A resource asset.
pub struct StatAsset {

	pub name: &'static str,
	pub category: &'static str,

}

impl StatAsset {

	/// Creates a new stat asset.
	pub fn new(name: &'static str, category: &'static str) -> Self {

		Self {

			name,
			category

		}

	}

}

impl StuffAsset for StatAsset {

	const NAME: &'static str = "asset_stat";

}

/// A stat storage.
pub struct StatStorage {

	stats: HashMap<String, Stat>

}

impl StatStorage {

	pub fn add_value(&mut self, name: &str, amount: f64) {

		self.stats
			.get_mut(name)
			.map(|s| s.add_value(amount));

	}

	/// Returns a stat's value.
	pub fn get_value(&self, name: &str) -> Option<f64> {

		self.stats
			.get(name)
			.map(|m| m.get_value())

	}

	/// Returns a stat's value.
	pub fn set_value(&self, name: &str) -> Option<f64> {

		self.stats
			.get(name)
			.map(|m| m.get_value())

	}

}

impl StuffStorage<Stat> for StatStorage {

	fn new() -> Self {

		Self {

			stats: HashMap::new(),

		}

	}

	fn get(&self, name: &str) -> Option<&Stat> {
		
		self.stats.get(name)

	}

	fn get_asset(&self, name: &str) -> Option<&StatAsset> {
		
		self.stats
			.get(name)
			.map(|b| b.get_asset())

	}

	fn get_mut(&mut self, name: &str) -> Option<&mut Stat> {
		
		self.stats.get_mut(name)

	}

	fn iter(&self) -> Iter<String, Stat> {
		
		self.stats.iter()

	}

	fn load(&mut self, asset: StatAsset) {
		
		self.stats.insert(String::from(asset.name), Stat::new(asset));

	}

	fn reset(&mut self) {
		
		self.stats
			.iter_mut()
			.for_each(|(_, r)| r.reset());

	}

}