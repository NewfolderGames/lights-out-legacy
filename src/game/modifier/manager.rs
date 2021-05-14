use std::collections::HashMap;
use std::rc::Rc;
use super::Modifier;
use super::ModifierData;

/// A modifier manager.
pub struct ModifierManager {

	modifiers: HashMap<String, Modifier>,

}

// Constructor.

impl ModifierManager {

	/// Creates a new modifier manager.
	pub fn new() -> Self {

		Self {

			modifiers: HashMap::new()

		}

	}

}

// Modifier handling.

impl ModifierManager {

	/// Loads modifier data into the manager.
	pub fn load(&mut self, data: ModifierData) {

		let name = String::from(data.name);
		let modifier = Modifier::new(Rc::new(data));

		self.modifiers.insert(name, modifier);

	}

}