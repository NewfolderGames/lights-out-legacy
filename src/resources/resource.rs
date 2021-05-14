use crate::game::resource::{ ResourceData, ResourceManager };

pub fn load(manager: &mut ResourceManager) {

	// Raw material.

	manager.load(ResourceData::new(
		"resource_wood",
		"resource_wood_title",
		"resource_wood_description",
		"resource_wood_image",
		"raw_resource",
		false,
		true,
		100f64
	));

	manager.load(ResourceData::new(
		"resource_stone",
		"resource_stone_title",
		"resource_stone_description",
		"resource_stone_image",
		"raw_resource",
		false,
		true,
		100f64
	));

	manager.load(ResourceData::new(
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