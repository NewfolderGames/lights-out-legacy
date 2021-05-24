use crate::game::stuff::{ StatAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// Lighthouse.

	stuff_manager.load_stat(StatAsset::new("stat_lighthouse_lightsout", "lighthouse"));
	stuff_manager.load_stat(StatAsset::new("stat_lighthouse_examined", "lighthouse"));
	stuff_manager.load_stat(StatAsset::new("stat_lighthouse_gathered", "lighthouse"));

	// Misc.

	stuff_manager.load_stat(StatAsset::new("stat_debug", "misc"));
	stuff_manager.load_stat(StatAsset::new("stat_game_booted", "misc"));
	stuff_manager.load_stat(StatAsset::new("stat_game_ticks_current", "misc"));
	stuff_manager.load_stat(StatAsset::new("stat_game_ticks_total", "misc"));

}