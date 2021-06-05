use std::collections::hash_map::{ HashMap, Iter };
use super::Stat;

/// A stat manager.
pub struct StatManager {

	stats: HashMap<String, Stat>

}

impl StatManager {

	/// Creates a new stat manager.
	pub fn new() -> Self {
		
		Self {

			stats: HashMap::new()

		}

	}

	/// Adds amount to a stat's value.
	pub fn add_value(&mut self, name: &str, amount: f64) {

		self.stats
			.get_mut(name)
			.map(|s| s.add_value(amount));

	}

	/// Returns a reference to a stat.
	pub fn get(&self, name: &str) -> Option<&Stat> {

		self.stats.get(name)

	}

	/// Returns a stat's value.
	pub fn get_value(&self, name: &str) -> Option<f64> {

		self.stats
			.get(name)
			.map(|s| s.get_value())

	}

	/// Returns a modifier iterator.
	pub fn iter(&self) -> Iter<String, Stat> {
		
		self.stats.iter()

	}

	/// Loads a stat.
	pub fn load(&mut self, name: &str, category: &str) {

		self.stats.insert(String::from(name), Stat::new(category));

	}

	/// Resets stat manager.
	pub fn reset(&mut self) {

		self.stats
			.iter_mut()
			.for_each(|(_, s)| s.reset());

	}

	/// Sets a stat's value.
	pub fn set_value(&mut self, name: &str, value: f64) {

		self.stats
			.get_mut(name)
			.map(|s| s.set_value(value));

	}

}