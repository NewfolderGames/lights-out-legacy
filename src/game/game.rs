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

	is_running: bool,
	is_loading_done: bool,

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
			is_running: false,
			is_loading_done: false,

		}

	}

	#[wasm_bindgen]
	pub fn init(&mut self) {

		self.load_assets();
		self.load_save();

		self.is_running = true;

	}

	#[wasm_bindgen]
	pub fn is_running(&self) -> bool {

		self.is_running

	}

	#[wasm_bindgen]
	pub fn is_loading_done(&self) -> bool {

		self.is_loading_done

	}

	#[wasm_bindgen]
	pub fn pause(&mut self) {

		self.is_running = false;

	}

	#[wasm_bindgen]
	pub fn resume(&mut self) {

		self.is_running = true;

	}

	#[wasm_bindgen]
	pub fn tick(&mut self) {

		if !self.is_running { return }

	}

}

// Game state.

impl Game {

	pub fn load_assets(&mut self) {

		if self.is_loading_done { return }

		// Asset loading.

		load_building(&mut self.asset_manager);
		load_modifier(&mut self.asset_manager);
		load_resource(&mut self.asset_manager);

		// Manager stuff.

		// Done

		self.is_loading_done = true;

	}

	pub fn load_save(&mut self) {



	}

}

// Debug.

impl Game {


}