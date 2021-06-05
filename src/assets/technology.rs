use crate::game::stuff::StuffManager;
use crate::game::stuff::technology::TechnologyAsset;

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_technology(TechnologyAsset::new(
		"lighthouse",
		vec![
			("science", 10f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"agriculture",
		vec![
			("science", 20f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"housing_basic",
		vec![
			("science", 20f64),
			("stone", 15f64),
			("wood", 15f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"tools_simple",
		vec![
			("science", 30f64),
			("stone", 20f64),
			("wood", 20f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"woodworking",
		vec![
			("science", 50f64),
			("wood", 25f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"mining",
		vec![
			("science", 50f64),
			("stone", 25f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"stoneCutting",
		vec![
			("science", 65f64),
			("stone", 40f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"smelting",
		vec![
			("science", 65f64),
			("ore", 30f64),
		]
	));

	stuff_manager.load_technology(TechnologyAsset::new(
		"ironWorking",
		vec![
			("science", 100f64),
			("ore", 30f64),
			("copper", 30f64),
		]
	));

}