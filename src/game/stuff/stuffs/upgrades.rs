use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ ModifierStorage, Stuff, StuffAsset, StuffStorage };

/// A upgrade data.
pub struct Upgrade {

	asset: UpgradeAsset,

	is_upgraded: bool,
	is_unlocked: bool

}

impl Upgrade {

	/// Returns `true` if the upgrade is unlocked.
	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	/// Returns `true` if the upgrade is researched.
	pub fn is_upgraded(&self) -> bool {

		self.is_upgraded

	}

	/// Researches the upgrade.
	pub fn upgrade(&mut self) {

		self.is_upgraded = true;

	}

	/// Unlocks the upgrade.
	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}

impl Stuff for Upgrade {

	type Asset = UpgradeAsset;
	type Storage = UpgradeStorage;

	fn new(asset: UpgradeAsset) -> Self {

		Self {

			asset,
			is_upgraded: false,
			is_unlocked: false,

		}

	}

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

	fn reset(&mut self) {
		
		self.is_upgraded = false;
		self.is_unlocked = false;

	}

}

/// A upgrade asset.
pub struct UpgradeAsset {

	pub name: &'static str,

	pub modifiers: Box<dyn Fn(&ModifierStorage) -> Vec<(&'static str, f64)>>,
	pub price: Box<dyn Fn(&ModifierStorage) -> Vec<(&'static str, f64)>>,

}

impl UpgradeAsset {

	/// Creates a new upgrade asset.
	pub fn new(
		name: &'static str,
		modifiers: Box<dyn Fn(&ModifierStorage) -> Vec<(&'static str, f64)>>,
		price: Box<dyn Fn(&ModifierStorage) -> Vec<(&'static str, f64)>>
	) -> Self {

		Self {

			name,
			modifiers,
			price,

		}

	}

}

impl StuffAsset for UpgradeAsset {

	const NAME: &'static str = "asset_upgrade";

}

/// A upgrade storage.
pub struct UpgradeStorage {

	upgrades: HashMap<String, Upgrade>,

	calculated_modifiers: HashMap<String, f64>,

}

impl UpgradeStorage {

	/// Calculates upgrade modifiers.
	pub fn calculate(&mut self, modifier_storage: &ModifierStorage) {

		self.calculated_modifiers.clear();

		for (_, upgrade) in self.upgrades.iter_mut() {

			if !upgrade.is_unlocked() || !upgrade.is_upgraded() { continue; }

			for (name, value) in upgrade.get_asset().modifiers.as_ref()(modifier_storage).iter() {

				if let Some(modifier) = self.calculated_modifiers.get_mut(*name) {

					*modifier += value;

				} else {

					self.calculated_modifiers.insert(String::from(*name), *value);

				}

			}

		}

	}

	/// Returns calculate modifiers.
	pub fn get_modifiers(&self) -> &HashMap<String, f64> {

		&self.calculated_modifiers

	}

	/// Unlocks a upgrade.
	pub fn unlock(&mut self, name: &str) {

		self.upgrades
			.get_mut(name)
			.map(|u| u.unlock());

	}

}

impl StuffStorage<Upgrade> for UpgradeStorage {

	fn new() -> Self {

		Self {

			upgrades: HashMap::new(),
			calculated_modifiers: HashMap::new()

		}

	}

	fn get(&self, name: &str) -> Option<&Upgrade> {
		
		self.upgrades.get(name)

	}

	fn get_asset(&self, name: &str) -> Option<&UpgradeAsset> {
		
		self.upgrades
			.get(name)
			.map(|b| b.get_asset())

	}

	fn get_mut(&mut self, name: &str) -> Option<&mut Upgrade> {
		
		self.upgrades.get_mut(name)

	}

	fn iter(&self) -> Iter<String, Upgrade> {
		
		self.upgrades.iter()

	}

	fn load(&mut self, asset: UpgradeAsset) {
		
		self.upgrades.insert(String::from(asset.name), Upgrade::new(asset));

	}

	fn reset(&mut self) {
		
		self.upgrades
			.iter_mut()
			.for_each(|(_, r)| r.reset());

	}

}