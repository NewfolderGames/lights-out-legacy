use std::collections::HashMap;
use crate::game::stuff::{ BuildingAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Mana.

	stuff_manager.load_building(BuildingAsset::new(
		"building_researchBench",
		"mana",
		vec![
			("modifier_resource_science_production_base", 0.25f64)
		],
		vec![
			("resource_wood", 10f64),
			("resource_stone", 10f64),
		],
		1.15f64
	));

	// Production.

	stuff_manager.load_building(BuildingAsset::new(
		"building_garden",
		"production",
		vec![
			("modifier_resource_food_production_base", 0.25f64),
		],
		vec![
			("resource_wood", 10f64),
			("resource_stone", 10f64),
		],
		1.15f64
	));

	
	// Storage.

	stuff_manager.load_building(BuildingAsset::new(
		"building_stockpile",
		"storage",
		vec![
			("modifier_resource_food_storage_base", 100f64),
			("modifier_resource_stone_storage_base", 100f64),
			("modifier_resource_wood_storage_base", 100f64),
		],
		vec![
			("resource_stone", 10f64),
			("resource_wood", 10f64),
		],
		1.15f64
	));

}