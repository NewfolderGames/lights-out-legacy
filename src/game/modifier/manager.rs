use std::collections::HashMap;
use super::Modifier;

pub struct ModifierManager {

	modifiers: HashMap<String, Modifier>,

}

impl ModifierManager {

	pub fn new() -> Self {

		Self {

			modifiers: HashMap::new()

		}

	}

}
