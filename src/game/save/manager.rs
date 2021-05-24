use web_sys::{ Storage, Window };
use super::Save;
use super::super::StuffManager;

pub struct SaveManager {

	window: Window,
	storage: Storage,

	save: Save

}

impl SaveManager {

	pub fn new() -> Self {

		let window = web_sys::window().expect("Window not found.");
		let storage = window.local_storage().expect("Local Storage cannot be used.").expect("Local storage not found.");

		Self {

			window,
			storage,
			save: Save::new()

		}

	}

	pub fn save(&mut self, stuff_manager: &StuffManager) {

		// Set data.

		self.save.clear();
		
		self.save.set_buildings(stuff_manager);
		self.save.set_resources(stuff_manager);
		self.save.set_stats(stuff_manager);
		self.save.set_technologies(stuff_manager);
		self.save.set_unlocks(stuff_manager);
		self.save.set_upgrades(stuff_manager);

		// Serialize.

		let save = serde_json::to_string(&self.save).unwrap();
		let save = self.window.btoa(&save).unwrap();

		self.storage.set_item("save", &save).unwrap();

	}

	pub fn load_from_storage(&self, stuff_manager: &mut StuffManager) -> bool {

		if let Ok(Some(save)) = self.storage.get_item("save") {

			self.load_from_string(&save, stuff_manager)

		} else { return false }

	}

	pub fn load_from_string(&self, save: &str, stuff_manager: &mut StuffManager) -> bool {

		// Decode.

		let save = self.window.atob(save).unwrap();

		// Deserialize.

		if let Ok(save) = serde_json::from_str::<Save>(&save) {

			save.buildings.iter().for_each(|(b_name, b_count)| stuff_manager.set_building(b_name, *b_count));
			save.resources.iter().for_each(|(r_name, r_count)| stuff_manager.set_resource(r_name, *r_count));
			save.stats.iter().for_each(|(s_name, s_value)| stuff_manager.set_stat(s_name, *s_value));
			save.unlocks.iter().for_each(|u| stuff_manager.unlock(u));
			save.technologies.iter().for_each(|t| stuff_manager.research(t));
			save.upgrades.iter().for_each(|u| stuff_manager.upgrade(u));

			return true;

		} else { return false }

	}

}