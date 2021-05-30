use std::collections::hash_map::{ HashMap, Iter };
use super::*;

pub struct StuffManager {

	building_storage: BuildingStorage,
	feature_storage: FeatureStorage,
	modifier_storage: ModifierStorage,
	resource_storage: ResourceStorage,
	stat_storage: StatStorage,
	technology_storage: TechnologyStorage,
	text_storage: TextStorage,
	unlock_storage: UnlockStorage,
	upgrade_storage: UpgradeStorage,

}

impl StuffManager {

	/// Creates a new manager.
	pub fn new() -> Self {

		Self {

			building_storage: BuildingStorage::new(),
			feature_storage: FeatureStorage::new(),
			modifier_storage: ModifierStorage::new(),
			resource_storage: ResourceStorage::new(),
			stat_storage: StatStorage::new(),
			technology_storage: TechnologyStorage::new(),
			text_storage: TextStorage::new(),
			unlock_storage: UnlockStorage::new(),
			upgrade_storage: UpgradeStorage::new(),

		}

	}

	/// Adds a building's count.
	pub fn add_building(&mut self, name: &str, amount: i32) {

		self.building_storage.add_count(name, amount);

	}

	/// Adds a resource's count.
	pub fn add_resource(&mut self, name: &str, amount: f64) {

		self.resource_storage.add_count(name, amount);

	}

	/// Adds a stat's value.
	pub fn add_stat(&mut self, name: &str, amount: f64) {

		self.stat_storage.add_value(name, amount);

	}

	/// Returns a reference to a building.
	pub fn get_building(&self, name: &str) -> Option<&Building> {

		self.building_storage.get(name)

	}

	/// Returns a modifier's value.
	pub fn get_modifier_value(&self, name: &str) -> Option<f64> {

		self.modifier_storage
			.get(name)
			.map(|m| m.get_value())

	}

	/// Returns a reference to a resource.
	pub fn get_resource(&self, name: &str) -> Option<&Resource> {

		self.resource_storage.get(name)

	}

	/// Returns a reference to a stat.
	pub fn get_stat(&self, name: &str) -> Option<&Stat> {

		self.stat_storage.get(name)

	}

	/// Returns a reference to a technology.
	pub fn get_technology(&self, name: &str) -> Option<&Technology> {

		self.technology_storage.get(name)

	}

	/// Returns a reference to a text.
	pub fn get_text(&self, name: &str) -> Option<&str> {

		self.text_storage.get(name)

	}

	/// Returns a reference to a unlock.
	pub fn get_unlock(&self, name: &str) -> Option<&Unlock> {

		self.unlock_storage.get(name)

	}


	/// Returns `true` if the feature is unlocked.
	pub fn is_feature_unlocked(&self, name: &str) -> bool {

		self.feature_storage
			.get(name)
			.map_or(false,|f| f.is_unlocked())

	}

	/// Returns `true` if the technology is researched.
	pub fn is_technology_researched(&self, name: &str) -> bool {

		self.technology_storage
			.get(name)
			.map_or(false,|t| t.is_researched())

	}

	/// Retuns `true` if unlocked.
	pub fn is_unlocked(&self, name: &str) -> bool {

		self.unlock_storage
			.get(name)
			.map_or(false, |u| u.is_unlocked())

	}

	/// Returns `true` if the upgrade is researched.
	pub fn is_upgrade_researched(&self, name: &str) -> bool {

		self.upgrade_storage
			.get(name)
			.map_or(false,|u| u.is_researched())

	}

	/// Returns a building iterator.
	pub fn iter_building(&self) -> Iter<String, Building> {

		self.building_storage.iter()

	}

	/// Returns a feature iterator.
	pub fn iter_feature(&self) -> Iter<String, Feature> {

		self.feature_storage.iter()

	}

	/// Returns a modifier iterator.
	pub fn iter_modifier(&self) -> Iter<String, Modifier> {

		self.modifier_storage.iter()

	}

	/// Returns a resource iterator.
	pub fn iter_resource(&self) -> Iter<String, Resource> {

		self.resource_storage.iter()

	}

	/// Returns a stat iterator.
	pub fn iter_stat(&self) -> Iter<String, Stat> {

		self.stat_storage.iter()

	}

	/// Returns a technology iterator.
	pub fn iter_technology(&self) -> Iter<String, Technology> {

		self.technology_storage.iter()

	}

	/// Returns a unlock iterator.
	pub fn iter_unlock(&self) -> Iter<String, Unlock> {

		self.unlock_storage.iter()

	}

	/// Returns a unlock iterator.
	pub fn iter_upgrade(&self) -> Iter<String, Upgrade> {

		self.upgrade_storage.iter()

	}
		
	/// Loads a building.
	pub fn load_building(&mut self, asset: BuildingAsset) {

		self.building_storage.load(asset);

	}

	/// Loads a feature.
	pub fn load_feature(&mut self, asset: FeatureAsset) {

		self.feature_storage.load(asset);

	}

	/// Loads a modifier.
	pub fn load_modifier(&mut self, asset: ModifierAsset) {

		self.modifier_storage.load(asset);

	}

