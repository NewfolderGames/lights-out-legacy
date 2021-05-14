use crate::game::modifier::{ ModifierData, ModifierManager };

pub fn load(manager: &mut ModifierManager) {

	// Resource.

	manager.load(ModifierData::new("modifier_resource_wood_production_base", "modifier_resource_wood_production_base", 0f64));
	manager.load(ModifierData::new("modifier_resource_wood_production_multiplier", "modifier_resource_wood_production_multiplier", 1f64));
	manager.load(ModifierData::new("modifier_resource_wood_storage_base", "modifier_resource_wood_storage_base", 0f64));
	manager.load(ModifierData::new("modifier_resource_wood_storage_multiplier", "modifier_resource_wood_storage_multiplier", 1f64));
	
}