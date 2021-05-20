use crate::game::stuff::{ ResourceAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Raw material.

	stuff_manager.load_resource(ResourceAsset::new("resource_copper", "rawMateiral", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_iron", "rawMateiral", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_stone", "rawMateiral", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_wood", "rawMateiral", 10f64));

	// Mana.

	stuff_manager.load_resource(ResourceAsset::new("resource_culture", "mana", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_knowledge", "mana", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("resource_science", "mana", 10f64));

}