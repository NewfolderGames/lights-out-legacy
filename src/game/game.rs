use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{ Document, Window };
use super::stuff::{ Stuff, StuffManager, Unlockable };
use super::rendering::RenderingManager;
use super::save::SaveManager;

#[wasm_bindgen]
pub struct Game {

	// Web stuffs.

	web_window: Rc<Window>,
	web_document: Rc<Document>,

	// Managers.

	rendering_manager: RenderingManager,
	save_manager: SaveManager,
	stuff_manager: StuffManager,

	// State.

	is_loaded: bool,
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

		// Managers.

		let rendering_manager = RenderingManager::new(web_window.clone(), web_document.clone());
		let save_manager = SaveManager::new(web_window.clone());
		let stuff_manager = StuffManager::new();

		Self {

			web_window,
			web_document,
			rendering_manager,
			save_manager,
			stuff_manager,
			is_loaded: false,
			is_playing: false,

		}

	}

}

#[wasm_bindgen]
impl Game {

	#[wasm_bindgen]
	pub fn load(&mut self) {

		if self.is_loaded { return }

		self.rendering_manager.set_loading(true);

		// Asset loading.

		self.rendering_manager.set_loading_description("Loading assets.");

		crate::assets::load_building(&mut self.stuff_manager);
		crate::assets::load_feature(&mut self.stuff_manager);
		crate::assets::load_modifier(&mut self.stuff_manager);
		crate::assets::load_resource(&mut self.stuff_manager);
		crate::assets::load_stat(&mut self.stuff_manager);
		crate::assets::load_text(&mut self.stuff_manager);
		crate::assets::load_technology(&mut self.stuff_manager);
		crate::assets::load_unlock(&mut self.stuff_manager);
		crate::assets::load_upgrade(&mut self.stuff_manager);

		// Load save.

		self.rendering_manager.set_loading_description("Loading save file.");

		self.save_manager.load_from_storage(&mut self.stuff_manager);

		self.stuff_manager.unlock("unlock_default");
		self.stuff_manager.add_stat("stat_game_booted", 1f64);
		self.stuff_manager.tick();

		// Rendering.

		self.rendering_manager.set_loading_description("Initializing rendering manager.");

		self.rendering_manager.init(&self.stuff_manager);

		// Done.

		self.is_loaded = true;
		self.is_playing = true;

		self.rendering_manager.set_loading(false);
		self.rendering_manager.set_loading_description("");
		self.rendering_manager.render(&self.stuff_manager);

	}

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

