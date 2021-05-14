use crate::game::asset::AssetManager;
use crate::game::modifier::ModifierAsset;

pub fn load(manager: &mut AssetManager) {

	// Resource.

	manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_base", "modifier_resource_wood_production_base", 0f64));
	manager.load_modifier(ModifierAsset::new("modifier_resource_wood_production_multiplier", "modifier_resource_wood_production_multiplier", 1f64));
	manager.load_modifier(ModifierAsset::new("modifier_resource_wood_storage_base", "modifier_resource_wood_storage_base", 0f64));
	manager.load_modifier(ModifierAsset::new("modifier_resource_wood_storage_multiplier", "modifier_resource_wood_storage_multiplier", 1f64));
	
}