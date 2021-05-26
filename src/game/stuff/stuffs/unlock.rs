use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ Stuff, StuffAsset, StuffStorage };

/// Unlockable stuffs.
pub enum Unlockable {

	Building(&'static str),
	Feature(&'static str),
	Policy(&'static str),
	Resource(&'static str),
	Technology(&'static str),
	Unlock(&'static str),
	Upgrade(&'static str),

}

/// A unlock data.
pub struct Unlock {

	asset: UnlockAsset,

	is_unlocked: bool,

}

impl Unlock {

	/// Returns `true` if unlocked.
	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	/// Unlocks.
	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}

impl Stuff for Unlock {

	type Asset = UnlockAsset;
	type Storage = UnlockStorage;

	fn new(asset: UnlockAsset) -> Self {

		Self {

			asset,
			is_unlocked: false,

		}

	}

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

	fn reset(&mut self) {
		
		self.is_unlocked = false;

	}

}

/// A unlock asset.
pub struct UnlockAsset {

	pub name: &'static str,
	pub unlocks: Vec<Unlockable>,

}

impl UnlockAsset {

	/// Creates a new unlock asset.
	pub fn new(name: &'static str, unlocks: Vec<Unlockable>) -> Self {

		Self {

			name,
			unlocks

		}

	}

}

impl StuffAsset for UnlockAsset {

	const NAME: &'static str = "asset_unlock";

}

/// A unlock storage.
pub struct UnlockStorage {

	unlocks: HashMap<String, Unlock>

}

impl StuffStorage<Unlock> for UnlockStorage {

	fn new() -> Self {

		Self {

			unlocks: HashMap::new(),

		}

	}

	fn get(&self, name: &str) -> Option<&Unlock> {
		
		self.unlocks.get(name)

	}

	fn get_asset(&self, name: &str) -> Option<&UnlockAsset> {
		
		self.unlocks
			.get(name)
			.map(|b| b.get_asset())

	}

	fn get_mut(&mut self, name: &str) -> Option<&mut Unlock> {
		
		self.unlocks.get_mut(name)

	}

	fn iter(&self) -> Iter<String, Unlock> {
		
		self.unlocks.iter()

	}

	fn load(&mut self, asset: UnlockAsset) {
		
		self.unlocks.insert(String::from(asset.name), Unlock::new(asset));

	}

	fn reset(&mut self) {
		
		self.unlocks
			.iter_mut()
			.for_each(|(_, r)| r.reset());

	}

}