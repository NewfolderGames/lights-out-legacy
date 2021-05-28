use crate::game::stuff::{ UpgradeAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// DO NOT CHANGE OUTPUT MODIFIER LIST OR PRICE LIST ON RUNTIME.
	// ONLY VALUES SHOULD BE CHANGED.

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_lighthouse_examine",
		Box::new(|_| {
			vec![
				("modifier_lighthouse_examine_base", 1f64)
			]
		}),
		Box::new(|_| {
			vec![
				("resource_science", 50f64),
			]
		})
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_lighthouse_gather",
		Box::new(|_| {
			vec![
				("modifier_lighthouse_gather_base", 1f64)
			]
		}),
		Box::new(|_| {
			vec![
				("resource_knowledge", 10f64),
				("resource_wood", 50f64),
			]
		})
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_pickaxe_wood",
		Box::new(|_| {
			vec![
				("modifier_resource_stone_production_multiplier", 0.1)
			]
		}),
		Box::new(|_| {
			vec![
				("resource_knowledge", 50f64),
				("resource_wood", 50f64),
			]
		})
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_axe_wood",
		Box::new(|_| {
			vec![
				("modifier_resource_wood_production_multiplier", 0.1)
			]
		}),
		Box::new(|_| {
			vec![
				("resource_knowledge", 50f64),
				("resource_wood", 50f64),
			]
		})
	));

}