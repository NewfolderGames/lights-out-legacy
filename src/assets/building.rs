use std::collections::HashMap;
use crate::game::stuff::{ BuildingAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Production.

	stuff_manager.load_building(BuildingAsset::new(
		"building_garden",
		"production",
		Box::new(|stuff_manager| {
			let mut hashmap = HashMap::new();
			hashmap.insert(String::from("modifier_resource_food_production_base"), 0.25f64);
			hashmap
		}),
		Box::new(|stuff_manager| {
			let mut hashmap = HashMap::new();
			hashmap.insert(String::from("resource_wood"), 10f64);
			hashmap.insert(String::from("resource_stone"), 10f64);
			hashmap
		}),
		1.15f64
	));

	// Mana.
	
	// Storage.

	stuff_manager.load_building(BuildingAsset::new(
		"building_stockpile",
		"storage",
		Box::new(|stuff_manager| {
			let mut hashmap = HashMap::new();
			hashmap.insert(String::from("modifier_resource_food_storage_base"), 100f64);
			hashmap.insert(String::from("modifier_resource_stone_storage_base"), 100f64);
			hashmap.insert(String::from("modifier_resource_wood_storage_base"), 100f64);
			hashmap
		}),
		Box::new(|stuff_manager| {
			let mut hashmap = HashMap::new();
			hashmap.insert(String::from("resource_stone"), 10f64);
			hashmap.insert(String::from("resource_wood"), 10f64);
			hashmap
		}),
		1.15f64
	));

}