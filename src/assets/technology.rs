use crate::game::stuff::{ StuffManager, TechnologyAsset };

pub fn load(stuff_manager: &mut StuffManager) {

	// DO NOT CHANGE PRICE LIST ON RUNTIME.
	// ONLY VALUES SHOULD BE CHANGED.

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_lighthouse",
		Box::new(|_| {
			vec![
				("resource_science", 10f64),
			]
		})
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_agriculture",
		Box::new(|_| {
			vec![
				("resource_science", 25f64),
			]
		})
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_hut",
		Box::new(|_| {
			vec![
				("resource_science", 35f64),
				("resource_wood", 20f64),
			]
		})
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_workbench",
		Box::new(|_| {
			vec![
				("resource_science", 45f64),
				("resource_stone", 20f64),
				("resource_wood", 20f64),
			]
		})
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_woodworking",
		Box::new(|_| {
			vec![
				("resource_science", 50f64),
				("resource_wood", 25f64),
			]
		})
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_mining",
		Box::new(|_| {
			vec![
				("resource_science", 50f64),
				("resource_stone", 25f64),
			]
		})
	));

}