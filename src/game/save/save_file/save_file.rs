use serde::{ Serialize, Deserialize };
use crate::game::stuff::StuffManager;
use super::{ Options, Stuffs };

/// A save file.
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
	
	pub options: Option<Options>,
	pub stuffs: Option<Stuffs>,

}

impl SaveFile {

	/// Creates a new save file.
	pub fn new() -> Self {

		Self {

			options: Some(Options::new()),
			stuffs: Some(Stuffs::new())

		}

	}

	/// Clears the save file.
	pub fn clear(&mut self) {

		self.options.get_or_insert(Options::new()).clear();
		self.stuffs.get_or_insert(Stuffs::new()).clear();

	}

	/// Loads save file data to managers.
	pub fn load(&mut self, stuff_manager: &mut StuffManager) {

		self.options.get_or_insert(Options::new()).load();
		self.stuffs.get_or_insert(Stuffs::new()).load(stuff_manager);

	}

	/// Fills save file.
	pub fn save(&mut self, stuff_manager: &StuffManager) {

		self.options.get_or_insert(Options::new()).save();
		self.stuffs.get_or_insert(Stuffs::new()).save(stuff_manager);

	}

}
