use wasm_bindgen::prelude::*;
use crate::resources::*;
use super::modifier::ModifierManager;
use super::resource::ResourceManager;
use super::building::BuildingManager;

#[wasm_bindgen]
pub struct Game {

	building_manager: BuildingManager,
	modifier_manager: ModifierManager,
	resource_manager: ResourceManager,

	is_loading_done: bool,

}

// Constructor.

#[wasm_bindgen]
impl Game {

	/// Creates a new game.
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {

		Self {

			building_manager: BuildingManager::new(),
			modifier_manager: ModifierManager::new(),
			resource_manager: ResourceManager::new(),
			is_loading_done: false,

		}

	}

}

// Game State.

impl Game {

	/// Loads resource into game.
	pub fn load_resources(&mut self) {

		if self.is_loading_done { return }

		load_building(&mut self.building_manager);
		load_modifier(&mut self.modifier_manager);
		load_resource(&mut self.resource_manager);

		self.is_loading_done = true;

	}

	/// Loads save file.
	pub fn load_save(&mut self) {



	}

}

// Managers.

impl Game {

	/// Returns the building manager.
	pub fn building_manager(&self) -> &BuildingManager {

		&self.building_manager

	}

	/// Returns the modifier manager.
	pub fn modifier_manager(&self) -> &ModifierManager {

		&self.modifier_manager

	}

	/// Returns the resource manager.
	pub fn resource_manager(&self) -> &ResourceManager {

		&self.resource_manager

	}

}

// Debug information.

impl Game {


}