use wasm_bindgen::prelude::*;
use crate::assets::*;
use super::{ RenderingManager, SaveManager, StuffManager };

#[wasm_bindgen]
pub struct Game {

	rendering_manager: RenderingManager,
	save_manager: SaveManager,
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
			save_manager: SaveManager::new(),
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

		// Load Save.

		self.rendering_manager.set_loading_description("Loading save file.");

		self.save_manager.load_from_storage(&mut self.stuff_manager);

		self.stuff_manager.unlock("unlock_default");
		self.stuff_manager.add_stat("stat_game_booted", 1f64);

		// Done.

		self.is_loaded = true;

		self.rendering_manager.set_loading(false);
		self.rendering_manager.set_loading_description("");
		self.rendering_manager.render(&self.stuff_manager);

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
	pub fn save(&mut self) {

		self.save_manager.save(&self.stuff_manager);
		self.rendering_manager.push_log("Game saved.", Some("#ffffff"));

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

			self.stuff_manager.unlock("unlock_quest_exmaine");
			self.rendering_manager.push_log(self.stuff_manager.get_text("log_tab_lighthouse_examine_0").unwrap_or("LOG_TAB_LIGHTHOUSE_EXAMINE_0"), None);
			
		} else {
		
			if self.stuff_manager.is_unlocked("unlock_quest_gather") {

				self.stuff_manager.add_resource("resource_science", 1f64);
				self.rendering_manager.push_log(self.stuff_manager.get_text("log_tab_lighthouse_examine_2").unwrap_or("LOG_TAB_LIGHTHOUSE_EXAMINE_2"), None);
			
			} else {

				self.rendering_manager.push_log(self.stuff_manager.get_text("log_tab_lighthouse_examine_1").unwrap_or("LOG_TAB_LIGHTHOUSE_EXAMINE_1"), None);

			}

		}

	}

	pub fn lighthouse_gather(&mut self) {

		self.stuff_manager.add_stat("stat_lighthouse_gathered", 1f64);

		if !self.stuff_manager.is_unlocked("unlock_quest_gather") {

			self.stuff_manager.unlock("unlock_quest_gather");
			
		}

		self.stuff_manager.add_resource("resource_stone", 1f64);
		self.stuff_manager.add_resource("resource_wood", 1f64);

		self.rendering_manager.push_log(self.stuff_manager.get_text("log_tab_lighthouse_gather").unwrap_or("LOG_TAB_LIGHTHOUSE_GATHER"), None);

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
		self.rendering_manager.push_log("DEBUG_ADD_RESOURCE", Some("#00FF00"));

	}

	#[wasm_bindgen]
	pub fn debug_set_resource(&mut self, name: &str, amount: f64) {

		self.stuff_manager.set_resource(name, amount);
		self.stuff_manager.add_stat("stat_debug", 1f64);
		self.rendering_manager.render(&self.stuff_manager);
		self.rendering_manager.push_log("DEBUG_SET_RESOURCE", Some("#00FF00"));

	}

	#[wasm_bindgen]
	pub fn debug_unlock(&mut self, name: &str) {

		self.stuff_manager.unlock(name);
		self.stuff_manager.add_stat("stat_debug", 1f64);
		self.rendering_manager.render(&self.stuff_manager);
		self.rendering_manager.push_log("DEBUG_UNLOCK", Some("#00FF00"));

	}

}