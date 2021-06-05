use crate::game::stuff::StuffManager;

pub fn load(stuff_manager: &mut StuffManager) {

	// Building - All
	
	stuff_manager.load_modifier("building_stockpile_ore_capacity_base");

	// Jobs.

	stuff_manager.load_modifier("job_farmer_capacity_base");
	stuff_manager.load_modifier("job_farmer_capacity_multiplier");
	stuff_manager.load_modifier("job_farmer_consumption_base");
	stuff_manager.load_modifier("job_farmer_consumption_multiplier");
	stuff_manager.load_modifier("job_farmer_production_base");
	stuff_manager.load_modifier("job_farmer_production_multiplier");

	stuff_manager.load_modifier("job_miner_capacity_base");
	stuff_manager.load_modifier("job_miner_capacity_multiplier");
	stuff_manager.load_modifier("job_miner_consumption_base");
	stuff_manager.load_modifier("job_miner_consumption_multiplier");
	stuff_manager.load_modifier("job_miner_production_base");
	stuff_manager.load_modifier("job_miner_production_multiplier");

	stuff_manager.load_modifier("job_woodcutter_capacity_base");
	stuff_manager.load_modifier("job_woodcutter_capacity_multiplier");
	stuff_manager.load_modifier("job_woodcutter_consumption_base");
	stuff_manager.load_modifier("job_woodcutter_consumption_multiplier");
	stuff_manager.load_modifier("job_woodcutter_production_base");
	stuff_manager.load_modifier("job_woodcutter_production_multiplier");

	// Misc.

	stuff_manager.load_modifier("housing_base");
	stuff_manager.load_modifier("housing_mutiplier");

	// Resource - Categories.

	stuff_manager.load_modifier("resource_category_rawMaterial_capacity_base");
	stuff_manager.load_modifier("resource_category_rawMaterial_capacity_multiplier");

	stuff_manager.load_modifier("resource_category_mana_capacity_base");
	stuff_manager.load_modifier("resource_category_mana_capacity_multiplier");

	// Resource - Types.

	stuff_manager.load_modifier("resource_copper_production_base");
	stuff_manager.load_modifier("resource_copper_production_multiplier");
	stuff_manager.load_modifier("resource_copper_consumption_base");
	stuff_manager.load_modifier("resource_copper_consumption_multiplier");
	stuff_manager.load_modifier("resource_copper_capacity_base");
	stuff_manager.load_modifier("resource_copper_capacity_multiplier");
	stuff_manager.load_modifier("resource_food_production_base");
	stuff_manager.load_modifier("resource_food_production_multiplier");
	stuff_manager.load_modifier("resource_food_consumption_base");
	stuff_manager.load_modifier("resource_food_consumption_multiplier");
	stuff_manager.load_modifier("resource_food_capacity_base");
	stuff_manager.load_modifier("resource_food_capacity_multiplier");
	stuff_manager.load_modifier("resource_iron_production_base");
	stuff_manager.load_modifier("resource_iron_production_multiplier");
	stuff_manager.load_modifier("resource_iron_consumption_base");
	stuff_manager.load_modifier("resource_iron_consumption_multiplier");
	stuff_manager.load_modifier("resource_iron_capacity_base");
	stuff_manager.load_modifier("resource_iron_capacity_multiplier");
	stuff_manager.load_modifier("resource_knowledge_production_base");
	stuff_manager.load_modifier("resource_knowledge_production_multiplier");
	stuff_manager.load_modifier("resource_knowledge_consumption_base");
	stuff_manager.load_modifier("resource_knowledge_consumption_multiplier");
	stuff_manager.load_modifier("resource_knowledge_capacity_base");
	stuff_manager.load_modifier("resource_knowledge_capacity_multiplier");
	stuff_manager.load_modifier("resource_ore_production_base");
	stuff_manager.load_modifier("resource_ore_production_multiplier");
	stuff_manager.load_modifier("resource_ore_consumption_base");
	stuff_manager.load_modifier("resource_ore_consumption_multiplier");
	stuff_manager.load_modifier("resource_ore_capacity_base");
	stuff_manager.load_modifier("resource_ore_capacity_multiplier");
	stuff_manager.load_modifier("resource_science_production_base");
	stuff_manager.load_modifier("resource_science_production_multiplier");
	stuff_manager.load_modifier("resource_science_consumption_base");
	stuff_manager.load_modifier("resource_science_consumption_multiplier");
	stuff_manager.load_modifier("resource_science_capacity_base");
	stuff_manager.load_modifier("resource_science_capacity_multiplier");
	stuff_manager.load_modifier("resource_stone_production_base");
	stuff_manager.load_modifier("resource_stone_production_multiplier");
	stuff_manager.load_modifier("resource_stone_consumption_base");
	stuff_manager.load_modifier("resource_stone_consumption_multiplier");
	stuff_manager.load_modifier("resource_stone_capacity_base");
	stuff_manager.load_modifier("resource_stone_capacity_multiplier");
	stuff_manager.load_modifier("resource_wood_production_base");
	stuff_manager.load_modifier("resource_wood_production_multiplier");
	stuff_manager.load_modifier("resource_wood_consumption_base");
	stuff_manager.load_modifier("resource_wood_consumption_multiplier");
	stuff_manager.load_modifier("resource_wood_capacity_base");
	stuff_manager.load_modifier("resource_wood_capacity_multiplier");

	// Lighthouse

	stuff_manager.load_modifier("lighthouse_examine_base");
	stuff_manager.load_modifier("lighthouse_gather_base");

}