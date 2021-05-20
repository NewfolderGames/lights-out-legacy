use crate::game::stuff::{ ModifierAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Building - All
	

	// Resource - Categories.

	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_rawMaterial_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_rawMaterial_capacity_multiplier"));

	// Resource - Types.

	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_capacity_multiplier"));
	
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_capacity_multiplier"));
	
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_capacity_multiplier"));

	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_capacity_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_capacity_multiplier"));

}