use crate::game::stuff::{ ResourceAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Raw material.

	stuff_manager.load_resource(ResourceAsset::new(
		"resource_wood",
		"resource_wood_title",
		"resource_wood_description",
		"resource_wood_image",
		"rawMateiral",
		Box::new(|stuff_manager| {
			(
				100f64 + 
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
		"resource_stone_title",
		"resource_stone_description",
		"resource_stone_image",
		"rawMateiral",
		Box::new(|stuff_manager| {
			(
				100f64 + 
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
		"resource_copper_title",
		"resource_copper_description",
		"resource_copper_image",
		"rawMateiral",
		Box::new(|stuff_manager| {
			(
				100f64 + 
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
		"resource_iron_title",
		"resource_iron_description",
		"resource_iron_image",
		"rawMateiral",
		Box::new(|stuff_manager| {
			(
				100f64 + 
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

}