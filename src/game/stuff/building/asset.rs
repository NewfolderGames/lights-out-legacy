use std::collections::HashMap;
use crate::game::stuff::StuffManager;

pub struct BuildingAsset {

	pub name: &'static str,

	pub category: &'static str,
	pub modifiers: Vec<(&'static str, f64)>,
	pub price: Vec<(&'static str, f64)>,
	pub price_multiplier: f64,

}

impl BuildingAsset {

	pub fn new(name: &'static str, category: &'static str, modifiers: Vec<(&'static str, f64)>, price: Vec<(&'static str, f64)>, price_multiplier: f64) -> Self {

		Self {

			name,
			category,
			modifiers,
			price,
			price_multiplier,

		}

	}

}