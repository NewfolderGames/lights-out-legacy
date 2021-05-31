use crate::game::stuff::{ BuildingAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// DO NOT CHANGE OUTPUT MODIFIER LIST OR PRICE LIST ON RUNTIME.
	// ONLY VALUES SHOULD BE CHANGED.

	// Mana

	stuff_manager.load_building(BuildingAsset::new(
		"building_researchBench",
		"mana",
		Box::new(|_| {
			vec![
				("modifier_resource_science_capacity_base", 10f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("resource_stone", 25f64),
			("resource_wood", 25f64),
		],
		1.15f64
	));

	stuff_manager.load_building(BuildingAsset::new(
		"building_workbench",
		"mana",
		Box::new(|_| {
			vec![
				("modifier_resource_knowledge_capacity_base", 10f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("resource_stone", 25f64),
			("resource_wood", 25f64),
		],
		1.15f64
	));

	// Housing.

	stuff_manager.load_building(BuildingAsset::new(
		"building_tent",
		"housing",
		Box::new(|_| {
			vec![
				("modifier_housing_base", 1f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("resource_stone", 25f64),
			("resource_wood", 25f64),
		],
		1.20f64
	));

	// Production.

	stuff_manager.load_building(BuildingAsset::new(
		"building_garden",
		"production",
		Box::new(|_| {
			vec![
				("modifier_resource_food_production_base", 0.2f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("resource_stone", 15f64),
			("resource_wood", 15f64),
		],
		1.125f64
	));

	stuff_manager.load_building(BuildingAsset::new(
		"building_mine",
		"production",
		Box::new(|_| {
			vec![
				("modifier_job_miner_capacity_base", 1f64),
				("modifier_job_miner_production_base", 0.2f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("resource_stone", 15f64),
			("resource_wood", 15f64),
		],
		1.125f64
	));

	// Storage.

	stuff_manager.load_building(BuildingAsset::new(
		"building_stockpile",
		"storage",
		Box::new(|ms| {
			vec![
				("modifier_resource_copper_capacity_base", ms.get_value("modifier_building_stockpile_capacity_copper_base").unwrap_or(0f64)),
				("modifier_resource_iron_capacity_base", ms.get_value("modifier_building_stockpile_capacity_iron_base").unwrap_or(0f64)),
				("modifier_resource_ore_capacity_base", ms.get_value("modifier_building_stockpile_ore_capacity_base").unwrap_or(0f64)),
				("modifier_resource_stone_capacity_base", 10f64),
				("modifier_resource_wood_capacity_base", 10f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("resource_stone", 10f64),
			("resource_wood", 10f64),
		],
		1.15f64
	));

}