use crate::game::stuff::{ UpgradeAsset, StuffManager };

pub fn load(stuff_manager: &mut StuffManager) {

	// DO NOT CHANGE OUTPUT MODIFIER LIST OR PRICE LIST ON RUNTIME.
	// ONLY VALUES SHOULD BE CHANGED.

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_lighthouse_examine",
		vec![
			("modifier_lighthouse_examine_base", 1f64)
		],
		vec![
			("resource_science", 10f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_lighthouse_gather",
		vec![
			("modifier_lighthouse_gather_base", 1f64)
		],
		vec![
			("resource_knowledge", 20f64),
			("resource_wood", 20f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_building_stockpile_ore_capacity_base",
		vec![
			("modifier_building_stockpile_ore_capacity_base", 10f64)
		],
		vec![
			("resource_knowledge", 15f64),
			("resource_wood", 40f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_pickaxe_wood",
		vec![
			("modifier_job_miner_production_multiplier", 0.1)
		],
		vec![
			("resource_knowledge", 30f64),
			("resource_wood", 30f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_axe_wood",
		vec![
			("modifier_job_woodcutter_production_multiplier", 0.1)
		],
		vec![
			("resource_knowledge", 30f64),
			("resource_wood", 30f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_hoe_wood",
		vec![
			("modifier_job_farmer_production_multiplier", 0.1)
		],
		vec![
			("resource_knowledge", 30f64),
			("resource_wood", 30f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_pickaxe_stone",
		vec![
			("modifier_job_miner_production_multiplier", 0.2)
		],
		vec![
			("resource_knowledge", 50f64),
			("resource_stone", 50f64),
			("resource_wood", 50f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_axe_stone",
		vec![
			("modifier_job_woodcutter_production_multiplier", 0.2)
		],
		vec![
			("resource_knowledge", 50f64),
			("resource_stone", 50f64),
			("resource_wood", 50f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_hoe_stone",
		vec![
			("modifier_job_farmer_production_multiplier", 0.2)
		],
		vec![
			("resource_knowledge", 50f64),
			("resource_stone", 50f64),
			("resource_wood", 50f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_pickaxe_copper",
		vec![
			("modifier_job_miner_production_multiplier", 0.3)
		],
		vec![
			("resource_copper", 50f64),
			("resource_knowledge", 65f64),
			("resource_wood", 75f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_axe_copper",
		vec![
			("modifier_job_woodcutter_production_multiplier", 0.3)
		],
		vec![
			("resource_copper", 50f64),
			("resource_knowledge", 65f64),
			("resource_wood", 75f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_hoe_copper",
		vec![
			("modifier_job_farmer_production_multiplier", 0.3)
		],
		vec![
			("resource_copper", 50f64),
			("resource_knowledge", 65f64),
			("resource_wood", 75f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_pickaxe_iron",
		vec![
			("modifier_job_miner_production_multiplier", 0.4)
		],
		vec![
			("resource_iron", 50f64),
			("resource_knowledge", 65f64),
			("resource_wood", 75f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_axe_iron",
		vec![
			("modifier_job_woodcutter_production_multiplier", 0.4)
		],
		vec![
			("resource_iron", 50f64),
			("resource_knowledge", 65f64),
			("resource_wood", 75f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"upgrade_tool_hoe_iron",
		vec![
			("modifier_job_farmer_production_multiplier", 0.4)
		],
		vec![
			("resource_iron", 50f64),
			("resource_knowledge", 65f64),
			("resource_wood", 75f64),
		]
	));

}