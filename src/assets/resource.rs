use crate::game::stuff::{ ResourceAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Raw material.

	stuff_manager.load_resource(ResourceAsset::new(
		"resource_wood",
		"rawMateiral",
		Box::new(|stuff_manager| {
			(
				10f64 + 
				stuff_manager.get_modifier_value("modifier_resource_wood_storage_base").unwrap_or(0f64) + 
				stuff_manager.get_modifier_value("modifier_resource_rawMaterial_storage_base").unwrap_or(0f64)
			) * (
				1f64 + 
				stuff_manager.get_modifier_value("modifier_resource_wood_storage_multiplier").unwrap_or(0f64) +
				stuff_manager.get_modifier_value("modifier_resource_rawMaterial_storage_multiplier").unwrap_or(0f64)
			)
		}),
		Box::new(|stuff_manager| {
			(
				stuff_manager.get_modifier_value("modifier_resource_wood_production_base").unwrap_or(0f64)
			) * (
				1f64 + 
				stuff_manager.get_modifier_value("modifier_resource_wood_production_multiplier").unwrap_or(0f64)
			)
		}),
	));

	stuff_manager.load_resource(ResourceAsset::new(
		"resource_stone",
		"rawMateiral",
		Box::new(|stuff_manager| {
			(
				10f64 + 
				stuff_manager.get_modifier_value("modifier_resource_stone_storage_base").unwrap_or(0f64) + 
				stuff_manager.get_modifier_value("modifier_resource_rawMaterial_storage_base").unwrap_or(0f64)
			) * (
				1f64 + 
				stuff_manager.get_modifier_value("modifier_resource_stone_storage_multiplier").unwrap_or(0f64) +
				stuff_manager.get_modifier_value("modifier_resource_rawMaterial_storage_multiplier").unwrap_or(0f64)
			)
		}),
		Box::new(|stuff_manager| {
			(
				stuff_manager.get_modifier_value("modifier_resource_stone_production_base").unwrap_or(0f64)
			) * (
				1f64 + 
				stuff_manager.get_modifier_value("modifier_resource_stone_production_multiplier").unwrap_or(0f64)
			)
		}),
	));

	stuff_manager.load_resource(ResourceAsset::new(
		"resource_copper",
		"rawMateiral",
		Box::new(|stuff_manager| {
			(
				10f64 + 
				stuff_manager.get_modifier_value("modifier_resource_copper_storage_base").unwrap_or(0f64) + 
				stuff_manager.get_modifier_value("modifier_resource_rawMaterial_storage_base").unwrap_or(0f64)
			) * (
				1f64 + 
				stuff_manager.get_modifier_value("modifier_resource_copper_storage_multiplier").unwrap_or(0f64) +
				stuff_manager.get_modifier_value("modifier_resource_rawMaterial_storage_multiplier").unwrap_or(0f64)
			)
		}),
		Box::new(|stuff_manager| {
			(
				stuff_manager.get_modifier_value("modifier_resource_copper_production_base").unwrap_or(0f64)
			) * (
				1f64 + 
				stuff_manager.get_modifier_value("modifier_resource_copper_production_multiplier").unwrap_or(0f64)
			)
		}),
	));

	stuff_manager.load_resource(ResourceAsset::new(
		"resource_iron",
		"rawMateiral",
		Box::new(|stuff_manager| {
			(
				10f64 + 
				stuff_manager.get_modifier_value("modifier_resource_iron_storage_base").unwrap_or(0f64) + 
				stuff_manager.get_modifier_value("modifier_resource_rawMaterial_storage_base").unwrap_or(0f64)
			) * (
				1f64 + 
				stuff_manager.get_modifier_value("modifier_resource_iron_storage_multiplier").unwrap_or(0f64) +
				stuff_manager.get_modifier_value("modifier_resource_rawMaterial_storage_multiplier").unwrap_or(0f64)
			)
		}),
		Box::new(|stuff_manager| {
			(
				stuff_manager.get_modifier_value("modifier_resource_iron_production_base").unwrap_or(0f64)
			) * (
				1f64 + 
				stuff_manager.get_modifier_value("modifier_resource_iron_production_multiplier").unwrap_or(0f64)
			)
		}),
	));

	// Mana.

	stuff_manager.load_resource(ResourceAsset::new(
		"resource_science",
		"mana",
		Box::new(|stuff_manager| {
			(
				25f64 + 
				stuff_manager.get_modifier_value("modifier_resource_science_storage_base").unwrap_or(0f64) + 
				stuff_manager.get_modifier_value("modifier_resource_mana_storage_base").unwrap_or(0f64)
			) * (
				1f64 + 
				stuff_manager.get_modifier_value("modifier_resource_science_storage_multiplier").unwrap_or(0f64) +
				stuff_manager.get_modifier_value("modifier_resource_mana_storage_multiplier").unwrap_or(0f64)
			)
		}),
		Box::new(|stuff_manager| {
			(
				stuff_manager.get_modifier_value("modifier_resource_science_production_base").unwrap_or(0f64)
			) * (
				1f64 + 
				stuff_manager.get_modifier_value("modifier_resource_science_production_multiplier").unwrap_or(0f64)
			)
		}),
	));

}