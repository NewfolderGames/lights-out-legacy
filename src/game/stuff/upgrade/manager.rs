use std::collections::hash_map::{ HashMap, Iter };
use super::{ Upgrade, UpgradeAsset };

/// An upgrade manager.
pub struct UpgradeManager {

	upgrades: HashMap<String, Upgrade>,

}

impl UpgradeManager {

	/// Creates a new upgrade manager.
	pub fn new() -> Self {
		
		Self {

			upgrades: HashMap::new()

		}

	}

	/// Calculates upgrade modifiers.
	pub fn calculate(&mut self, modifiers: &mut HashMap<String, f64>) {

		for (_, upgrade) in self.upgrades.iter_mut() {

			upgrade.calculate_modifiers(modifiers);
			upgrade.calculate_price(modifiers);

			if !upgrade.is_unlocked() || !upgrade.is_researched() { continue; }

			for (name, value) in upgrade.get_modifiers() {

				*modifiers
					.entry(name.clone())
					.or_insert(0f64) += value;

			}

		}

	}

	/// Returns a reference to an upgrade.
	pub fn get(&self, name: &str) -> Option<&Upgrade> {

		self.upgrades.get(name)

	}

	/// Returns `true` if the upgrade is researched.
	pub fn is_researched(&self, name: &str) -> bool {

		self.upgrades
			.get(name)
			.map_or(false, |t| t.is_researched())

	}

	/// Returns a upgrade iterator.
	pub fn iter(&self) -> Iter<String, Upgrade> {
		
		self.upgrades.iter()

	}

	/// Loads an asset in to the manager.
	pub fn load(&mut self, asset: UpgradeAsset) {
		
		self.upgrades.insert(String::from(asset.name), Upgrade::new(asset));

	}

	/// Researches an upgrade.
	pub fn research(&mut self, name: &str) {

		self.upgrades
			.get_mut(name)
			.map(|u| u.research());

	}

	/// Resets the upgrade manager.
	pub fn reset(&mut self) {
		
		self.upgrades
			.iter_mut()
			.for_each(|(_, r)| r.reset());

	}

	/// Unlocks a upgrade.
	pub fn unlock(&mut self, name: &str) {

		self.upgrades
			.get_mut(name)
			.map(|u| u.unlock());

	}

}
