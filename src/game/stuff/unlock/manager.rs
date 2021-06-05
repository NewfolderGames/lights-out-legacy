use std::collections::hash_map::{ HashMap, Iter };
use super::{ Unlock, UnlockAsset };

pub struct UnlockManager {

	unlocks: HashMap<String, Unlock>

}

impl UnlockManager {

	/// Creates a new unlock manager.
	pub fn new() -> Self {
		
		Self {

			unlocks: HashMap::new()

		}

	}

	/// Returns a reference to unlock.
	pub fn get(&self, name: &str) -> Option<&Unlock> {

		self.unlocks.get(name)

	}

	/// Returns a mutable reference to unlock.
	pub fn get_mut(&mut self, name: &str) -> Option<&mut Unlock> {

		self.unlocks.get_mut(name)

	}
	
	/// Returns `true` if unlocked.
	pub fn is_unlocked(&self, name: &str) -> bool {

		self.unlocks
			.get(name)
			.map_or(false, |u| u.is_unlocked())

	}

	/// Returns an unlock iterator.
	pub fn iter(&self) -> Iter<String, Unlock> {

		self.unlocks.iter()

	}

	/// Resets unlock manager.
	pub fn reset(&mut self) {

		self.unlocks
			.iter_mut()
			.for_each(|(_, u)| u.reset());

	}

	/// Loads an asset into the manager.
	pub fn load(&mut self, asset: UnlockAsset) {

		self.unlocks.insert(String::from(asset.name), Unlock::new(asset));

	}

}
