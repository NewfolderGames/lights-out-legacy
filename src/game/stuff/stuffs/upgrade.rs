use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ ModifierStorage, Stuff, StuffAsset, StuffStorage };

/// A upgrade data.
pub struct Upgrade {

	asset: UpgradeAsset,

	calculated_modifiers: Vec<(String, f64)>,
	calculated_price: Vec<(String, f64)>,

	is_researched: bool,
	is_unlocked: bool

}

impl Upgrade {

	/// Calculates the upgrades's modifiers.
	pub fn calculate_modifiers(&mut self, modifier_storage: &ModifierStorage) {

		self.calculated_modifiers.clear();
		self.asset
			.modifiers
			.to_owned()
			.into_iter()
			.for_each(|(m_name, m_value)| self.calculated_modifiers.push((String::from(m_name), m_value)));

	}

	/// Calculates the upgrade's price.
	pub fn calculate_price(&mut self, modifier_storage: &ModifierStorage) {

		self.calculated_price.clear();
		self.asset
			.price
			.to_owned()
			.into_iter()
			.for_each(|(r_name, r_price)| self.calculated_price.push((String::from(r_name), r_price)));

	}

	/// Returns the upgrade's calculated modifiers.
	pub fn get_modifiers(&self) -> &Vec<(String, f64)> {

		&self.calculated_modifiers

	}

	/// Returns calculated price.
	pub fn get_price(&self) -> &Vec<(String, f64)> {

		&self.calculated_price

	}

	/// Returns `true` if the upgrade is unlocked.
	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	/// Returns `true` if the upgrade is researched.
	pub fn is_researched(&self) -> bool {

		self.is_researched

	}

	/// Researches the upgrade.
	pub fn research(&mut self) {

		self.is_researched = true;

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
			calculated_modifiers: Vec::new(),
			calculated_price: Vec::new(),
			is_researched: false,
			is_unlocked: false,

		}

	}

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

	fn reset(&mut self) {
		
		self.is_researched = false;
		self.is_unlocked = false;

	}

}

/// A upgrade asset.
pub struct UpgradeAsset {

	pub name: &'static str,

	pub modifiers:  Vec<(&'static str, f64)>,
	pub price: Vec<(&'static str, f64)>,

}

impl UpgradeAsset {

	/// Creates a new upgrade asset.
	pub fn new(name: &'static str, modifiers: Vec<(&'static str, f64)>,price: Vec<(&'static str, f64)>) -> Self {

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

			upgrade.calculate_modifiers(modifier_storage);
			upgrade.calculate_price(modifier_storage);

			if !upgrade.is_unlocked() || !upgrade.is_researched() { continue; }

			for (name, value) in upgrade.get_modifiers() {

				if let Some(modifier) = self.calculated_modifiers.get_mut(name) {

					*modifier += value;

				} else {

					self.calculated_modifiers.insert(String::from(name), *value);

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