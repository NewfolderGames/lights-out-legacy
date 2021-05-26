use crate::game::stuff::{ StuffManager, TechnologyAsset };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_agriculture",
		Box::new(|_| {
			vec![
				("resource_science", 25f64),
			]
		}),
		"unlock_technology_agriculture"
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_woodworking",
		Box::new(|_| {
			vec![
				("resource_science", 100f64),
				("resource_wood", 25f64),
			]
		}),
		"unlock_technology_woodworking"
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_mining",
		Box::new(|_| {
			vec![
				("resource_science", 100f64),
				("resource_stone", 25f64),
			]
		}),
		"unlock_technology_mining"
	));

}