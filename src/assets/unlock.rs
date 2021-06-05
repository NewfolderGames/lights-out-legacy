use crate::game::stuff::StuffManager;
use crate::game::stuff::unlock::{ UnlockAsset, Unlockable };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_unlock(UnlockAsset::new(
		"default",
		vec![
			Unlockable::Feature("tab_lighthouse"),
			Unlockable::Feature("tab_lighthouse_examine"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"quest_exmaine",
		vec![
			Unlockable::Feature("tab_lighthouse_gather"),
			Unlockable::Feature("tab_stats"),
			Unlockable::Resource("wood"),
			Unlockable::Resource("stone"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"quest_gather",
		vec![
			Unlockable::Feature("tab_technology"),
			Unlockable::Resource("science"),
			Unlockable::Technology("lighthouse"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"technology_lighthouse",
		vec![
			Unlockable::Feature("tab_building"),
			Unlockable::Building("researchBench"),
			Unlockable::Building("stockpile"),
			Unlockable::Technology("agriculture"),
			Unlockable::Technology("housing_basic"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"technology_agriculture",
		vec![
			Unlockable::Building("garden"),
			Unlockable::Resource("food"),
			Unlockable::Technology("tools_simple"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"technology_housing_basic",
		vec![
			Unlockable::Building("tent"),
			Unlockable::Feature("tab_lighthouse_search"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"technology_tools_simple",
		vec![
			Unlockable::Building("workbench"),
			Unlockable::Feature("tab_upgrade"),
			Unlockable::Resource("knowledge"),
			Unlockable::Technology("woodworking"),
			Unlockable::Technology("mining"),
			Unlockable::Upgrade("lighthouse_examine"),
			Unlockable::Upgrade("lighthouse_gather"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"technology_mining",
		vec![
			Unlockable::Resource("ore"),
			Unlockable::Technology("smelting"),
			Unlockable::Technology("stoneCutting"),
			Unlockable::Upgrade("stockpile_ore_capacity_base"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"technology_woodworking",
		vec![
			Unlockable::Technology("carpentry"),
			Unlockable::Upgrade("tool_axe_wood"),
			Unlockable::Upgrade("tool_hoe_wood"),
			Unlockable::Upgrade("tool_pickaxe_wood"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"technology_smelting",
		vec![
			Unlockable::Building("furnace"),
			Unlockable::Resource("copper"),
			Unlockable::Technology("ironWorking"),
			Unlockable::Upgrade("stockpile_capacity_copper"),
			Unlockable::Upgrade("tool_axe_copper"),
			Unlockable::Upgrade("tool_hoe_copper"),
			Unlockable::Upgrade("tool_pickaxe_copper"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"technology_stoneCutting",
		vec![
			Unlockable::Upgrade("tool_axe_stone"),
			Unlockable::Upgrade("tool_hoe_stone"),
			Unlockable::Upgrade("tool_pickaxe_stone"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"technology_ironWorking",
		vec![
			Unlockable::Resource("iron"),
			Unlockable::Upgrade("smelter_production_iron"),
			Unlockable::Upgrade("tool_axe_iron"),
			Unlockable::Upgrade("tool_hoe_iron"),
			Unlockable::Upgrade("tool_pickaxe_iron"),
		]
	));

}
