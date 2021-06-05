use std::collections::hash_map::{ HashMap, Iter };
use super::building::*;
use super::feature::*;
use super::modifier::*;
use super::resource::*;
use super::stat::*;
use super::technology::*;
use super::text::*;
use super::unlock::*;
use super::upgrade::*;

/// A stuff manager.
pub struct StuffManager {

	building_manager: BuildingManager,
	feature_manager: FeatureManager,
	modifier_manager: ModifierManager,
	resource_manager: ResourceManager,
	stat_manager: StatManager,
	technology_manager: TechnologyManager,
	text_manager: TextManager,
	unlock_manager: UnlockManager,
	upgrade_manager: UpgradeManager,

}

impl StuffManager {

	/// Creates a new stuff manager.
	pub fn new() -> Self {

		Self {

			building_manager: BuildingManager::new(),
			feature_manager: FeatureManager::new(),
			modifier_manager: ModifierManager::new(),
			resource_manager: ResourceManager::new(),
			stat_manager: StatManager::new(),
			technology_manager: TechnologyManager::new(),
			text_manager: TextManager::new(),
			unlock_manager: UnlockManager::new(),
			upgrade_manager: UpgradeManager::new(),

		}

	}

	/// Adds a building's count.
	pub fn add_building(&mut self, name: &str, amount: i32) {

		self.building_manager.add_count(name, amount);

	}

	/// Adds a resource's count.
	pub fn add_resource(&mut self, name: &str, amount: f64) {

		self.resource_manager.add_count(name, amount);

	}

	/// Adds a stat's value.
	pub fn add_stat(&mut self, name: &str, amount: f64) {

		self.stat_manager.add_value(name, amount);

	}

	/// Returns a reference to a building.
	pub fn get_building(&self, name: &str) -> Option<&Building> {
		
		self.building_manager.get(name)

	}

	/// Returns a modifier's value.
	pub fn get_modifier_value(&self, name: &str) -> Option<f64> {

		self.modifier_manager.get_value(name)

	}

	/// Returns a reference to a resource.
	pub fn get_resource(&self, name: &str) -> Option<&Resource> {
		
		self.resource_manager.get(name)

	}

	/// Returns a reference to a stat.
	pub fn get_stat(&self, name: &str) -> Option<&Stat> {
		
		self.stat_manager.get(name)

	}

	/// Returns a text string.
	pub fn get_text_string(&self, name: &str) -> Option<&str> {

		self.text_manager.get_string(name)

	}

	/// Returns a reference to a unlock.
	pub fn get_unlock(&self, name: &str) -> Option<&Unlock> {
		
		self.unlock_manager.get(name)

	}

	/// Returns `true` if the feature is unlocked.
	pub fn is_feature_unlocked(&self, name: &str) -> bool {

		self.feature_manager.is_unlocked(name)

	}

	/// Returns `true` if the technology is researched.
	pub fn is_technology_researched(&self, name: &str) -> bool {

		self.technology_manager.is_researched(name)

	}

	/// Retuns `true` if unlocked.
	pub fn is_unlocked(&self, name: &str) -> bool {

		self.unlock_manager.is_unlocked(name)

	}

	/// Returns `true` if the upgrade is researched.
	pub fn is_upgrade_researched(&self, name: &str) -> bool {

		self.upgrade_manager.is_researched(name)

	}

	/// Returns a building iterator.
	pub fn iter_building(&self) -> Iter<String, Building> {

		self.building_manager.iter()

	}

	/// Returns a modifier iterator.
	pub fn iter_modifier(&self) -> Iter<String, f64> {

		self.modifier_manager.iter()

	}

	/// Returns a resource iterator.
	pub fn iter_resource(&self) -> Iter<String, Resource> {

		self.resource_manager.iter()

	}

	/// Returns a stat iterator.
	pub fn iter_stat(&self) -> Iter<String, Stat> {

		self.stat_manager.iter()

	}

	/// Returns a technology iterator.
	pub fn iter_technology(&self) -> Iter<String, Technology> {

		self.technology_manager.iter()

	}

	/// Returns a unlock iterator.
	pub fn iter_upgrade(&self) -> Iter<String, Upgrade> {

		self.upgrade_manager.iter()

	}

	/// Returns a stat iterator.
	pub fn iter_unlock(&self) -> Iter<String, Unlock> {

		self.unlock_manager.iter()

	}

	/// Loads a building.
	pub fn load_building(&mut self, asset: BuildingAsset) {

		self.building_manager.load(asset);

	}

