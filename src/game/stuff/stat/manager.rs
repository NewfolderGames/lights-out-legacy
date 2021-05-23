use std::collections::{ HashMap, hash_map::Iter };
use super::{ Stat, StatAsset };

pub struct StatManager {

	stats: HashMap<String, Stat>,

}

impl StatManager {

	pub fn new() -> Self {

		Self {

			stats: HashMap::new()

		}
		
	}

	pub fn add(&mut self, name: &str, amount: f64) {

		self.stats
			.get_mut(name)
			.map(|s| s.add(amount));

	}

	pub fn get(&self, name: &str) -> Option<&Stat> {
		
		self.stats.get(name)
		
	}
	
	pub fn get_mut(&mut self, name: &str) -> Option<&mut Stat> {
		
		self.stats.get_mut(name)
		
	}
	
	pub fn get_value(&self, name: &str) -> Option<f64> {

		self.stats
			.get(name)
			.map(|m| m.get_value())

	}

	pub fn iter(&self) -> Iter<String, Stat> {

		self.stats.iter()

	}

	pub fn load(&mut self, asset: StatAsset) {

		let name = String::from(asset.name);
		let stat = Stat::new(asset);

		self.stats.insert(name, stat);

	}

	pub fn set_value(&mut self, name: &str, value: f64) {

		self.stats
			.get_mut(name)
			.map(|m| m.set_value(value));

	}

}