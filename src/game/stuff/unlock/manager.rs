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

	pub fn get_mut(&mut self, name: &str) -> Option<&mut Unlock> {

		self.unlocks.get_mut(name)

	}

	pub fn load(&mut self, asset: UnlockAsset) {

		let name = String::from(asset.name);
		let unlock = Unlock::new(asset);

		self.unlocks.insert(name, unlock);

	}

	pub fn is_unlocked(&self, name: &str) -> bool {

		self.unlocks
			.get(name)
			.map_or(false, |u| u.is_unlocked())

	}

	pub fn iter(&self) -> Iter<String, Unlock> {

		self.unlocks.iter()

	}

	pub fn set_unlock(&mut self, name: &str, unlock: bool) {

		self.unlocks
			.get_mut(name)
			.map(|u| u.set_unlock(unlock));

	}

}