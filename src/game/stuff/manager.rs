use std::collections::HashMap;
use std::rc::Rc;
use crate::game::asset::{ BuildingAsset, ModifierAsset, ResourceAsset, UnlockAsset, UnlockStuff };
use super::{ Building, Modifier, Resource };

pub struct StuffManager {

	unlocks: HashMap<String, Rc<UnlockAsset>>,

	buildings: HashMap<String, Building>,
	modifiers: HashMap<String, Modifier>,
	resources: HashMap<String, Resource>,

}

impl StuffManager {

	pub fn new() -> Self {

		Self {

			unlocks: HashMap::new(),
			buildings: HashMap::new(),
			modifiers: HashMap::new(),
			resources: HashMap::new(),
			
		}

	}

	pub fn load_building(&mut self, asset: Rc<BuildingAsset>) {

		let name = String::from(asset.name);
		let building = Building::new(asset);

		self.buildings.insert(name, building);

	}

	pub fn load_modifier(&mut self, asset: Rc<ModifierAsset>) {

		let name = String::from(asset.name);
		let modifier = Modifier::new(asset);

		self.modifiers.insert(name, modifier);

	}

	pub fn load_resource(&mut self, asset: Rc<ResourceAsset>) {

		let name = String::from(asset.name);
		let resource = Resource::new(asset);

		self.resources.insert(name, resource);

	}

	pub fn get_building(&self, name: &str) -> Option<&Building> {

		self.buildings.get(name)

	}

	pub fn get_modifier(&self, name: &str) -> Option<&Modifier> {

		self.modifiers.get(name)

	}

	pub fn get_modifier_value(&self, name: &str) -> Option<f64> {

		self.modifiers
			.get(name)
			.map(|m| m.get_value())

	}

	pub fn get_resource(&self, name: &str) -> Option<&Resource> {

		self.resources.get(name)

	}

	pub fn unlock(&mut self, unlock: Rc<UnlockAsset>) {

		for u in unlock.unlocks.iter() {

			match *u {

				UnlockStuff::Building(name) => {

					self.buildings
						.get_mut(name)
						.map(|b| b.unlock());

				}
				UnlockStuff::Resource(name) => {

					self.resources
						.get_mut(name)
						.map(|r| r.unlock());

				}

			}

		}

		self.unlocks.insert(String::from(unlock.name), unlock);

	}

}
