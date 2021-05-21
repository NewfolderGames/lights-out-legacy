use crate::game::stuff::{ StuffManager, TextAsset };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_text(TextAsset::new("resource_category_rawMaterial", "Raw Material"));
	stuff_manager.load_text(TextAsset::new("resource_category_mana", "Mana"));

	stuff_manager.load_text(TextAsset::new("resource_copper_title", "Copper"));
	stuff_manager.load_text(TextAsset::new("resource_culture_title", "Culture"));
	stuff_manager.load_text(TextAsset::new("resource_iron_title", "Iron"));
	stuff_manager.load_text(TextAsset::new("resource_knowledge_title", "Knowledge"));
	stuff_manager.load_text(TextAsset::new("resource_science_title", "Science"));
	stuff_manager.load_text(TextAsset::new("resource_stone_title", "Stone"));
	stuff_manager.load_text(TextAsset::new("resource_wood_title", "Wood"));

}
