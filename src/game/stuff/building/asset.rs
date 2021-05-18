use crate::game::stuff::StuffManager;

pub struct BuildingAsset {

	pub name: &'static str,
	pub title: &'static str,
	pub description: &'static str,
	pub image: &'static str,

	pub category: &'static str,

	pub modifiers: Box<dyn Fn(&StuffManager) -> Vec<(String, f64)>>,
	pub price: Box<dyn Fn(&StuffManager) -> Vec<(String, f64)>>,
	pub price_multiplier: f64,

}

impl BuildingAsset {

	pub fn new(name: &'static str, title: &'static str, description: &'static str, image: &'static str, category: &'static str, modifiers: Box<dyn Fn(&StuffManager) -> Vec<(String, f64)>>, price: Box<dyn Fn(&StuffManager) -> Vec<(String, f64)>>, price_multiplier: f64) -> Self {

		Self {

			name,
			title,
			description,
			image,
			category,
			modifiers,
			price,
			price_multiplier,

		}

	}

}