use crate::game::stuff::{ StuffManager, UnlockAsset, Unlockable };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_default",
		vec![
			Unlockable::Feature("feature_tab_lighthouse"),
			Unlockable::Feature("feature_lighthouse_examine"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_quest_exmaine",
		vec![
			Unlockable::Feature("feature_lighthouse_gather"),
			Unlockable::Feature("feature_tab_stats"),
			Unlockable::Resource("resource_wood"),
			Unlockable::Resource("resource_stone"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_quest_gather",
		vec![
			Unlockable::Feature("feature_tab_technology"),
			Unlockable::Resource("resource_science"),
			Unlockable::Technology("technology_lighthouse"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_lighthouse",
		vec![
			Unlockable::Feature("feature_tab_building"),
			Unlockable::Building("building_researchBench"),
			Unlockable::Building("building_stockpile"),
			Unlockable::Technology("technology_agriculture"),
			Unlockable::Technology("technology_housing_basic"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_agriculture",
		vec![
			Unlockable::Building("building_garden"),
			Unlockable::Resource("resource_food"),
			Unlockable::Technology("technology_workbench"),
			Unlockable::Technology("technology_woodworking"),
			Unlockable::Technology("technology_mining"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_housing_basic",
		vec![
			Unlockable::Building("building_tent"),
			Unlockable::Feature("feature_lighthouse_search"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_workbench",
		vec![
			Unlockable::Building("building_workbench"),
			Unlockable::Feature("feature_tab_upgrade"),
			Unlockable::Resource("resource_knowledge"),
			Unlockable::Upgrade("upgrade_lighthouse_examine"),
			Unlockable::Upgrade("upgrade_lighthouse_gather"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_mining",
		vec![
			Unlockable::Resource("resource_ore"),
			Unlockable::Technology("technology_smelting"),
			Unlockable::Technology("technology_stoneCutting"),
			Unlockable::Upgrade("upgrade_building_stockpile_ore_capacity_base"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_woodworking",
		vec![
			Unlockable::Technology("technology_carpentry"),
			Unlockable::Upgrade("upgrade_tool_axe_wood"),
			Unlockable::Upgrade("upgrade_tool_hoe_wood"),
			Unlockable::Upgrade("upgrade_tool_pickaxe_wood"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_smelting",
		vec![
			Unlockable::Building("building_furnace"),
			Unlockable::Resource("resource_copper"),
			Unlockable::Technology("unlock_technology_ironWorking"),
			Unlockable::Upgrade("upgrade_building_stockpile_capacity_copper"),
			Unlockable::Upgrade("upgrade_tool_axe_copper"),
			Unlockable::Upgrade("upgrade_tool_hoe_copper"),
			Unlockable::Upgrade("upgrade_tool_pickaxe_copper"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_stoneCutting",
		vec![
			Unlockable::Upgrade("upgrade_tool_axe_stone"),
			Unlockable::Upgrade("upgrade_tool_hoe_stone"),
			Unlockable::Upgrade("upgrade_tool_pickaxe_stone"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_ironWorking",
		vec![
			Unlockable::Resource("resource_iron"),
			Unlockable::Upgrade("upgrade_building_smelter_production_iron"),
			Unlockable::Upgrade("upgrade_tool_axe_iron"),
			Unlockable::Upgrade("upgrade_tool_hoe_iron"),
			Unlockable::Upgrade("upgrade_tool_pickaxe_iron"),
		]
	));

}