	/// Loads a feature.
	pub fn load_feature(&mut self, name: &str) {

		self.feature_manager.load(name);

	}

	/// Loads a modifier.
	pub fn load_modifier(&mut self, name: &str) {

		self.modifier_manager.load(name);

	}

	/// Loads a resource.
	pub fn load_resource(&mut self, asset: ResourceAsset) {

		self.resource_manager.load(asset);

	}

	/// Loads a stat.
	pub fn load_stat(&mut self, name: &str, category: &str) {

		self.stat_manager.load(name, category);

	}

	/// Loads a technology.
	pub fn load_technology(&mut self, asset: TechnologyAsset) {

		self.technology_manager.load(asset);

	}

	/// Loads a text.
	pub fn load_text_string(&mut self, name: &str, content: &'static str) {

		self.text_manager.load_string(name, content);

	}

	/// Loads a unlock.
	pub fn load_unlock(&mut self, asset: UnlockAsset) {

		self.unlock_manager.load(asset);

	}

	/// Loads a unlock.
	pub fn load_upgrade(&mut self, asset: UpgradeAsset) {

		self.upgrade_manager.load(asset);

	}

	/// Tries to purchase a building.
	pub fn purchase_building(&mut self, name: &str) -> bool {

		if let Some(building) = self.building_manager.get(name) {

			if !building.is_unlocked() { return false; }

			if self.resource_manager.spend_resources(building.get_price()) {

				self.add_building(name, 1);
				return true;

			}

		}

		false

	}

	/// Tries to purchase a technology.
	pub fn purchase_technology(&mut self, name: &str) -> bool {

		if let Some(technology) = self.technology_manager.get(name) {

			if !technology.is_unlocked() || technology.is_researched() { return false; }

			if self.resource_manager.spend_resources(technology.get_price()) {

				self.research_technology(name);
				return true;

			}

		}

		false

	}

	/// Tries to purchase a upgrade.
	pub fn purchase_upgrade(&mut self, name: &str) -> bool {

		if let Some(upgrade) = self.upgrade_manager.get(name) {

			if !upgrade.is_unlocked() || upgrade.is_researched() { return false; }

			if self.resource_manager.spend_resources(upgrade.get_price()) {

				self.research_upgrade(name);
				return true;

			}

		}

		false

	}

	/// Resets all stuffs to original state.
	pub fn reset(&mut self) {

		self.building_manager.reset();
		self.feature_manager.reset();
		self.modifier_manager.reset();
		self.resource_manager.reset();
		self.stat_manager.reset();
		self.technology_manager.reset();
		self.unlock_manager.reset();
		self.upgrade_manager.reset();

	}

	/// Researchs a technology.
	pub fn research_technology(&mut self, name: &str) {

		self.technology_manager.research(name);

	}

	/// Researchs a upgrade.
	pub fn research_upgrade(&mut self, name: &str) {

		self.upgrade_manager.research(name);

	}

	/// Sets a building's count.
	pub fn set_building(&mut self, name: &str, amount: i32) {

		self.building_manager.set_count(name, amount);

	}

	/// Sets a resource's count.
	pub fn set_resource(&mut self, name: &str, amount: f64) {

		self.resource_manager.set_count(name, amount);

	}

	/// Sets a stat's value.
	pub fn set_stat(&mut self, name: &str, value: f64) {

		self.stat_manager.set_value(name, value);

	}

	/// Game tick.
	pub fn tick(&mut self) {

		// Calculation.

		self.resource_manager.calculate(&self.modifier_manager);

		let mut modifiers: HashMap<String, f64> = HashMap::new();

		self.upgrade_manager.calculate(&mut modifiers);
		self.building_manager.calculate(&mut modifiers, &self.resource_manager);
		self.technology_manager.calculate(&modifiers);

		modifiers
			.iter()
			.for_each(|(name, value)| self.modifier_manager.set_value(name, *value));

		// Add stats.

		self.add_stat("game_ticks_current", 1f64);
		self.add_stat("game_ticks_total", 1f64);

	}

	/// Toggles a building.
	pub fn toggle_building(&mut self, name: &str) {

		self.building_manager.toggle(name);

	}

	/// Unlocks unlock.
	pub fn unlock(&mut self, name: &str) {

		if let Some(unlock) = self.unlock_manager.get_mut(name) {

			unlock.unlock();
			
			for u in unlock.get_unlocks().iter() {

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

}