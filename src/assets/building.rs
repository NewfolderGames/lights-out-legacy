use crate::game::stuff::{ BuildingAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Production.
	
	stuff_manager.load_building(BuildingAsset::new(
		"building_sawmill",
		"building_sawmill_title",
		"building_sawmill_description",
		"building_sawmill_image",
		"production",
		Box::new(|stuff_manager| {
			vec![
				(String::from("modifier_resource_wood_production_base"), 1f64),
				(String::from("modifier_resource_wood_production_multiplier"), 0.01f64)
			]
		}),
		Box::new(|stuff_manager| {
			vec![
				(String::from("wood"), 10f64),
				(String::from("stone"), 10f64),
				(String::from("iron"), 10f64)
			]
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
			vec![
				(String::from("modifier_resource_wood_storage_base"), 100f64),
				(String::from("modifier_resource_stone_storage_base"), 100f64),
			]
		}),
		Box::new(|stuff_manager| {
			vec![
				(String::from("wood"), 10f64)
			]
		}),
		1.15f64
	));

	stuff_manager.load_building(BuildingAsset::new(
		"building_shed",
		"building_shed_title",
		"building_shed_description",
		"building_shed_image",
		"storage",
		Box::new(|stuff_manager| {
			vec![
				(String::from("modifier_resource_wood_storage_base"), 500f64),
				(String::from("modifier_resource_stone_storage_base"), 500f64),
				(String::from("modifier_resource_iron_storage_base"), 100f64),
			]
		}),
		Box::new(|stuff_manager| { 
			vec![
				(String::from("wood"), 100f64),
				(String::from("stone"), 100f64)
			]
		}),
		1.15f64
	));
	

}