use crate::game::stuff::{ ModifierAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Building.

	

	// Resource.

	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_rawMaterial_storage_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_rawMaterial_storage_multiplier"));

	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_storage_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_storage_multiplier"));
	
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_storage_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_storage_multiplier"));
	
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_storage_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_storage_multiplier"));
	
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_production_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_production_multiplier"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_storage_base"));
	stuff_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_storage_multiplier"));

}