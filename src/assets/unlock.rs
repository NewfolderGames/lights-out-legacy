use crate::game::asset::{ AssetManager, UnlockAsset, UnlockStuff };

pub fn load(asset_manager: &mut AssetManager) {

	asset_manager.load_unlock(UnlockAsset::new(
		"unlock_default",
		vec![
			UnlockStuff::Building("building_stockpile"),
			UnlockStuff::Resource("resource_wood")
		]
	))

}