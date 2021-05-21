use crate::game::stuff::{ StuffManager, FeatureAsset };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_feature(FeatureAsset::new("feature_tab_building"));
	stuff_manager.load_feature(FeatureAsset::new("feature_tab_lighthouse"));
	stuff_manager.load_feature(FeatureAsset::new("feature_tab_technology"));

	stuff_manager.load_feature(FeatureAsset::new("feature_lighthouse_examine"));
	stuff_manager.load_feature(FeatureAsset::new("feature_lighthouse_gatherDebris"));
	
}
