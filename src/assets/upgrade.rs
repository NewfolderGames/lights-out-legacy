use crate::game::stuff::{ UpgradeAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_pickaxe_wood",
		vec![
			("resource_knowledge", 10f64),
			("resource_wood", 50f64),
		],
		vec![
			("resource_stone_production_multiplier", 0.1)
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_axe_wood",
		vec![
			("resource_knowledge", 10f64),
			("resource_wood", 50f64),
		],
		vec![
			("resource_wood_production_multiplier", 0.1)
		]
	));

}