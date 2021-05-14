use crate::game::building::{ BuildingData, BuildingManager };

pub fn load(manager: &mut BuildingManager) {

	// Production.

	manager.load(BuildingData::new(
		"building_sawmill",
		"building_sawmill_title",
		"building_sawmill_description",
		"building_sawmill_image",
		"production",
		false,
		false,
		vec![
			(String::from("modifier_resource_wood_production_base"), 1f64),
			(String::from("modifier_resource_wood_production_multiplier"), 0.01f64)
		],
		vec![
			(String::from("wood"), 10f64),
			(String::from("stone"), 10f64),
			(String::from("iron"), 10f64)
		],
		1.15f64
	));

	// Storage.

	manager.load(BuildingData::new(
		"building_stockpile",
		"building_stockpile_title",
		"building_stockpile_description",
		"building_stockpile_image",
		"storage",
		false,
		true,
		vec![
			(String::from("modifier_resource_wood_storage_base"), 100f64),
			(String::from("modifier_resource_stone_storage_base"), 100f64),
		],
		vec![
			(String::from("wood"), 10f64)
		],
		1.15f64
	));

	manager.load(BuildingData::new(
		"building_shed",
		"building_shed_title",
		"building_shed_description",
		"building_shed_image",
		"storage",
		false,
		true,
		vec![
			(String::from("modifier_resource_wood_storage_base"), 500f64),
			(String::from("modifier_resource_stone_storage_base"), 500f64),
			(String::from("modifier_resource_iron_storage_base"), 100f64),
		],
		vec![
			(String::from("wood"), 100f64),
			(String::from("stone"), 100f64)
		],
		1.15f64
	));
	

}