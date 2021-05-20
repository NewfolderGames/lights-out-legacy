use std::collections::HashMap;
use crate::game::stuff::{ StuffManager, TechnologyAsset };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_agriculture",
		vec![
			("resource_science", 25f64),
		],
		"unlock_technology_agriculture"
	));

}