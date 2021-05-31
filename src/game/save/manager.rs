use std::rc::Rc;
use web_sys::{ Storage, Window };
use crate::game::stuff::StuffManager;
use super::*;

/// A save manager.
pub struct SaveManager {

	web_window: Rc<Window>,
	web_storage: Rc<Storage>,

	save_file: SaveFile

}

impl SaveManager {

	pub fn new(window: Rc<Window>) -> Self {

		let web_storage = window.local_storage().expect("Local Storage cannot be used.").expect("Local storage not found.");

		Self {

			web_window: window.clone(),
			web_storage: Rc::new(web_storage),
			save_file: SaveFile::new()

		}

	}

	pub fn save(&mut self, stuff_manager: &StuffManager) {

		// Set data.

		self.save_file.clear();
		self.save_file.save(stuff_manager);

		// Serialize.

		let save_file = serde_json::to_string(&self.save_file).unwrap();
		let save_file = self.web_window.btoa(&save_file).unwrap();

		self.web_storage.set_item("save", &save_file).unwrap();

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

		if let Ok(mut save) = serde_json::from_str::<SaveFile>(&save) {

			save.load(stuff_manager);
			return true;

		} else { return false }

	}

}