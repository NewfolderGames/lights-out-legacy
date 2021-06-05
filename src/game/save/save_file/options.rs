use serde::{ Serialize, Deserialize };

/// A option save file.
#[derive(Serialize, Deserialize)]
pub struct Options {


}

impl Options {

	/// Creates a new option save file.
	pub fn new() -> Self {

		Self { }

	}

	/// Clears the save file.
	pub fn clear(&mut self) {

		

	}

	/// Loads the save data into the manager.
	pub fn load(&mut self) {

		
	}

	/// Saves data into the save file.
	pub fn save(&mut self) {


	}

}