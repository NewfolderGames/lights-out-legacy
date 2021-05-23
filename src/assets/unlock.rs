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
			Unlockable::Building("building_stockpile"),
			Unlockable::Feature("feature_lighthouse_gather"),
			Unlockable::Feature("feature_tab_stats"),
			Unlockable::Feature("feature_tab_building"),
			Unlockable::Resource("resource_wood"),
			Unlockable::Resource("resource_stone"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_quest_gather",
		vec![
			Unlockable::Building("building_researchBench"),
			Unlockable::Feature("feature_tab_technology"),
			Unlockable::Resource("resource_science"),
			Unlockable::Technology("technology_agriculture"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_agriculture",
		vec![
			Unlockable::Building("building_garden"),
			Unlockable::Resource("resource_food"),
			Unlockable::Technology("technology_woodworking"),
			Unlockable::Technology("technology_mining"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_mining",
		vec![
			Unlockable::Technology("technology_copper"),
			Unlockable::Technology("technology_stoneCutting"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_woodworking",
		vec![
			Unlockable::Technology("technology_carpentry"),
			Unlockable::Upgrade("upgrade_tool_pickaxe_wood"),
			Unlockable::Upgrade("upgrade_tool_axe_wood"),
		]
	));

}