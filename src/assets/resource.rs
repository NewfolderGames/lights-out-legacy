use crate::game::stuff::StuffManager;
use crate::game::stuff::resource::ResourceAsset;

pub fn load(stuff_manager: &mut StuffManager) {

	// Mana.

	stuff_manager.load_resource(ResourceAsset::new("culture", "mana", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("knowledge", "mana", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("science", "mana", 10f64));

	// Raw material.

	stuff_manager.load_resource(ResourceAsset::new("copper", "rawMaterial", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("food", "rawMaterial", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("iron", "rawMaterial", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("ore", "rawMaterial", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("stone", "rawMaterial", 10f64));
	stuff_manager.load_resource(ResourceAsset::new("wood", "rawMaterial", 10f64));

}