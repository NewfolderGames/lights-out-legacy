use crate::game::stuff::{ StuffManager, UnlockAsset, Unlockable };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_default",
		vec![
			Unlockable::Building("building_stockpile"),
			Unlockable::Feature("feature_tab_lighthouse"),
			Unlockable::Feature("feature_lighthouse_examineLightHouse"),
			Unlockable::Resource("resource_science"),
			Unlockable::Technology("technology_lighthouse"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_lighthouse",
		vec![
			Unlockable::Building("building_stockpile"),
			Unlockable::Feature("feature_lighthouse_gatherDebris"),
			Unlockable::Resource("resource_wood"),
			Unlockable::Resource("resource_stone"),
			Unlockable::Technology("technology_agriculture"),
		]
	));

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_technology_agriculture",
		vec![
			Unlockable::Building("building_garden"),
			Unlockable::Resource("resource_food"),
			Unlockable::Technology("technology_woodCutting"),
			Unlockable::Technology("technology_mining"),
		]
	));

}