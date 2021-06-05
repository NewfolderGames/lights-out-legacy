use crate::game::stuff::StuffManager;

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_feature("tab_building");
	stuff_manager.load_feature("tab_lighthouse");
	stuff_manager.load_feature("tab_lighthouse_examine");
	stuff_manager.load_feature("tab_lighthouse_gather");
	stuff_manager.load_feature("tab_lighthouse_lightsout");
	stuff_manager.load_feature("tab_lighthouse_search");
	stuff_manager.load_feature("tab_stats");
	stuff_manager.load_feature("tab_technology");
	stuff_manager.load_feature("tab_upgrade");
	
}
