use crate::game::stuff::StuffManager;
use crate::game::stuff::building::BuildingAsset;

pub fn load(stuff_manager: &mut StuffManager) {

	// Mana

	stuff_manager.load_building(BuildingAsset::new(
		"researchBench",
		"mana",
		Box::new(|_| {
			vec![
				("resource_science_capacity_base", 10f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("stone", 25f64),
			("wood", 25f64),
		],
		1.15f64
	));

	stuff_manager.load_building(BuildingAsset::new(
		"workbench",
		"mana",
		Box::new(|_| {
			vec![
				("resource_knowledge_capacity_base", 10f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("stone", 25f64),
			("wood", 25f64),
		],
		1.15f64
	));

	// Housing.

	stuff_manager.load_building(BuildingAsset::new(
		"tent",
		"housing",
		Box::new(|_| {
			vec![
				("housing_base", 1f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("stone", 25f64),
			("wood", 25f64),
		],
		1.20f64
	));

	// Production.

	stuff_manager.load_building(BuildingAsset::new(
		"garden",
		"production",
		Box::new(|_| {
			vec![
				("resource_food_production_base", 0.2f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("stone", 15f64),
			("wood", 15f64),
		],
		1.125f64
	));

	stuff_manager.load_building(BuildingAsset::new(
		"mine",
		"production",
		Box::new(|_| {
			vec![
				("job_miner_capacity_base", 1f64),
				("job_miner_production_base", 0.2f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("stone", 15f64),
			("wood", 15f64),
		],
		1.125f64
	));

	// Storage.

	stuff_manager.load_building(BuildingAsset::new(
		"stockpile",
		"storage",
		Box::new(|modifiers| {
			vec![
				("resource_copper_capacity_base", modifiers.get("stockpile_capacity_copper_base").cloned().unwrap_or(0f64)),
				("resource_iron_capacity_base", modifiers.get("stockpile_capacity_iron_base").cloned().unwrap_or(0f64)),
				("resource_ore_capacity_base", modifiers.get("stockpile_ore_capacity_base").cloned().unwrap_or(0f64)),
				("resource_stone_capacity_base", 10f64),
				("resource_wood_capacity_base", 10f64),
			]
		}),
		Box::new(|_| false),
		vec![
			("stone", 10f64),
			("wood", 10f64),
		],
		1.15f64
	));

}