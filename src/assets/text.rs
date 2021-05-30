use crate::game::stuff::{ StuffManager, TextAsset };

pub fn load(stuff_manager: &mut StuffManager) {

	// Buildings.

	stuff_manager.load_text(TextAsset::new("building_category_mana", "Mana"));
	stuff_manager.load_text(TextAsset::new("building_category_housing", "Housing"));
	stuff_manager.load_text(TextAsset::new("building_category_production", "Production"));
	stuff_manager.load_text(TextAsset::new("building_category_storage", "Storage"));

	stuff_manager.load_text(TextAsset::new("building_garden_title", "Garden"));
	stuff_manager.load_text(TextAsset::new("building_researchBench_title", "Research Bench"));
	stuff_manager.load_text(TextAsset::new("building_stockpile_title", "Stockpile"));
	stuff_manager.load_text(TextAsset::new("building_tent_title", "Tent"));
	stuff_manager.load_text(TextAsset::new("building_workbench_title", "Workbench"));

	stuff_manager.load_text(TextAsset::new("building_garden_description", "A tiny garden that uses lighthouse to produce food."));
	stuff_manager.load_text(TextAsset::new("building_researchBench_description", "A simple bench where you can store papers."));
	stuff_manager.load_text(TextAsset::new("building_stockpile_description", "An area where you can put all sorts of stuffs."));
	stuff_manager.load_text(TextAsset::new("building_tent_description", "A small tent that can house one person."));
	stuff_manager.load_text(TextAsset::new("building_workbench_description", "A simple bench where you can store experiments."));

	// Features.

	stuff_manager.load_text(TextAsset::new("feature_lighthouse_examine", "Exmaine lighthouse"));
	stuff_manager.load_text(TextAsset::new("feature_lighthouse_gather", "Gather scraps"));
	stuff_manager.load_text(TextAsset::new("feature_lighthouse_lightsout", "Lights out"));

	stuff_manager.load_text(TextAsset::new("feature_tab_building", "Building tab"));
	stuff_manager.load_text(TextAsset::new("feature_tab_lighthouse", "Lighthouse tab"));
	stuff_manager.load_text(TextAsset::new("feature_tab_stats", "Stats tab"));
	stuff_manager.load_text(TextAsset::new("feature_tab_technology", "Technology tab"));
	stuff_manager.load_text(TextAsset::new("feature_tab_upgrade", "Upgrade tab"));

	// Jobs.

	stuff_manager.load_text(TextAsset::new("job_farmer_title", "Farmer"));
	stuff_manager.load_text(TextAsset::new("job_miner_title", "Miner"));
	stuff_manager.load_text(TextAsset::new("job_woodcutter_title", "Woodcutter"));

	// Logs.

	stuff_manager.load_text(TextAsset::new("log_game_pause", "Game paused."));
	stuff_manager.load_text(TextAsset::new("log_game_resume", "Game resumed."));
	stuff_manager.load_text(TextAsset::new("log_game_save", "Game saved."));
	stuff_manager.load_text(TextAsset::new("log_game_save_loaded", "Game save loaded."));

	stuff_manager.load_text(TextAsset::new("log_tab_lighthouse_examine", "You can feel the warmth surrounding you"));
	stuff_manager.load_text(TextAsset::new("log_tab_lighthouse_gather", "You should build something with it."));
	
	// Modifiers.

	stuff_manager.load_text(TextAsset::new("modifier_housing_base", "Housing base"));
	stuff_manager.load_text(TextAsset::new("modifier_housing_multiplier", "Housing multiplier"));

	stuff_manager.load_text(TextAsset::new("modifier_job_farmer_capacity_base", "Farmer job capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_job_farmer_capacity_multiplier", "Farmer job capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_job_farmer_consumption_base", "Farmer job consumption base"));
	stuff_manager.load_text(TextAsset::new("modifier_job_farmer_consumption_multiplier", "Farmer job consumption multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_job_farmer_production_base", "Farmer job production base"));
	stuff_manager.load_text(TextAsset::new("modifier_job_farmer_production_multiplier", "Farmer job production multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_job_miner_capacity_base", "Miner job capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_job_miner_capacity_multiplier", "Miner job capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_job_miner_consumption_base", "Miner job consumption base"));
	stuff_manager.load_text(TextAsset::new("modifier_job_miner_consumption_multiplier", "Miner job consumption multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_job_miner_production_base", "Miner job production base"));
	stuff_manager.load_text(TextAsset::new("modifier_job_miner_production_multiplier", "Miner job production multiplier"));

	stuff_manager.load_text(TextAsset::new("modifier_lighthouse_examine_base", "Lighthouse examining base"));
	stuff_manager.load_text(TextAsset::new("modifier_lighthouse_gather_base", "Lighthouse scrap gathering base"));

	stuff_manager.load_text(TextAsset::new("modifier_resource_category_rawMaterial_capacity_base", "Raw material capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_category_rawMaterial_capacity_multiplier", "Raw material capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_category_mana_capacity_base", "Mana capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_category_mana_capacity_multiplier", "Mana capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_copper_capacity_base", "Copper capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_copper_capacity_multiplier", "Copper capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_copper_consumption_base", "Copper consumption base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_copper_consumption_multiplier", "Copper consumption multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_copper_production_multiplier", "Copper production multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_copper_production_base", "Copper production base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_food_capacity_base", "Food capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_food_capacity_multiplier", "Food capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_food_consumption_base", "Food consumption base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_food_consumption_multiplier", "Food consumption multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_food_production_base", "Food production base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_food_production_multiplier", "Food production multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_iron_capacity_base", "Iron capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_iron_capacity_multiplier", "Iron capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_iron_consumption_base", "Iron consumption base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_iron_consumption_multiplier", "Iron consumption multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_iron_production_base", "Iron production base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_iron_production_multiplier", "Iron production multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_knowledge_capacity_base", "Knowledge capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_knowledge_capacity_multiplier", "Knowledge capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_knowledge_consumption_base", "Knowledge consumption base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_knowledge_consumption_multiplier", "Knowledge consumption multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_knowledge_production_base", "Knowledge production base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_knowledge_production_multiplier", "Knowledge production multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_science_capacity_base", "Science capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_science_capacity_multiplier", "Science capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_science_consumption_base", "Science consumption base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_science_consumption_multiplier", "Science consumption multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_science_production_base", "Science production base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_science_production_multiplier", "Science production multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_stone_capacity_base", "Stone capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_stone_capacity_multiplier", "Stone capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_stone_consumption_base", "Stone consumption base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_stone_consumption_multiplier", "Stone consumption multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_stone_production_base", "Stone production base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_stone_production_multiplier", "Stone production multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_wood_capacity_base", "Wood capacity base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_wood_capacity_multiplier", "Wood capacity multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_wood_consumption_base", "Wood consumption base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_wood_consumption_multiplier", "Wood consumption multiplier"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_wood_production_base", "Wood production base"));
	stuff_manager.load_text(TextAsset::new("modifier_resource_wood_production_multiplier", "Wood production multiplier"));

	// Resources.

	stuff_manager.load_text(TextAsset::new("resource_category_rawMaterial", "Raw Material"));
	stuff_manager.load_text(TextAsset::new("resource_category_mana", "Mana"));

	stuff_manager.load_text(TextAsset::new("resource_copper", "Copper"));
	stuff_manager.load_text(TextAsset::new("resource_culture", "Culture"));
	stuff_manager.load_text(TextAsset::new("resource_food", "Food"));
	stuff_manager.load_text(TextAsset::new("resource_iron", "Iron"));
	stuff_manager.load_text(TextAsset::new("resource_knowledge", "Knowledge"));
	stuff_manager.load_text(TextAsset::new("resource_science", "Science"));
	stuff_manager.load_text(TextAsset::new("resource_stone", "Stone"));
	stuff_manager.load_text(TextAsset::new("resource_wood", "Wood"));

	// Stats.

	stuff_manager.load_text(TextAsset::new("stat_category_lighthouse", "Lighthouse"));
	stuff_manager.load_text(TextAsset::new("stat_category_misc", "Misc"));

	stuff_manager.load_text(TextAsset::new("stat_lighthouse_examined_title", "Lighthouse examined"));
	stuff_manager.load_text(TextAsset::new("stat_lighthouse_gathered_title", "Lighthouse scraps gathered"));
	stuff_manager.load_text(TextAsset::new("stat_lighthouse_lightsout_title", "Lights out"));
	stuff_manager.load_text(TextAsset::new("stat_lighthouse_search_title", "Lighthouse survivor searched"));

	stuff_manager.load_text(TextAsset::new("stat_debug_title", "Debug"));
	stuff_manager.load_text(TextAsset::new("stat_game_booted_title", "Game booted"));
	stuff_manager.load_text(TextAsset::new("stat_game_ticks_current_title", "Game ticks (current)"));
	stuff_manager.load_text(TextAsset::new("stat_game_ticks_total_title", "Game ticks (total)"));

	// Technologies.

	stuff_manager.load_text(TextAsset::new("technology_agriculture_title", "Agriculture"));
	stuff_manager.load_text(TextAsset::new("technology_housing_basic_title", "Housing"));
	stuff_manager.load_text(TextAsset::new("technology_lighthouse_title", "Lighthouse"));
	stuff_manager.load_text(TextAsset::new("technology_mining_title", "Mining"));
	stuff_manager.load_text(TextAsset::new("technology_woodworking_title", "Woodworking"));
	stuff_manager.load_text(TextAsset::new("technology_workbench_title", "Workbench"));
	
	stuff_manager.load_text(TextAsset::new("technology_agriculture_description", "Develops a way to grow platns using the lighthouse."));
	stuff_manager.load_text(TextAsset::new("technology_housing_basic_description", "Allows constuction of tent to shelter people."));
	stuff_manager.load_text(TextAsset::new("technology_lighthouse_description", "A structure that keeps the void away."));
	stuff_manager.load_text(TextAsset::new("technology_mining_description", "Extracts minerals from the earth."));
	stuff_manager.load_text(TextAsset::new("technology_woodworking_description", "Crafts various tools from wood."));
	stuff_manager.load_text(TextAsset::new("technology_workbench_description", "Enables upgrades."));

	// Upgrades.

	stuff_manager.load_text(TextAsset::new("upgrade_lighthouse_examine_title", "Knowledge gathering"));
	stuff_manager.load_text(TextAsset::new("upgrade_lighthouse_gather_title", "Efficient gathering"));
	stuff_manager.load_text(TextAsset::new("upgrade_tool_axe_wood_title", "Wooden axe"));
	stuff_manager.load_text(TextAsset::new("upgrade_tool_hoe_wood_title", "Wooden hoe"));
	stuff_manager.load_text(TextAsset::new("upgrade_tool_pickaxe_wood_title", "Wooden pickaxe"));

	stuff_manager.load_text(TextAsset::new("upgrade_lighthouse_examine_description", "Gains knowledge and extra science from examining the lighthouse."));
	stuff_manager.load_text(TextAsset::new("upgrade_lighthouse_gather_description", "Gains more stuffs from gathering scraps."));
	stuff_manager.load_text(TextAsset::new("upgrade_tool_axe_wood_description", "Increases woodcutter production by +10%."));
	stuff_manager.load_text(TextAsset::new("upgrade_tool_hoe_wood_description", "Increases farmer production by +10%."));
	stuff_manager.load_text(TextAsset::new("upgrade_tool_pickaxe_wood_description", "Increases miner production by +10%."));

	// UIs.

	stuff_manager.load_text(TextAsset::new("ui_tab_building", "Building"));

	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse", "Lighthouse"));
	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse_button_examine", "Examine the lighthouse"));
	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse_button_gather", "Gather scraps"));
	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse_button_lightsout", "Lights out"));
	stuff_manager.load_text(TextAsset::new("ui_tab_lighthouse_button_search", "Search for survivors"));

	stuff_manager.load_text(TextAsset::new("ui_tab_stats", "Stats"));

	stuff_manager.load_text(TextAsset::new("ui_tab_technology", "Technology"));

	stuff_manager.load_text(TextAsset::new("ui_tab_upgrade", "Upgrade"));

}
