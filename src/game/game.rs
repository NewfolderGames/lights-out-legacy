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

}

// Game state.

#[wasm_bindgen]
impl Game {

	#[wasm_bindgen]
	pub fn load(&mut self) {

		if self.is_playing || self.is_loaded { return }

		self.rendering_manager.set_loading(true);

		// Asset loading.

		self.rendering_manager.set_loading_description("Loading assets.");

		load_building(&mut self.stuff_manager);
		load_feature(&mut self.stuff_manager);
		load_modifier(&mut self.stuff_manager);
		load_resource(&mut self.stuff_manager);
		load_stat(&mut self.stuff_manager);
		load_text(&mut self.stuff_manager);
		load_technology(&mut self.stuff_manager);
		load_unlock(&mut self.stuff_manager);
		load_upgrade(&mut self.stuff_manager);

		// Rendering.

		self.rendering_manager.init(&self.stuff_manager);

		// Done.

		self.is_loaded = true;

		self.rendering_manager.set_loading(false);
		self.rendering_manager.set_loading_description("");

	}

	#[wasm_bindgen]
	pub fn load_save(&mut self) {

		if !self.is_loaded { return }

		self.rendering_manager.set_loading(false);
		self.rendering_manager.set_loading_description("Loading save file.");

		self.stuff_manager.set_unlock("unlock_default", true);
		self.stuff_manager.add_stat("stat_game_booted", 1f64);
		self.rendering_manager.render(&self.stuff_manager);

		self.rendering_manager.set_loading(false);
		self.rendering_manager.set_loading_description("");

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

		self.stuff_manager.tick();
		self.rendering_manager.render(&self.stuff_manager);

	}

}

// UI.

#[wasm_bindgen] 
impl Game {

	#[wasm_bindgen]
	pub fn change_tab(&mut self, name: &str) {

		self.rendering_manager.change_tab(name, &self.stuff_manager);

	}

}

// Special.

#[wasm_bindgen] 
impl Game {

	#[wasm_bindgen]
	pub fn lighthouse_examine(&mut self) {

		self.stuff_manager.add_stat("stat_lighthouse_examined", 1f64);

		if !self.stuff_manager.is_unlocked("unlock_quest_exmaine") {

			self.stuff_manager.set_unlock("unlock_quest_exmaine", true);
			
		} else if self.stuff_manager.is_unlocked("unlock_quest_gather") {

			self.stuff_manager.add_resource("resource_science", 1f64);

		}

	}

	pub fn lighthouse_gather(&mut self) {

		self.stuff_manager.add_stat("stat_lighthouse_gathered", 1f64);

		if !self.stuff_manager.is_unlocked("unlock_quest_gather") {

			self.stuff_manager.set_unlock("unlock_quest_gather", true);
			
		}

		self.stuff_manager.add_resource("resource_stone", 1f64);
		self.stuff_manager.add_resource("resource_wood", 1f64);

	}

	pub fn lighthouse_lightsout(&mut self) {


	}

}

// Debug.

#[wasm_bindgen]
impl Game {

	#[wasm_bindgen]
	pub fn debug_add_resource(&mut self, name: &str, amount: f64) {

		self.stuff_manager.add_resource(name, amount);
		self.stuff_manager.add_stat("stat_debug", 1f64);
		self.rendering_manager.render(&self.stuff_manager);

	}

	#[wasm_bindgen]
	pub fn debug_unlock(&mut self, name: &str) {

		self.stuff_manager.set_unlock(name, true);
		self.stuff_manager.add_stat("stat_debug", 1f64);
		self.rendering_manager.render(&self.stuff_manager);

	}

}