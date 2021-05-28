use std::rc::Rc;
use web_sys::{ Storage, Window };
use crate::game::stuff::StuffManager;
use super::Save;

/// A save manager.
pub struct SaveManager {

	web_window: Rc<Window>,
	web_storage: Rc<Storage>,

	save: Save

}

impl SaveManager {

	pub fn new(window: Rc<Window>) -> Self {

		let web_storage = window.local_storage().expect("Local Storage cannot be used.").expect("Local storage not found.");

		Self {

			web_window: window.clone(),
			web_storage: Rc::new(web_storage),
			save: Save::new()

		}

	}

	pub fn save(&mut self, stuff_manager: &StuffManager) {

		// Set data.

		self.save.stuff.clear();
		
		self.save.stuff.set_buildings(stuff_manager);
		self.save.stuff.set_resources(stuff_manager);
		self.save.stuff.set_stats(stuff_manager);
		self.save.stuff.set_technologies(stuff_manager);
		self.save.stuff.set_unlocks(stuff_manager);
		self.save.stuff.set_upgrades(stuff_manager);

		// Serialize.

		let save = serde_json::to_string(&self.save).unwrap();
		let save = self.web_window.btoa(&save).unwrap();

		self.web_storage.set_item("save", &save).unwrap();

	}

	pub fn load_from_storage(&self, stuff_manager: &mut StuffManager) -> bool {

		if let Ok(Some(save)) = self.web_storage.get_item("save") {

			self.load_from_string(&save, stuff_manager)

		} else { return false }

	}

	pub fn load_from_string(&self, save: &str, stuff_manager: &mut StuffManager) -> bool {

		// Decode.

		let save = self.web_window.atob(save).unwrap();

		// Deserialize.

		if let Ok(save) = serde_json::from_str::<Save>(&save) {

			save.stuff.buildings.iter().for_each(|(b_name, b_count)| stuff_manager.set_building(b_name, *b_count));
			save.stuff.resources.iter().for_each(|(r_name, r_count)| stuff_manager.set_resource(r_name, *r_count));
			save.stuff.stats.iter().for_each(|(s_name, s_value)| stuff_manager.set_stat(s_name, *s_value));
			save.stuff.unlocks.iter().for_each(|u| stuff_manager.unlock(u));
			save.stuff.technologies.iter().for_each(|t| stuff_manager.research_technology(t));
			save.stuff.upgrades.iter().for_each(|u| stuff_manager.research_upgrade(u));

			return true;

		} else { return false }

	}

}