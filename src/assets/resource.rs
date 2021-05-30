use crate::game::stuff::{ ResourceAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Mana.

	stuff_manager.load_resource(ResourceAsset::new("resource_culture", "mana", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_knowledge", "mana", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_science", "mana", 10f64));

	// Raw material.

	stuff_manager.load_resource(ResourceAsset::new("resource_copper", "rawMaterial", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_food", "rawMaterial", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_iron", "rawMaterial", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_ore", "rawMaterial", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_stone", "rawMaterial", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_wood", "rawMaterial", 10f64));

}