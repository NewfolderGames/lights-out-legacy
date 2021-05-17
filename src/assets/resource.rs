use crate::game::asset::{ AssetManager, ResourceAsset };
use crate::game::stuff::StuffManager;

pub fn load(asset_manager: &mut AssetManager) {

	// Raw material.

	asset_manager.load_resource(ResourceAsset::new(
		"resource_wood",
		"resource_wood_title",
		"resource_wood_description",
		"resource_wood_image",
		"raw_resource",
		Box::new(|stuff_manager| {
			let mut value = 100f64;
			value += stuff_manager.get_modifier("modifier_resource_wood_storage_base").map_or(0f64, |m| m.get_value());
			value *= stuff_manager.get_modifier("modifier_resource_wood_storage_multiplier").map_or(1f64, |m| m.get_value());
			value
		}),
		Box::new(|stuff_manager| {
			let mut value = 0f64;
			value += stuff_manager.get_modifier("modifier_resource_wood_production_base").map_or(0f64, |m| m.get_value());
			value *= stuff_manager.get_modifier("modifier_resource_wood_production_multiplier").map_or(1f64, |m| m.get_value());
			value
		}),
	));

	asset_manager.load_resource(ResourceAsset::new(
		"resource_stone",
		"resource_stone_title",
		"resource_stone_description",
		"resource_stone_image",
		"raw_resource",
		Box::new(|stuff_manager| {
			let mut value = 100f64;
			value += stuff_manager.get_modifier("modifier_resource_stone_storage_base").map_or(0f64, |m| m.get_value());
			value *= stuff_manager.get_modifier("modifier_resource_stone_storage_multiplier").map_or(1f64, |m| m.get_value());
			value
		}),
		Box::new(|stuff_manager| {
			let mut value = 0f64;
			value += stuff_manager.get_modifier("modifier_resource_stone_production_base").map_or(0f64, |m| m.get_value());
			value *= stuff_manager.get_modifier("modifier_resource_stone_production_multiplier").map_or(1f64, |m| m.get_value());
			value
		}),
	));

	asset_manager.load_resource(ResourceAsset::new(
		"resource_iron",
		"resource_iron_title",
		"resource_iron_description",
		"resource_iron_image",
		"raw_resource",
		Box::new(|stuff_manager| {
			let mut value = 100f64;
			value += stuff_manager.get_modifier("modifier_resource_iron_storage_base").map_or(0f64, |m| m.get_value());
			value *= stuff_manager.get_modifier("modifier_resource_iron_storage_multiplier").map_or(1f64, |m| m.get_value());
			value
		}),
		Box::new(|stuff_manager| {
			let mut value = 0f64;
			value += stuff_manager.get_modifier("modifier_resource_iron_production_base").map_or(0f64, |m| m.get_value());
			value *= stuff_manager.get_modifier("modifier_resource_iron_production_multiplier").map_or(1f64, |m| m.get_value());
			value
		}),
	));

	// Other.

	asset_manager.load_resource(ResourceAsset::new(
		"resource_energy",
		"resource_energy_title",
		"resource_energy_description",
		"resource_energy_image",
		"other",
		Box::new(|stuff_manager| {
			let mut value = 100f64;
			value += stuff_manager.get_modifier("modifier_resource_energy_storage_base").map_or(0f64, |m| m.get_value());
			value *= stuff_manager.get_modifier("modifier_resource_energy_storage_multiplier").map_or(1f64, |m| m.get_value());
			value
		}),
		Box::new(|stuff_manager| {
			let mut value = 0f64;
			value += stuff_manager.get_modifier("modifier_resource_energy_production_base").map_or(0f64, |m| m.get_value());
			value *= stuff_manager.get_modifier("modifier_resource_energy_production_multiplier").map_or(1f64, |m| m.get_value());
			value
		}),
	));

}