		self.rendering_manager.push_log(self.stuff_manager.get_text("log_game_save_loaded").unwrap_or("LOG_GAME_SAVE_LOADED"), Some("log-game"));

	}

	#[wasm_bindgen]
	pub fn pause(&mut self) {

		self.is_playing = false;
		self.rendering_manager.push_log(self.stuff_manager.get_text("log_game_pause").unwrap_or("LOG_GAME_PAUSE"), Some("log-game"));

	}

	#[wasm_bindgen]
	pub fn resume(&mut self) {

		self.is_playing = true;
		self.rendering_manager.push_log(self.stuff_manager.get_text("log_game_resume").unwrap_or("LOG_GAME_RESUME"), Some("log-game"));

	}

	#[wasm_bindgen]
	pub fn save(&mut self) {

		self.save_manager.save(&self.stuff_manager);
		self.rendering_manager.push_log(self.stuff_manager.get_text("log_game_save").unwrap_or("LOG_GAME_SAVE"), Some("log-game"));

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
			self.rendering_manager.push_log(&format!("Constructed a building '{}'.", self.stuff_manager.get_text(&format!("{}_title", name)).unwrap_or(&name.to_uppercase())), Some("log-info"));

		}

	}

	#[wasm_bindgen]
	pub fn purchase_technology(&mut self, name: &str) {

		if self.stuff_manager.purchase_technology(name) {

			self.rendering_manager.render(&self.stuff_manager);
			self.rendering_manager.push_log(&format!("Researched a technology '{}'.", self.stuff_manager.get_text(&format!("{}_title", name)).unwrap_or(&name.to_uppercase())), Some("log-info"));
			self.unlock(&format!("unlock_{}", name));

		}

	}

	#[wasm_bindgen]
	pub fn purchase_upgrade(&mut self, name: &str) {

		if self.stuff_manager.purchase_upgrade(name) {

			self.rendering_manager.render(&self.stuff_manager);
			self.rendering_manager.push_log(&format!("Researched a upgrade '{}'.", self.stuff_manager.get_text(&format!("{}_title", name)).unwrap_or(&name.to_uppercase())), Some("log-info"));

		}

	}

	#[wasm_bindgen]
	pub fn toggle_building(&mut self, name: &str) {

		self.stuff_manager.toggle_building(name);
		self.rendering_manager.render(&self.stuff_manager);

	}

	fn unlock(&mut self, name: &str) {

		self.stuff_manager.unlock(name);
		self.stuff_manager
			.get_unlock(name)
			.map(|u| {

				u.get_asset()
				 .unlocks
				 .iter()
				 .for_each(|u| {

					let log = match *u {

						Unlockable::Building(name) => format!("Unlocked a new buliding '{}'.", self.stuff_manager.get_text(&format!("{}_title", name)).unwrap_or(&name.to_uppercase())),
						Unlockable::Feature(name) => format!("Unlocked a new feature '{}'.", self.stuff_manager.get_text(name).unwrap_or(&name.to_uppercase())),
						Unlockable::Resource(name) => format!("Unlocked a new resource '{}'.", self.stuff_manager.get_text(name).unwrap_or(&name.to_uppercase())),
						Unlockable::Technology(name) => format!("Unlocked a new technology '{}'.", self.stuff_manager.get_text(&format!("{}_title", name)).unwrap_or(&name.to_uppercase())),
						Unlockable::Upgrade(name) => format!("Unlocked a new upgrade '{}'.", self.stuff_manager.get_text(&format!("{}_title", name)).unwrap_or(&name.to_uppercase())),
						_ => format!("Unlocked a new thing '{}'.", &name.to_uppercase())
						
					};
					self.rendering_manager.push_log(&log, Some("log-warn"));

				 });

			});

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

			if self.stuff_manager.get_stat("stat_lighthouse_examined").unwrap().get_value() >= 5f64 {

				self.unlock("unlock_quest_exmaine");
				self.rendering_manager.push_log(self.stuff_manager.get_text("log_tab_lighthouse_examine").unwrap_or("LOG_TAB_LIGHTHOUSE_EXAMINE_1"), None);

			}
			
		} else {
		
			if self.stuff_manager.is_unlocked("unlock_quest_gather") {

				self.stuff_manager.add_resource("resource_science", 1f64 + self.stuff_manager.get_modifier_value("modifier_lighthouse_examine_base").unwrap_or(0f64));

				if self.stuff_manager.is_upgrade_researched("upgrade_lighthouse_examine") {

					self.stuff_manager.add_resource("resource_knowledge", self.stuff_manager.get_modifier_value("modifier_lighthouse_examine_base").unwrap_or(0f64));

				}

			}

		}

	}

	#[wasm_bindgen]
	pub fn lighthouse_gather(&mut self) {

		self.stuff_manager.add_stat("stat_lighthouse_gathered", 1f64);
		let gathered = self.stuff_manager.get_stat("stat_lighthouse_gathered").unwrap().get_value();

		self.stuff_manager.add_resource("resource_stone", 1f64 + self.stuff_manager.get_modifier_value("modifier_lighthouse_gather_base").unwrap_or(0f64));
		self.stuff_manager.add_resource("resource_wood", 1f64 + self.stuff_manager.get_modifier_value("modifier_lighthouse_gather_base").unwrap_or(0f64));

		if !self.stuff_manager.is_unlocked("unlock_quest_gather") && gathered >= 10f64 {

			self.unlock("unlock_quest_gather");
			self.rendering_manager.push_log(self.stuff_manager.get_text("log_tab_lighthouse_gather").unwrap_or("LOG_TAB_LIGHTHOUSE_GATHER"), None)
			
		}
	}

	#[wasm_bindgen]
	pub fn lighthouse_lightsout(&mut self) {


	}

	#[wasm_bindgen]
	pub fn lighthouse_search(&mut self) {


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
		self.rendering_manager.push_log("DEBUG_ADD_RESOURCE", Some("log-debug"));

	}

	#[wasm_bindgen]
	pub fn debug_set_resource(&mut self, name: &str, amount: f64) {

		self.stuff_manager.set_resource(name, amount);
		self.stuff_manager.add_stat("stat_debug", 1f64);
		self.rendering_manager.render(&self.stuff_manager);
		self.rendering_manager.push_log("DEBUG_SET_RESOURCE", Some("log-debug"));

	}

	#[wasm_bindgen]
	pub fn debug_unlock(&mut self, name: &str) {

		self.stuff_manager.unlock(name);
		self.stuff_manager.add_stat("stat_debug", 1f64);
		self.rendering_manager.render(&self.stuff_manager);
		self.rendering_manager.push_log("DEBUG_UNLOCK", Some("log-debug"));

	}

}