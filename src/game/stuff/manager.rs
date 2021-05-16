use std::collections::HashMap;
use super::{ Modifier, Resource };

pub struct StuffManager {

	modifiers: HashMap<String, Modifier>,
	resources: HashMap<String, Resource>,

}

impl StuffManager {

	pub fn new() -> Self {

		Self {

			modifiers: HashMap::new(),
			resources: HashMap::new(),

		}

	}

}