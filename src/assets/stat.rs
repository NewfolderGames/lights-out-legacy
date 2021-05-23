use crate::game::stuff::{ StatAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Lighthouse.

	stuff_manager.load_stat(StatAsset::new("stat_lightsout", "lighthouse"));
	stuff_manager.load_stat(StatAsset::new("stat_examined", "lighthouse"));
	stuff_manager.load_stat(StatAsset::new("stat_gathered", "lighthouse"));

	// Misc.

	stuff_manager.load_stat(StatAsset::new("stat_booted_total", "misc"));
	stuff_manager.load_stat(StatAsset::new("stat_debug", "misc"));
	stuff_manager.load_stat(StatAsset::new("stat_ticks_current", "misc"));
	stuff_manager.load_stat(StatAsset::new("stat_ticks_total", "misc"));

}