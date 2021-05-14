use wasm_bindgen::prelude::*;
use crate::assets::*;
use super::asset::AssetManager;
use super::modifier::ModifierManager;
use super::resource::ResourceManager;
use super::building::BuildingManager;

#[wasm_bindgen]
pub struct Game {

	asset_manager: AssetManager,
	building_manager: BuildingManager,
	modifier_manager: ModifierManager,
	resource_manager: ResourceManager,

	is_loading_done: bool,

}

// Constructor.

#[wasm_bindgen]
impl Game {

	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {

		Self {

			asset_manager: AssetManager::new(),
			building_manager: BuildingManager::new(),
			modifier_manager: ModifierManager::new(),
			resource_manager: ResourceManager::new(),
			is_loading_done: false,

		}

	}

}

// Game State.

impl Game {

	pub fn load_assets(&mut self) {

		if self.is_loading_done { return }

		load_building(&mut self.asset_manager);
		load_modifier(&mut self.asset_manager);
		load_resource(&mut self.asset_manager);

		self.is_loading_done = true;

	}

	pub fn load_save(&mut self) {



	}

}

// Managers.

impl Game {

	pub fn asset_manager(&self) -> &AssetManager {

		&self.asset_manager

	}

	pub fn building_manager(&self) -> &BuildingManager {

		&self.building_manager

	}

	pub fn modifier_manager(&self) -> &ModifierManager {

		&self.modifier_manager

	}

	pub fn resource_manager(&self) -> &ResourceManager {

		&self.resource_manager

	}

}

// Debug information.

impl Game {


}