use std::collections::hash_map::Iter;
use super::*;

pub struct StuffManager {

	building_manager: BuildingManager,
	modifier_manager: ModifierManager,
	resource_manager: ResourceManager,
	technology_manager: TechnologyManager,
	unlock_manager: UnlockManager,

}

impl StuffManager {

	pub fn new() -> Self {

		Self {

			building_manager: BuildingManager::new(),
			modifier_manager: ModifierManager::new(),
			resource_manager: ResourceManager::new(),
			technology_manager: TechnologyManager::new(),
			unlock_manager: UnlockManager::new(),
			
		}

	}

	pub fn get_modifier_value(&self, name: &str) -> Option<f64> {

		self.modifier_manager
			.get(name)
			.map(|m| m.get_value())

	}

	pub fn iter_building(&self) -> Iter<&'static str, Building> {

		self.building_manager.iter()

	}

	pub fn iter_modifier(&self) -> Iter<&'static str, Modifier> {

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

	pub fn is_unlocked(&self, name: &str) -> bool {

		self.unlock_manager.is_unlocked(name)

	}

	pub fn load_building(&mut self, asset: BuildingAsset) {

		self.building_manager.load(asset)

	}

	pub fn load_modifier(&mut self, asset: ModifierAsset) {

		self.modifier_manager.load(asset)

	}

	pub fn load_resource(&mut self, asset: ResourceAsset) {

		self.resource_manager.load(asset)

	}

	pub fn load_technology(&mut self, asset: TechnologyAsset) {

		self.technology_manager.load(asset)

	}

	pub fn load_unlock(&mut self, asset: UnlockAsset) {

		self.unlock_manager.load(asset)

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
