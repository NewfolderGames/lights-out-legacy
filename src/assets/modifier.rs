use crate::game::asset::{ AssetManager, ModifierAsset };

pub fn load(asset_manager: &mut AssetManager) {

	// Resource.

	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_base", "modifier_resource_wood_production_base", 0f64));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_multiplier", "modifier_resource_wood_production_multiplier", 1f64));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_storage_base", "modifier_resource_wood_storage_base", 0f64));
	asset_manager.load_modifier(ModifierAsset::new("modifier_resource_wood_storage_multiplier", "modifier_resource_wood_storage_multiplier", 1f64));
	
}