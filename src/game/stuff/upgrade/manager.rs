use std::collections::{ HashMap, hash_map::Iter };
use super::{ Upgrade, UpgradeAsset };

pub struct UpgradeManager {

	upgrades: HashMap<String, Upgrade>,

	calculated_modifiers: HashMap<String, f64>,

}

impl UpgradeManager {

	pub fn new() -> Self {

		Self {

			upgrades: HashMap::new(),
			calculated_modifiers: HashMap::new(),

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

	pub fn iter(&self) -> Iter<String, Upgrade> {

		self.upgrades.iter()

	}

	pub fn set_unlock(&mut self, name: &str, unlock: bool) {

		self.upgrades
			.get_mut(name)
			.map(|u| u.set_unlock(unlock));

	}

}