	/// Loads a resource.
	pub fn load_resource(&mut self, asset: ResourceAsset) {

		self.resource_storage.load(asset);

	}

	/// Loads a stat.
	pub fn load_stat(&mut self, asset: StatAsset) {

		self.stat_storage.load(asset);

	}

	/// Loads a technology.
	pub fn load_technology(&mut self, asset: TechnologyAsset) {

		self.technology_storage.load(asset);

	}

	/// Loads a text.
	pub fn load_text(&mut self, asset: TextAsset) {

		self.text_storage.load(asset);

	}

	/// Loads a unlock.
	pub fn load_unlock(&mut self, asset: UnlockAsset) {

		self.unlock_storage.load(asset);

	}

	/// Loads a unlock.
	pub fn load_upgrade(&mut self, asset: UpgradeAsset) {

		self.upgrade_storage.load(asset);

	}

	/// Tries to purchase a building.
	pub fn purchase_building(&mut self, name: &str) -> bool {

		if let Some(building) = self.building_storage.get(name) {

			if !building.is_unlocked() { return false; }

			if self.resource_storage.spend_resources(building.get_price()) {

				self.add_building(name, 1);
				return true;

			}

		}

		false

	}

	/// Tries to purchase a technology.
	pub fn purchase_technology(&mut self, name: &str) -> bool {

		if let Some(technology) = self.technology_storage.get(name) {

			if !technology.is_unlocked() || technology.is_researched() { return false; }

			if self.resource_storage.spend_resources(technology.get_price()) {

				self.research_technology(name);
				return true;

			}

		}

		false

	}

	/// Tries to purchase a upgrade.
	pub fn purchase_upgrade(&mut self, name: &str) -> bool {

		if let Some(upgrade) = self.upgrade_storage.get(name) {

			if !upgrade.is_unlocked() || upgrade.is_researched() { return false; }

			if self.resource_storage.spend_resources(upgrade.get_price()) {

				self.research_upgrade(name);
				return true;

			}

		}

		false

	}

	/// Resets all stuffs to original state.
	pub fn reset(&mut self) {

		self.building_storage.reset();
		self.feature_storage.reset();
		self.modifier_storage.reset();
		self.resource_storage.reset();
		self.stat_storage.reset();
		self.technology_storage.reset();
		self.unlock_storage.reset();
		self.upgrade_storage.reset();

	}

	/// Researchs a technology.
	pub fn research_technology(&mut self, name: &str) {

		self.technology_storage
			.get_mut(name)
			.map(|t| t.research());

	}

	/// Researchs a upgrade.
	pub fn research_upgrade(&mut self, name: &str) {

		self.upgrade_storage
			.get_mut(name)
			.map(|u| u.research());

	}

	/// Sets a building's count.
	pub fn set_building(&mut self, name: &str, amount: i32) {

		self.building_storage
			.get_mut(name)
			.map(|b| b.set_count(amount));

	}

	/// Sets a resource's count.
	pub fn set_resource(&mut self, name: &str, amount: f64) {

		self.resource_storage
			.get_mut(name)
			.map(|r| r.set_count(amount));

	}

	/// Sets a stat's value.
	pub fn set_stat(&mut self, name: &str, amount: f64) {

		self.stat_storage
			.get_mut(name)
			.map(|s| s.set_value(amount));

	}

	/// Game tick.
	pub fn tick(&mut self) {

		// Calculate resource.

		self.resource_storage.calculate(&self.modifier_storage);

		// Calculate modifiers.

		let mut modifiers: HashMap<String, f64> = HashMap::new();

		self.upgrade_storage.calculate(&self.modifier_storage);
		self.technology_storage.calculate(&self.modifier_storage);
		self.building_storage.calculate(&self.modifier_storage);

		self.upgrade_storage
			.get_modifiers()
			.iter()
			.for_each(|(m_name, m_value)| { modifiers.insert(String::from(m_name), *m_value); });

		self.building_storage
			.get_modifiers()
			.iter()
			.for_each(|(m_name, m_value)| {
				if let Some(modifier) = modifiers.get_mut(m_name) { *modifier += m_value; }
				else { modifiers.insert(String::from(m_name), *m_value); }
			});

		modifiers.iter().for_each(|(name, value)| self.modifier_storage.set_value(name, *value));

		// Stats.

		self.add_stat("stat_game_ticks_current", 1f64);
		self.add_stat("stat_game_ticks_total", 1f64);

	}

	/// Unlocks unlock.
	pub fn unlock(&mut self, name: &str) {

		if let Some(unlock) = self.unlock_storage.get_mut(name) {

			unlock.unlock();
			
			for u in unlock.get_asset().unlocks.iter() {

				match *u {

					Unlockable::Building(name) => self.building_storage.unlock(name),
					Unlockable::Feature(name) => self.feature_storage.unlock(name),
					Unlockable::Resource(name) => self.resource_storage.unlock(name),
					Unlockable::Technology(name) => self.technology_storage.unlock(name),
					Unlockable::Upgrade(name) => self.upgrade_storage.unlock(name),
					_ => ()

				}

			}

		}

	}

}