use crate::game::stuff::StuffManager;

pub fn load(stuff_manager: &mut StuffManager) {

	// Lighthouse.

	stuff_manager.load_stat("lighthouse_examined", "lighthouse");
	stuff_manager.load_stat("lighthouse_gathered", "lighthouse");
	stuff_manager.load_stat("lighthouse_searched", "lighthouse");
	stuff_manager.load_stat("lighthouse_lightsout", "lighthouse");

	// Misc.

	stuff_manager.load_stat("debug", "misc");
	stuff_manager.load_stat("game_booted", "misc");
	stuff_manager.load_stat("game_ticks_current", "misc");
	stuff_manager.load_stat("game_ticks_total", "misc");

}