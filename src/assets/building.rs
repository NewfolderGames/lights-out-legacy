use crate::game::stuff::{ BuildingAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// DO NOT CHANGE OUTPUT MODIFIER LIST OR PRICE LIST ON RUNTIME.
	// ONLY VALUES SHOULD BE CHANGED.

	// Mana

	stuff_manager.load_building(BuildingAsset::new(
		"building_researchBench",
		"mana",
		Box::new(|_, _| {
			vec![
				("modifier_resource_science_capacity_base", 10f64),
			]
		}),
		Box::new(|_, _| {
			vec![
				("resource_stone", 25f64),
				("resource_wood", 25f64),
			]
		}),
		1.15f64
	));

	// Storage.

	stuff_manager.load_building(BuildingAsset::new(
		"building_stockpile",
		"storage",
		Box::new(|_, _| {
			vec![
				("modifier_resource_stone_capacity_base", 10f64),
				("modifier_resource_wood_capacity_base", 10f64),
			]
		}),
		Box::new(|_, _| {
			vec![
				("resource_stone", 10f64),
				("resource_wood", 10f64),
			]
		}),
		1.15f64
	));

}