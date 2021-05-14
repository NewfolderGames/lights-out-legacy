use std::collections::HashMap;

/// A loadable building data.
pub struct BuildingData {

	pub name: &'static str,
	pub title: &'static str,
	pub description: &'static str,
	pub image: &'static str,
	pub category: &'static str,
	
	pub is_hidden: bool,
	pub is_unlocked: bool,

	pub modifiers: Vec<(String, f64)>,
	pub price: Vec<(String, f64)>,
	pub price_multiplier: f64,

}

// Constructor.

impl BuildingData {

	/// Creates a new building data.
	pub fn new(name: &'static str, title: &'static str, description: &'static str, image: &'static str, category: &'static str, is_hidden: bool, is_unlocked: bool, modifiers: Vec<(String, f64)>, price: Vec<(String, f64)>, price_multiplier: f64) -> Self {

		Self {

			name,
			title,
			description,
			image,
			category,
			is_hidden,
			is_unlocked,
			price,
			price_multiplier,
			modifiers

		}

	}

}