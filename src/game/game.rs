use std::rc::Rc;
use wasm_bindgen::prelude::*;
use super::stuff::{ StuffManager, unlock::Unlockable };
use super::rendering::RenderingManager;
use super::save::SaveManager;

#[wasm_bindgen]
pub struct Game {

	// Managers.

	rendering_manager: RenderingManager,
	save_manager: SaveManager,
	stuff_manager: StuffManager,

	// State.

	is_playing: bool,
	
}

#[wasm_bindgen]
impl Game {

	/// Creates a new game.
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {

		// Panic hook.

		std::panic::set_hook(Box::new(console_error_panic_hook::hook));

		// Web stuffs.

		let web_window = Rc::new(web_sys::window().expect("Window not found."));
		let web_document = Rc::new(web_window.document().expect("Document not found."));

		// Stuff Manager.

		let mut stuff_manager = StuffManager::new();

		crate::assets::load_building(&mut stuff_manager);
		crate::assets::load_feature(&mut stuff_manager);
		crate::assets::load_modifier(&mut stuff_manager);
		crate::assets::load_resource(&mut stuff_manager);
		crate::assets::load_stat(&mut stuff_manager);
		crate::assets::load_text(&mut stuff_manager);
		crate::assets::load_technology(&mut stuff_manager);
		crate::assets::load_unlock(&mut stuff_manager);
		crate::assets::load_upgrade(&mut stuff_manager);

		// Save manager.

		let save_manager = SaveManager::new(web_window.clone());
		save_manager.load_from_storage(&mut stuff_manager);

		// Tick.

		stuff_manager.unlock("default");
		stuff_manager.add_stat("game_booted", 1f64);
		stuff_manager.tick();

		// Rendering Manager.

		let rendering_manager = RenderingManager::new(web_document.clone(), &stuff_manager);
		rendering_manager.set_loading(false);


		Self {

			rendering_manager,
			save_manager,
			stuff_manager,
			is_playing: true,

		}

	}

}

#[wasm_bindgen]
impl Game {

	#[wasm_bindgen]
	pub fn load_save_from_string(&mut self, save: &str) {

		self.rendering_manager.set_loading(true);
		self.rendering_manager.set_loading_description("Loading save file");

		self.is_playing = false;

		// Load save.

		self.stuff_manager.reset();
		self.save_manager.load_from_string(save, &mut self.stuff_manager);

		// Done.

		self.is_playing = true;

		self.rendering_manager.set_loading(false);
		self.rendering_manager.set_loading_description("");
		self.rendering_manager.render(&self.stuff_manager);

		self.rendering_manager.push_log(self.stuff_manager.get_text_string("log_game_save_loaded").unwrap_or("LOG_GAME_SAVE_LOADED"),"log-game");

	}

	#[wasm_bindgen]
	pub fn pause(&mut self) {

		self.is_playing = false;
		self.rendering_manager.push_log(self.stuff_manager.get_text_string("log_game_pause").unwrap_or("LOG_GAME_PAUSE"), "log-game");

	}

	#[wasm_bindgen]
	pub fn resume(&mut self) {

		self.is_playing = true;
		self.rendering_manager.push_log(self.stuff_manager.get_text_string("log_game_resume").unwrap_or("LOG_GAME_RESUME"), "log-game");

	}

	#[wasm_bindgen]
	pub fn save(&mut self) {

		self.save_manager.save(&self.stuff_manager);
		self.rendering_manager.push_log(self.stuff_manager.get_text_string("log_game_save").unwrap_or("LOG_GAME_SAVE"), "log-game");

	}

	#[wasm_bindgen]
	pub fn tick(&mut self) {
		
		if !self.is_playing { return }

		self.stuff_manager.tick();
		self.rendering_manager.render(&self.stuff_manager);

	}

}

// Stuffs.

#[wasm_bindgen] 
impl Game {

	#[wasm_bindgen]
	pub fn purchase_building(&mut self, name: &str) {

		if self.stuff_manager.purchase_building(name) {

			self.rendering_manager.render(&self.stuff_manager);
			self.rendering_manager.push_log(&format!("Constructed a building '{}'.", self.stuff_manager.get_text_string(&format!("building_{}_title", name)).unwrap_or(&format!("BUILDING{}_TITLE", name.to_uppercase()))), "log-info");

		}

	}

	#[wasm_bindgen]
	pub fn purchase_technology(&mut self, name: &str) {

		if self.stuff_manager.purchase_technology(name) {

			self.rendering_manager.render(&self.stuff_manager);
			self.rendering_manager.push_log(&format!("Researched a technology '{}'.",self.stuff_manager.get_text_string(&format!("technology_{}_title", name)).unwrap_or(&format!("TECHNOLOGY_{}_TITLE", name.to_uppercase()))), "log-info");
			self.unlock(&format!("technology_{}", name));

		}

	}

	#[wasm_bindgen]
	pub fn purchase_upgrade(&mut self, name: &str) {

		if self.stuff_manager.purchase_upgrade(name) {

			self.rendering_manager.render(&self.stuff_manager);
			self.rendering_manager.push_log(&format!("Researched a upgrade '{}'.",self.stuff_manager.get_text_string(&format!("upgrade_{}_title", name)).unwrap_or(&format!("UPGRADE_{}_TITLE", name.to_uppercase()))), "log-info");

		}

	}

