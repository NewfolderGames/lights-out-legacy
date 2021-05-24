use std::collections::{ HashMap, hash_map::Iter };
use super::*;

pub struct StuffManager {

	building_manager: BuildingManager,
	modifier_manager: ModifierManager,
	resource_manager: ResourceManager,
	stat_manager: StatManager,
	technology_manager: TechnologyManager,
	text_manager: TextManager,
	unlock_manager: UnlockManager,
	upgrade_manager: UpgradeManager,
	feature_manager: FeatureManager,

}

impl StuffManager {

	pub fn new() -> Self {

		Self {

			building_manager: BuildingManager::new(),
			modifier_manager: ModifierManager::new(),
			resource_manager: ResourceManager::new(),
			stat_manager: StatManager::new(),
			technology_manager: TechnologyManager::new(),
			text_manager: TextManager::new(),
			unlock_manager: UnlockManager::new(),
			upgrade_manager: UpgradeManager::new(),
			feature_manager: FeatureManager::new(),
			
		}

	}

	pub fn add_resource(&mut self, name: &str, amount: f64) {

		self.resource_manager.add(name, amount);

	}

	pub fn add_stat(&mut self, name: &str, amount: f64) {

		self.stat_manager.add(name, amount);

	}

	pub fn get_resource(&self, name: &str) -> Option<&Resource> {

		self.resource_manager.get(name)

	}

	pub fn get_stat(&self, name: &str) -> Option<&Stat> {

		self.stat_manager.get(name)

	}

	pub fn get_text(&self, name: &str) -> Option<&str> {

		self.text_manager.get(name)

	}

	pub fn iter_building(&self) -> Iter<String, Building> {

		self.building_manager.iter()

	}

	pub fn iter_modifier(&self) -> Iter<String, Modifier> {

		self.modifier_manager.iter()

	}

	pub fn iter_resource(&self) -> Iter<String, Resource> {

		self.resource_manager.iter()

	}

	pub fn iter_stat(&self) -> Iter<String, Stat> {

		self.stat_manager.iter()

	}

	pub fn iter_technology(&self) -> Iter<String, Technology> {

		self.technology_manager.iter()

	}

	pub fn iter_upgrades(&self) -> Iter<String, Upgrade> {

		self.upgrade_manager.iter()

	}

	pub fn iter_unlock(&self) -> Iter<String, Unlock> {

		self.unlock_manager.iter()

	}

	pub fn is_feature_unlocked(&self, name: &str) -> bool {

		self.feature_manager
			.get(name)
			.map_or(false, |f| f.is_unlocked())

	}

	pub fn is_unlocked(&self, name: &str) -> bool {

		self.unlock_manager.is_unlocked(name)

	}

	pub fn load_building(&mut self, asset: BuildingAsset) {

		self.building_manager.load(asset);

	}

	pub fn load_feature(&mut self, asset: FeatureAsset) {

		self.feature_manager.load(asset);

	}

	pub fn load_modifier(&mut self, asset: ModifierAsset) {

		self.modifier_manager.load(asset);

	}

	pub fn load_stat(&mut self, asset: StatAsset) {

		self.stat_manager.load(asset);

	}

	pub fn load_resource(&mut self, asset: ResourceAsset) {

		self.resource_manager.load(asset);

	}

	pub fn load_technology(&mut self, asset: TechnologyAsset) {

		self.technology_manager.load(asset);

	}

	pub fn load_text(&mut self, asset: TextAsset) {

		self.text_manager.load(asset);

	}

	pub fn load_unlock(&mut self, asset: UnlockAsset) {

		self.unlock_manager.load(asset);

	}

	pub fn load_upgrade(&mut self, asset: UpgradeAsset) {

		self.upgrade_manager.load(asset)

	}

	pub fn research(&mut self, name: &str) {

		self.technology_manager.research(name);

	}

	pub fn set_building(&mut self, name: &str, amount: i32) {

		self.building_manager.set_count(name, amount);

	}

	pub fn set_resource(&mut self, name: &str, amount: f64) {

		self.resource_manager.set_count(name, amount);

	}

	pub fn set_stat(&mut self, name: &str, amount: f64) {

		self.stat_manager.set_value(name, amount);

	}

	pub fn tick(&mut self) {

		// Calculate modifiers.

		let mut modifiers: HashMap<String, f64> = HashMap::new();

		if self.upgrade_manager.is_dirty() {

			self.upgrade_manager.calculate();
			self.upgrade_manager.clear_dirty();
		
		}

		if self.building_manager.is_dirty() {
			
			self.building_manager.calculate(&self.modifier_manager);
			self.building_manager.clear_dirty();
		
		}

		for (name, value) in self.upgrade_manager.get_modifiers().iter() {

			modifiers.insert(String::from(name), *value);

		}

		for (name, value) in self.building_manager.get_modifiers().iter() {

			if let Some(modifier) = modifiers.get_mut(name) {

				*modifier += value;

			} else {

				modifiers.insert(String::from(name), *value);

			}

		}

		// Calculate resource.

		self.resource_manager.calculate(&self.modifier_manager);

		// Stat.

		self.add_stat("stat_game_ticks_current", 1f64);
		self.add_stat("stat_game_ticks_total", 1f64);

	}

	pub fn unlock(&mut self, name: &str) {

		if let Some(unlock) = self.unlock_manager.get_mut(name) {

			unlock.unlock();
			
			for u in unlock.get_asset().unlocks.iter() {

				match *u {

					Unlockable::Building(name) => self.building_manager.unlock(name),
					Unlockable::Feature(name) => self.feature_manager.unlock(name),
					Unlockable::Resource(name) => self.resource_manager.unlock(name),
					Unlockable::Technology(name) => self.technology_manager.unlock(name),
					Unlockable::Upgrade(name) => self.upgrade_manager.unlock(name),
					_ => ()

				}

			}

		}

	}

	pub fn upgrade(&mut self, name: &str) {

		self.upgrade_manager.upgrade(name);

	}

}
