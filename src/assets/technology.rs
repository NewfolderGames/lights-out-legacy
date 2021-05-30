use crate::game::stuff::{ StuffManager, TechnologyAsset };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_lighthouse",
		vec![
			("resource_science", 10f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_agriculture",
		vec![
			("resource_science", 25f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_housing_basic",
		vec![
			("resource_science", 25f64),
			("resource_stone", 15f64),
			("resource_wood", 15f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_workbench",
		vec![
			("resource_science", 30f64),
			("resource_stone", 20f64),
			("resource_wood", 20f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_woodworking",
		vec![
			("resource_science", 50f64),
			("resource_wood", 25f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_mining",
		vec![
			("resource_science", 50f64),
			("resource_stone", 25f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_stoneCutting",
		vec![
			("resource_science", 65f64),
			("resource_stone", 40f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_smelting",
		vec![
			("resource_science", 65f64),
			("resource_ore", 30f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"technology_ironWorking",
		vec![
			("resource_science", 100f64),
			("resource_ore", 30f64),
			("resource_copper", 30f64),
		]
	));

}