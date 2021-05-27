use crate::game::stuff::{ StuffManager, TextAsset };

pub fn load(stuff_manager: &mut StuffManager) {

	// Log.

	stuff_manager.load_text(TextAsset::new("log_game_pause", "Game paused."));
	stuff_manager.load_text(TextAsset::new("log_game_resume", "Game resumed."));
	stuff_manager.load_text(TextAsset::new("log_game_save", "Game saved."));
	stuff_manager.load_text(TextAsset::new("log_game_save_loaded", "Game save loaded."));

	stuff_manager.load_text(TextAsset::new("log_tab_lighthouse_examine_0", "You have examined the lighthouse."));
	stuff_manager.load_text(TextAsset::new("log_tab_lighthouse_examine_1", "You can feel the warmth surrounding you"));
	stuff_manager.load_text(TextAsset::new("log_tab_lighthouse_examine_2", "You have noticed some scraps on the ground."));
	stuff_manager.load_text(TextAsset::new("log_tab_lighthouse_examine_3", "You have studied the lighthouse."));
	stuff_manager.load_text(TextAsset::new("log_tab_lighthouse_gather_0", "You have gathered scraps lying around."));
	stuff_manager.load_text(TextAsset::new("log_tab_lighthouse_gather_1", "You should build something with it."));

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

	stuff_manager.load_text(TextAsset::new("stat_lighthouse_lightsout_title", "Lights out"));
	stuff_manager.load_text(TextAsset::new("stat_lighthouse_examined_title", "Lighthouse examined"));
	stuff_manager.load_text(TextAsset::new("stat_lighthouse_gathered_title", "Lighthouse scraps gathered"));

	stuff_manager.load_text(TextAsset::new("stat_debug_title", "Debug"));
	stuff_manager.load_text(TextAsset::new("stat_game_booted_title", "Game booted"));
	stuff_manager.load_text(TextAsset::new("stat_game_ticks_current_title", "Game ticks (current)"));
	stuff_manager.load_text(TextAsset::new("stat_game_ticks_total_title", "Game ticks (total)"));

	// Technology.

	stuff_manager.load_text(TextAsset::new("technology_lighthouse_title", "Lighthouse"));
	stuff_manager.load_text(TextAsset::new("technology_agriculture_title", "Agriculture"));
	
	stuff_manager.load_text(TextAsset::new("technology_lighthouse_description", "A structure that keeps the void away."));
	stuff_manager.load_text(TextAsset::new("technology_agriculture_description", "Develops a way to grow platns using the lighthouse."));

	// UI.

	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse", "Lighthouse"));
	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse_button_examine", "Examine the lighthouse"));
	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse_button_gather", "Gather scraps"));
	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse_button_lightsout", "Lights out"));

	stuff_manager.load_text(TextAsset::new("ui_tab_stats", "Stats"));

	stuff_manager.load_text(TextAsset::new("ui_tab_technology", "Technology"));

}
