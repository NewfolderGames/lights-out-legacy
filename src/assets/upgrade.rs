use crate::game::stuff::{ UpgradeAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_pickaxe_wood",
		Box::new(|_| {
			vec![
				("resource_knowledge", 10f64),
				("resource_wood", 50f64),
			]
		}),
		Box::new(|_| {
			vec![
				("resource_stone_production_multiplier", 0.1)
			]
		})
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_axe_wood",
		Box::new(|_| {
			vec![
				("resource_knowledge", 10f64),
				("resource_wood", 50f64),
			]
		}),
			Box::new(|_| {
			vec![
				("resource_wood_production_multiplier", 0.1)
			]
		})
	));

}