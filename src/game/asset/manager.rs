use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::rc::Rc;
use crate::game::building::BuildingAsset;
use crate::game::modifier::ModifierAsset;
use crate::game::resource::ResourceAsset;

pub struct AssetManager {

	buildings: HashMap<String, Rc<BuildingAsset>>,
	modifiers: HashMap<String, Rc<ModifierAsset>>,
	resources: HashMap<String, Rc<ResourceAsset>>,

}

impl AssetManager {

	pub fn new() -> Self {

		Self {

			buildings: HashMap::new(),
			modifiers: HashMap::new(),
			resources: HashMap::new(),

		}

	}

}

impl AssetManager {
	
	pub fn get_building(&self, name: &str) -> Option<Rc<BuildingAsset>> {
	
		self.buildings
			.get(name)
			.and_then(|a| Some(a.clone()))
	
	}

	pub fn get_modifier(&self, name: &str) -> Option<Rc<ModifierAsset>> {

		self.modifiers
			.get(name)
			.and_then(|a| Some(a.clone()))

	}

	pub fn get_resource(&self, name: &str) -> Option<Rc<ResourceAsset>> {

		self.resources
			.get(name)
			.and_then(|a| Some(a.clone()))

	}

	pub fn iter_buildings(&self) -> Iter<String, Rc<BuildingAsset>> {

		self.buildings.iter()

	}

	pub fn iter_modifiers(&self) -> Iter<String, Rc<ModifierAsset>> {

		self.modifiers.iter()

	}

	pub fn iter_resource(&self) -> Iter<String, Rc<ResourceAsset>> {

		self.resources.iter()

	}

}

impl AssetManager {

	pub fn load_building(&mut self, asset: BuildingAsset) {

		self.buildings.insert(String::from(asset.name), Rc::new(asset));

	}

	pub fn load_modifier(&mut self, asset: ModifierAsset) {

		self.modifiers.insert(String::from(asset.name), Rc::new(asset));

	}

	pub fn load_resource(&mut self, asset: ResourceAsset) {

		self.resources.insert(String::from(asset.name), Rc::new(asset));

	}

}
