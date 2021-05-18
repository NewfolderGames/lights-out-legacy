use crate::game::stuff::{ StuffManager, UnlockAsset, Unlockable };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_unlock(UnlockAsset::new(
		"unlock_default",
		vec![
			Unlockable::Building("building_stockpile"),
			Unlockable::Resource("resource_wood"),
			Unlockable::Resource("resource_stone"),
		]
	))

}