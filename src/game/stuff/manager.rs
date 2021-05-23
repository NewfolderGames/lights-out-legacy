use std::collections::{ HashMap, hash_map::Iter };
use super::*;

pub struct StuffManager {

	building_manager: BuildingManager,
	modifier_manager: ModifierManager,
	resource_manager: ResourceManager,
	technology_manager: TechnologyManager,
	unlock_manager: UnlockManager,
	upgrade_manager: UpgradeManager,
	feature_manager: FeatureManager,
	text_assets: HashMap<String, TextAsset>,

}

impl StuffManager {

	pub fn new() -> Self {

		Self {

			building_manager: BuildingManager::new(),
			modifier_manager: ModifierManager::new(),
			resource_manager: ResourceManager::new(),
			technology_manager: TechnologyManager::new(),
			unlock_manager: UnlockManager::new(),
			upgrade_manager: UpgradeManager::new(),
			feature_manager: FeatureManager::new(),
			text_assets: HashMap::new(),
			
		}

	}

	pub fn add_resource(&mut self, name: &str, amount: f64) {

		self.resource_manager.add(name, amount);

	}

	pub fn get_text(&self, name: &str) -> Option<&str> {

		self.text_assets
			.get(name)
			.map(|t| t.text)

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

	pub fn iter_technology(&self) -> Iter<String, Technology> {

		self.technology_manager.iter()

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

	pub fn load_resource(&mut self, asset: ResourceAsset) {

		self.resource_manager.load(asset);

	}

	pub fn load_technology(&mut self, asset: TechnologyAsset) {

		self.technology_manager.load(asset);

	}

	pub fn load_text(&mut self, asset: TextAsset) {

		self.text_assets.insert(String::from(asset.name), asset);

	}

	pub fn load_unlock(&mut self, asset: UnlockAsset) {

		self.unlock_manager.load(asset);

	}

	pub fn load_upgrade(&mut self, asset: UpgradeAsset) {

		self.upgrade_manager.load(asset)

	}

	pub fn tick(&mut self) {

		// Calculate modifiers.

		let mut modifiers: HashMap<String, f64> = HashMap::new();

		self.upgrade_manager.calculate();
		self.building_manager.calculate(&self.modifier_manager);

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

	}

	pub fn set_unlock(&mut self, name: &str, unlock_state: bool) {

		if let Some(unlock) = self.unlock_manager.get_mut(name) {

			unlock.set_unlock(unlock_state);
			
			for u in unlock.get_asset().unlocks.iter() {

				match *u {

					Unlockable::Building(name) => self.building_manager.set_unlock(name, unlock_state),
					Unlockable::Feature(name) => self.feature_manager.set_unlock(name, unlock_state),
					Unlockable::Resource(name) => self.resource_manager.set_unlock(name, unlock_state),
					Unlockable::Technology(name) => self.technology_manager.set_unlock(name, unlock_state),
					Unlockable::Upgrade(name) => self.upgrade_manager.set_unlock(name, unlock_state),
					_ => ()

				}

			}

		}

	}

}
