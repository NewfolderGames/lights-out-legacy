use crate::game::stuff::{ StuffManager, TextAsset };

pub fn load(stuff_manager: &mut StuffManager) {

	// Resource.

	stuff_manager.load_text(TextAsset::new("resource_category_rawMaterial", "Raw Material"));
	stuff_manager.load_text(TextAsset::new("resource_category_mana", "Mana"));

	stuff_manager.load_text(TextAsset::new("resource_copper_title", "Copper"));
	stuff_manager.load_text(TextAsset::new("resource_culture_title", "Culture"));
	stuff_manager.load_text(TextAsset::new("resource_iron_title", "Iron"));
	stuff_manager.load_text(TextAsset::new("resource_knowledge_title", "Knowledge"));
	stuff_manager.load_text(TextAsset::new("resource_science_title", "Science"));
	stuff_manager.load_text(TextAsset::new("resource_stone_title", "Stone"));
	stuff_manager.load_text(TextAsset::new("resource_wood_title", "Wood"));

	// Stat.

	stuff_manager.load_text(TextAsset::new("stat_category_lighthouse", "Lighthouse"));
	stuff_manager.load_text(TextAsset::new("stat_category_misc", "Misc"));

	stuff_manager.load_text(TextAsset::new("stat_lightsout_title", "Lights out"));
	stuff_manager.load_text(TextAsset::new("stat_examined_title", "Examined"));
	stuff_manager.load_text(TextAsset::new("stat_gathered_title", "Scraps gathered"));

	stuff_manager.load_text(TextAsset::new("stat_booted_total_title", "Game booted"));
	stuff_manager.load_text(TextAsset::new("stat_debug_title", "Debug"));
	stuff_manager.load_text(TextAsset::new("stat_ticks_current_title", "Ticks (current)"));
	stuff_manager.load_text(TextAsset::new("stat_ticks_total_title", "Ticks (total)"));

	// UI.

	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse", "Lighthouse"));
	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse_button_examine", "Examine the lighthouse"));
	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse_button_gather", "Gather scraps"));
	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse_button_lightsout", "Lights out"));

	stuff_manager.load_text(TextAsset::new("ui_tab_stats", "Stats"));

}
