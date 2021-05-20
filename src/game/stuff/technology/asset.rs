use std::collections::HashMap;
use crate::game::stuff::StuffManager;

pub struct TechnologyAsset {

	pub name: &'static str,

	pub price: Box<dyn Fn(&StuffManager) -> HashMap<String, f64>>,
	pub unlock: &'static str,

}

impl TechnologyAsset {

	pub fn new(name: &'static str, price: Box<dyn Fn(&StuffManager) -> HashMap<String, f64>>, unlock: &'static str) -> Self {

		Self {

			name,
			price,
			unlock

		}

	}

}