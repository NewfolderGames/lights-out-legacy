use crate::game::stuff::StuffManager;

pub struct TechnologyAsset {

	pub name: &'static str,
	pub title: &'static str,
	pub description: &'static str,
	pub image: &'static str,

	pub price: Box<dyn Fn(&StuffManager) -> Vec<(String, f64)>>,
	pub unlock: Vec<&'static str>,

}

impl TechnologyAsset {

	pub fn new(name: &'static str, title: &'static str, description: &'static str, image: &'static str, price: Box<dyn Fn(&StuffManager) -> Vec<(String, f64)>>, unlock: Vec<&'static str>) -> Self {

		Self {

			name,
			title,
			description,
			image,
			price,
			unlock

		}

	}

}