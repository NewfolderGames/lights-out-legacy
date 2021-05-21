use std::collections::{ HashMap, hash_map::Iter };
use super::*;

pub struct StuffManager {

	building_manager: BuildingManager,
	modifier_manager: ModifierManager,
	resource_manager: ResourceManager,
	technology_manager: TechnologyManager,
	unlock_manager: UnlockManager,
	upgrade_manager: UpgradeManager,

	textAssets: HashMap<&'static str, TextAsset>,

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
			textAssets: HashMap::new(),
			
		}

	}

	pub fn get_text(&self, name: &str) -> Option<&str> {

		self.textAssets
			.get(name)
			.map(|t| t.text)

	}

	pub fn iter_building(&self) -> Iter<&'static str, Building> {

		self.building_manager.iter()

	}

	pub fn iter_modifier(&self) -> Iter<&'static str, Modifier> {

		self.modifier_manager.iter()

	}

	pub fn iter_resource(&self) -> Iter<&'static str, Resource> {

		self.resource_manager.iter()

	}

	pub fn iter_technology(&self) -> Iter<&'static str, Technology> {

		self.technology_manager.iter()

	}

	pub fn iter_unlock(&self) -> Iter<&'static str, Unlock> {

		self.unlock_manager.iter()

	}

	pub fn is_unlocked(&self, name: &str) -> bool {

		self.unlock_manager.is_unlocked(name)

	}

	pub fn load_building(&mut self, asset: BuildingAsset) {

		self.building_manager.load(asset);

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

		self.textAssets.insert(asset.name, asset);

	}

	pub fn load_unlock(&mut self, asset: UnlockAsset) {

		self.unlock_manager.load(asset);

	}

	pub fn load_upgrade(&mut self, asset: UpgradeAsset) {

		self.upgrade_manager.load(asset)

	}

	pub fn tick(&mut self) {

		// Calculate modifiers.

		let mut modifiers = HashMap::new();

		self.upgrade_manager.calculate();
		self.building_manager.calculate(&self.modifier_manager);

		for (name, value) in self.upgrade_manager.get_modifiers().iter() {

			modifiers.insert(name, *value);

		}

		for (name, value) in self.building_manager.get_modifiers().iter() {

			if let Some(modifier) = modifiers.get_mut(name) {

				*modifier += value;

			} else {

				modifiers.insert(name, *value);

			}

		}

		// Calculate resource.

		self.resource_manager.calculate(&self.modifier_manager);

	}

	pub fn unlock(&mut self, name: &str) {

		if let Some(unlock) = self.unlock_manager.get_mut(name) {

			unlock.unlock();
			
			for u in unlock.get_asset().unlocks.iter() {

				match *u {

					Unlockable::Building(name) => self.building_manager.unlock(name),
					Unlockable::Resource(name) => self.resource_manager.unlock(name),
					_ => ()

				}

			}

		}

	}

}
