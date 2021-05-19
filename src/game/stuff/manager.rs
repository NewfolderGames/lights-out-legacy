use std::collections::hash_map::Iter;
use super::*;

pub struct StuffManager {

	building_manager: BuildingManager,
	modifier_manager: ModifierManager,
	resource_manager: ResourceManager,
	unlock_manager: UnlockManager,

}

impl StuffManager {

	pub fn new() -> Self {

		Self {

			building_manager: BuildingManager::new(),
			modifier_manager: ModifierManager::new(),
			resource_manager: ResourceManager::new(),
			unlock_manager: UnlockManager::new(),
			
		}

	}

	pub fn get_modifier_value(&self, name: &str) -> Option<f64> {

		self.modifier_manager
			.get(name)
			.map(|m| m.get_value())

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

	pub fn iter_unlock(&self) -> Iter<String, Unlock> {

		self.unlock_manager.iter()

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

	pub fn load_unlock(&mut self, asset: UnlockAsset) {

		self.unlock_manager.load(asset)

	}

}
