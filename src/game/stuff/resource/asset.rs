use crate::game::stuff::StuffManager;

pub struct ResourceAsset {

	pub name: &'static str,

	pub category: &'static str,
	pub capacity: Box<dyn Fn(&StuffManager) -> f64>,
	pub production: Box<dyn Fn(&StuffManager) -> f64>,

}

impl ResourceAsset {

	pub fn new(name: &'static str, category: &'static str, capacity: Box<dyn Fn(&StuffManager) -> f64>, production: Box<dyn Fn(&StuffManager) -> f64>) -> Self {

		Self {

			name,
			category,
			capacity,
			production

		}

	}

}