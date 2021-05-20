use wasm_bindgen::prelude::*;
use crate::assets::*;
use super::rendering::RenderingManager;
use super::stuff::StuffManager;

#[wasm_bindgen]
pub struct Game {

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

		load_building(&mut self.stuff_manager);
		load_modifier(&mut self.stuff_manager);
		load_resource(&mut self.stuff_manager);
		load_technology(&mut self.stuff_manager);
		load_unlock(&mut self.stuff_manager);

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

// Debug.

impl Game {


}