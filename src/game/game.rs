use wasm_bindgen::prelude::*;
use crate::assets::*;
use super::asset::AssetManager;
use super::rendering::RenderingManager;
use super::stuff::StuffManager;

#[wasm_bindgen]
pub struct Game {

	asset_manager: AssetManager,
	rendering_manager: RenderingManager,
	stuff_manager: StuffManager,

	is_loaded: bool,
	is_playing: bool,

}

// Exposed functions.

#[wasm_bindgen]
impl Game {

	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {

		Self {

			asset_manager: AssetManager::new(),
			rendering_manager: RenderingManager::new(),
			stuff_manager: StuffManager::new(),
			is_loaded: false,
			is_playing: false,

		}

	}

	#[wasm_bindgen]
	pub fn load_assets(&mut self) {

		if self.is_playing || self.is_loaded { return }

		// Asset loading.

		load_building(&mut self.asset_manager);
		load_modifier(&mut self.asset_manager);
		load_resource(&mut self.asset_manager);
		load_unlock(&mut self.asset_manager);

		// Add stuff.

		for (_, building) in self.asset_manager.iter_buildings() { self.stuff_manager.load_building(building.clone()) }
		for (_, modifier) in self.asset_manager.iter_modifiers() { self.stuff_manager.load_modifier(modifier.clone()) }
		for (_, resource) in self.asset_manager.iter_resources() { self.stuff_manager.load_resource(resource.clone()) }

		// Done

		self.is_loaded = true;

	}

	#[wasm_bindgen]
	pub fn load_save(&mut self) {

	}

	#[wasm_bindgen]
	pub fn pause(&mut self) {

		self.is_playing = false;

	}

	#[wasm_bindgen]
	pub fn resume(&mut self) {

		self.is_playing = true;

	}

	#[wasm_bindgen]
	pub fn tick(&mut self) {

		if !self.is_playing { return }

	}

}

impl Game {

	pub fn get_asset_manager(&self) -> &AssetManager {

		&self.asset_manager

	}

	pub fn get_asset_manager_mut(&mut self) -> &mut AssetManager {

		&mut self.asset_manager

	}

	pub fn get_stuff_manager(&self) -> &StuffManager {

		&self.stuff_manager

	}

	pub fn get_stuff_manager_mut(&mut self) -> &mut StuffManager {

		&mut self.stuff_manager

	}

}

// Debug.

impl Game {


}