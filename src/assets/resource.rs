use crate::game::asset::AssetManager;
use crate::game::resource::ResourceAsset;

pub fn load(manager: &mut AssetManager) {

	// Raw material.

	manager.load_resource(ResourceAsset::new(
		"resource_wood",
		"resource_wood_title",
		"resource_wood_description",
		"resource_wood_image",
		"raw_resource",
		false,
		true,
		100f64
	));

	manager.load_resource(ResourceAsset::new(
		"resource_stone",
		"resource_stone_title",
		"resource_stone_description",
		"resource_stone_image",
		"raw_resource",
		false,
		true,
		100f64
	));

	manager.load_resource(ResourceAsset::new(
		"resource_iron",
		"resource_iron_title",
		"resource_iron_description",
		"resource_iron_image",
		"raw_resource",
		false,
		false,
		100f64
	));

}