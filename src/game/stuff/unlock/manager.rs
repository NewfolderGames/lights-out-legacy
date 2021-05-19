use std::collections::{ HashMap, hash_map::Iter };
use super::{ Unlock, UnlockAsset };

pub struct UnlockManager {

	unlocks: HashMap<String, Unlock>,

}

impl UnlockManager {

	pub fn new() -> Self {

		Self {

			unlocks: HashMap::new()

		}
		
	}

	pub fn get(&self, name: &str) -> Option<&Unlock> {

		self.unlocks.get(name)

	}

	pub fn load(&mut self, asset: UnlockAsset) {

		let name = String::from(asset.name);
		let unlock = Unlock::new(asset);

		self.unlocks.insert(name, unlock);

	}

	pub fn iter(&self) -> Iter<String, Unlock> {

		self.unlocks.iter()

	}

}