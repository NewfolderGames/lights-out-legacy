use std::collections::HashMap;
use crate::game::stuff::{ StuffManager, TechnologyAsset };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_agriculture",
		Box::new(|stuff_manager| {
			let mut hashmap = HashMap::new();
			hashmap.insert(String::from("resource_science"), 25f64);
			hashmap
		}),
		"unlock_technology_agriculture"
	));

}