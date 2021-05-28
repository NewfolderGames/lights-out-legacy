use crate::game::stuff::{ ModifierAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Building - All
	

	// Resource - Categories.

	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_category_rawMaterial_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_category_rawMaterial_capacity_multiplier"));

	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_category_mana_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_category_mana_capacity_multiplier"));

	// Resource - Types.

	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_consumption_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_consumption_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_capacity_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_food_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_food_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_food_consumption_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_food_consumption_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_food_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_food_capacity_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_consumption_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_consumption_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_capacity_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_knowledge_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_knowledge_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_knowledge_consumption_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_knowledge_consumption_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_knowledge_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_knowledge_capacity_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_science_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_science_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_science_consumption_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_science_consumption_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_science_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_science_capacity_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_consumption_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_consumption_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_capacity_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_consumption_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_consumption_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_capacity_multiplier"));

	// Lighthouse

	stuff_manager.load_modifier(ModifierAsset::new("modifier_lighthouse_examine_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_lighthouse_gather_base"));

}