use wasm_bindgen::prelude::*;
use crate::assets::*;
use super::asset::AssetManager;
use super::modifier::ModifierManager;
use super::resource::ResourceManager;

#[wasm_bindgen]
pub struct Game {

	asset_manager: AssetManager,
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
			modifier_manager: ModifierManager::new(),
			resource_manager: ResourceManager::new(),
			is_loading_done: false,

		}

	}

	#[wasm_bindgen]
	pub fn tick(&mut self) {

		

	}

}

impl Game {

	pub fn load_assets(&mut self) {

		if self.is_loading_done { return }

		// Asset loading.

		load_building(&mut self.asset_manager);
		load_modifier(&mut self.asset_manager);
		load_resource(&mut self.asset_manager);

		// Manager stuff.

		for (_, asset) in self.asset_manager.iter_modifiers() {

			self.modifier_manager.add_modifier(asset.clone());

		}

		for (_, asset) in self.asset_manager.iter_resource() {

			self.resource_manager.add_resource(asset.clone());

		}

		// Done

		self.is_loading_done = true;

	}

	pub fn load_save(&mut self) {



	}

	pub fn asset_manager(&self) -> &AssetManager {

		&self.asset_manager

	}

	pub fn modifier_manager(&self) -> &ModifierManager {

		&self.modifier_manager

	}

	pub fn resource_manager(&self) -> &ResourceManager {

		&self.resource_manager

	}

}

// Debug.

impl Game {


}