use crate::game::asset::{ AssetManager, ModifierAsset };

pub fn load(asset_manager: &mut AssetManager) {

	// Building.

	

	// Resource.

	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_rawMaterial_storage_base", "modifier_resource_rawMaterial_storage_base"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_rawMaterial_storage_multiplier", "modifier_resource_rawMaterial_storage_multiplier"));

	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_base", "modifier_resource_wood_production_base"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_multiplier", "modifier_resource_wood_production_multiplier"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_storage_base", "modifier_resource_wood_storage_base"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_storage_multiplier", "modifier_resource_wood_storage_multiplier"));
	
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_production_base", "modifier_resource_stone_production_base"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_production_multiplier", "modifier_resource_stone_production_multiplier"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_storage_base", "modifier_resource_stone_storage_base"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_stone_storage_multiplier", "modifier_resource_stone_storage_multiplier"));
	
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_production_base", "modifier_resource_copper_production_base"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_production_multiplier", "modifier_resource_copper_production_multiplier"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_storage_base", "modifier_resource_copper_storage_base"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_copper_storage_multiplier", "modifier_resource_copper_storage_multiplier"));
	
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_production_base", "modifier_resource_iron_production_base"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_production_multiplier", "modifier_resource_iron_production_multiplier"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_storage_base", "modifier_resource_iron_storage_base"));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_iron_storage_multiplier", "modifier_resource_iron_storage_multiplier"));

}