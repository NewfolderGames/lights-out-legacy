use crate::game::asset::{ AssetManager, ResourceAsset };
use crate::game::stuff::StuffManager;

pub fn load(asset_manager: &mut AssetManager, stuff_manager: &StuffManager) {

	// Raw material.

	asset_manager.load_resource(ResourceAsset::new(
		"resource_wood",
		"resource_wood_title",
		"resource_wood_description",
		"resource_wood_image",
		"raw_resource",
		Box::new(|| {
			100f64
		}),
		Box::new(|| {
			0f64
		}),
	));

	asset_manager.load_resource(ResourceAsset::new(
		"resource_stone",
		"resource_stone_title",
		"resource_stone_description",
		"resource_stone_image",
		"raw_resource",
		Box::new(|| {
			100f64
		}),
		Box::new(|| {
			0f64
		}),
	));

	asset_manager.load_resource(ResourceAsset::new(
		"resource_iron",
		"resource_iron_title",
		"resource_iron_description",
		"resource_iron_image",
		"raw_resource",
		Box::new(|| {
			100f64
		}),
		Box::new(|| {
			0f64
		}),
	));

	// Other.

	asset_manager.load_resource(ResourceAsset::new(
		"resource_energy",
		"resource_energy_title",
		"resource_energy_description",
		"resource_energy_image",
		"other",
		Box::new(|| {
			100f64
		}),
		Box::new(|| {
			0f64
		}),
	));

}