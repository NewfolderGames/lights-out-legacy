use std::collections::{ HashMap, hash_map::Iter };
use super::{ Upgrade, UpgradeAsset };

pub struct UpgradeManager {

	upgrades: HashMap<String, Upgrade>,

	calculated_modifiers: HashMap<String, f64>,

	is_dirty: bool,

}

impl UpgradeManager {

	pub fn new() -> Self {

		Self {

			upgrades: HashMap::new(),
			calculated_modifiers: HashMap::new(),
			is_dirty: false,

		}

	}

	pub fn calculate(&mut self) {

		self.calculated_modifiers.clear();

		for (_, upgrade) in self.upgrades.iter() {

			if !upgrade.is_unlocked() || !upgrade.is_upgraded() { continue; }

			for (name, value) in upgrade.get_asset().price.iter() {

				if let Some(modifier) = self.calculated_modifiers.get_mut(*name) {

					*modifier += value;

				} else {

					self.calculated_modifiers.insert(String::from(*name), *value);

				}

			}

		}

	}

	pub fn clear_dirty(&mut self) {

		self.is_dirty = false;

	}

	pub fn get(&self, name: &str) -> Option<&Upgrade> {

		self.upgrades.get(name)

	}

	pub fn get_modifiers(&self) -> &HashMap<String, f64> {

		&self.calculated_modifiers

	}

	pub fn get_mut(&mut self, name: &str) -> Option<&mut Upgrade> {

		self.upgrades.get_mut(name)

	}

	pub fn load(&mut self, asset: UpgradeAsset) {

		let name = String::from(asset.name);
		let techonology = Upgrade::new(asset);

		self.upgrades.insert(name, techonology);

	}

	pub fn is_dirty(&self) -> bool {

		self.is_dirty

	}

	pub fn iter(&self) -> Iter<String, Upgrade> {

		self.upgrades.iter()

	}

	pub fn unlock(&mut self, name: &str) {

		self.upgrades
			.get_mut(name)
			.map(|u| u.unlock());

	}

	pub fn upgrade(&mut self, name: &str) {

		self.is_dirty = true;
		self.upgrades
			.get_mut(name)
			.map(|u| u.upgrade());

	}

}