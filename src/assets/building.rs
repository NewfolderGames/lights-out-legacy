use std::collections::HashMap;
use crate::game::stuff::{ BuildingAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Production.

	stuff_manager.load_building(BuildingAsset::new(
		"building_garden",
		"building_garden_title",
		"building_garden_description",
		"building_garden_image",
		"production",
		Box::new(|stuff_manager| {
			let mut hashmap = HashMap::new();
			hashmap.insert(String::from("modifier_resource_food_production_base"), 1f64);
			hashmap.insert(String::from("modifier_resource_food_production_multiplier"), 0.01f64);
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
	
	// Storage.

	stuff_manager.load_building(BuildingAsset::new(
		"building_stockpile",
		"building_stockpile_title",
		"building_stockpile_description",
		"building_stockpile_image",
		"storage",
		Box::new(|stuff_manager| {
			let mut hashmap = HashMap::new();
			hashmap.insert(String::from("modifier_resource_wood_storage_base"), 100f64);
			hashmap.insert(String::from("modifier_resource_stone_storage_base"), 100f64);
			hashmap
		}),
		Box::new(|stuff_manager| {
			let mut hashmap = HashMap::new();
			hashmap.insert(String::from("resource_wood"), 25f64);
			hashmap.insert(String::from("resource_stone"), 25f64);
			hashmap
		}),
		1.15f64
	));

}