	#[wasm_bindgen]
	pub fn toggle_building(&mut self, name: &str) {

		self.stuff_manager.toggle_building(name);
		self.rendering_manager.render(&self.stuff_manager);

	}

	pub fn unlock(&mut self, name: &str) {

		self.stuff_manager.unlock(name);
		self.stuff_manager
			.get_unlock(name)
			.map(|u| {

				u.get_unlocks()
				 .iter()
				 .for_each(|u| {

					let log = match *u {

						Unlockable::Building(name) => format!("Unlocked a new buliding '{}'.", self.stuff_manager.get_text_string(&format!("building_{}_title", name)).unwrap_or(&name.to_uppercase())),
						Unlockable::Feature(name) => format!("Unlocked a new feature '{}'.", self.stuff_manager.get_text_string(&format!("feature_{}", name)).unwrap_or(&name.to_uppercase())),
						Unlockable::Resource(name) => format!("Unlocked a new resource '{}'.", self.stuff_manager.get_text_string(&format!("resource_{}", name)).unwrap_or(&name.to_uppercase())),
						Unlockable::Technology(name) => format!("Unlocked a new technology '{}'.", self.stuff_manager.get_text_string(&format!("technology_{}_title", name)).unwrap_or(&name.to_uppercase())),
						Unlockable::Upgrade(name) => format!("Unlocked a new upgrade '{}'.", self.stuff_manager.get_text_string(&format!("upgrade_{}_title", name)).unwrap_or(&name.to_uppercase())),
						_ => format!("Unlocked a new thing '{}'.", &name.to_uppercase())
						
					};
					self.rendering_manager.push_log(&log, "log-warn");

				 });

			});

	}

}

// UI.

#[wasm_bindgen] 
impl Game {

	#[wasm_bindgen]
	pub fn ui_change_tab(&mut self, name: &str) {

		self.rendering_manager.change_tab(name, &self.stuff_manager);

	}

}

// Special.

#[wasm_bindgen] 
impl Game {

	#[wasm_bindgen]
	pub fn lighthouse_examine(&mut self) {

		self.stuff_manager.add_stat("lighthouse_examined", 1f64);

		if !self.stuff_manager.is_unlocked("quest_exmaine") {

			if self.stuff_manager.get_stat("lighthouse_examined").map_or(0f64, |s| s.get_value()) >= 5f64 {

				self.unlock("quest_exmaine");
				self.rendering_manager.push_log(self.stuff_manager.get_text_string("log_tab_lighthouse_examine").unwrap_or("LOG_TAB_LIGHTHOUSE_EXAMINE"), "log-general");

			}
			
		} else if self.stuff_manager.is_unlocked("quest_gather") {
		
			self.stuff_manager.add_resource("science", 1f64 + self.stuff_manager.get_modifier_value("lighthouse_examine_base").unwrap_or(0f64));

			if self.stuff_manager.is_upgrade_researched("lighthouse_examine") {

				self.stuff_manager.add_resource("knowledge", self.stuff_manager.get_modifier_value("lighthouse_examine_base").unwrap_or(0f64));

			}

		}

	}

	pub fn lighthouse_gather(&mut self) {

		self.stuff_manager.add_stat("lighthouse_gathered", 1f64);
		
		self.stuff_manager.add_resource("stone", 1f64 + self.stuff_manager.get_modifier_value("lighthouse_gather_base").unwrap_or(0f64));
		self.stuff_manager.add_resource("wood", 1f64 + self.stuff_manager.get_modifier_value("lighthouse_gather_base").unwrap_or(0f64));
		
		// Complete quest.

		let gathered = self.stuff_manager.get_stat("lighthouse_gathered").map_or(0f64, |s| s.get_value());

		if !self.stuff_manager.is_unlocked("quest_gather") && gathered >= 10f64 {

			self.unlock("quest_gather");
			self.rendering_manager.push_log(self.stuff_manager.get_text_string("log_tab_lighthouse_gather").unwrap_or("LOG_TAB_LIGHTHOUSE_GATHER"), "log-general");
			
		}
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
		self.rendering_manager.push_log("DEBUG_ADD_RESOURCE", "log-debug");

	}

	#[wasm_bindgen]
	pub fn debug_set_resource(&mut self, name: &str, amount: f64) {

		self.stuff_manager.set_resource(name, amount);
		self.stuff_manager.add_stat("stat_debug", 1f64);
		self.rendering_manager.render(&self.stuff_manager);
		self.rendering_manager.push_log("DEBUG_SET_RESOURCE", "log-debug");

	}

	#[wasm_bindgen]
	pub fn debug_unlock(&mut self, name: &str) {

		self.stuff_manager.unlock(name);
		self.stuff_manager.add_stat("stat_debug", 1f64);
		self.rendering_manager.render(&self.stuff_manager);
		self.rendering_manager.push_log("DEBUG_UNLOCK", "log-debug");

	}

}