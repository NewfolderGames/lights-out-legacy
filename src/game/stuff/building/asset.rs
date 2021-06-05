use std::collections::HashMap;
use crate::game::stuff::resource::ResourceManager;

/// A building asset.
pub struct BuildingAsset {

	pub name: &'static str,
	pub category: &'static str,
	pub modifiers: Box<dyn Fn(&HashMap<String, f64>) -> Vec<(&'static str, f64)>>,
	pub deficit: Box<dyn Fn(&ResourceManager) -> bool>,
	pub price: Vec<(&'static str, f64)>,
	pub price_multiplier: f64,

}

impl BuildingAsset {

	/// Creates a new building asset.
	pub fn new(
		name: &'static str,
		category: &'static str,
		modifiers: Box<dyn Fn(&HashMap<String, f64>) -> Vec<(&'static str, f64)>>,
		deficit: Box<dyn Fn(&ResourceManager) -> bool>,
		price: Vec<(&'static str, f64)>,
		price_multiplier: f64
	) -> Self {

		Self {

			name,
			category,
			modifiers,
			deficit,
			price,
			price_multiplier

		}
		
	}

}