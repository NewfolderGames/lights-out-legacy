use crate::game::stuff::{ StuffManager };
use crate::game::stuff::upgrade::UpgradeAsset;

pub fn load(stuff_manager: &mut StuffManager) {

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"lighthouse_examine",
		vec![
			("lighthouse_examine_base", 1f64)
		],
		vec![
			("science", 10f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"lighthouse_gather",
		vec![
			("lighthouse_gather_base", 1f64)
		],
		vec![
			("knowledge", 20f64),
			("wood", 20f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"building_stockpile_ore_capacity_base",
		vec![
			("building_stockpile_ore_capacity_base", 10f64)
		],
		vec![
			("knowledge", 15f64),
			("wood", 40f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_pickaxe_wood",
		vec![
			("job_miner_production_multiplier", 0.1)
		],
		vec![
			("knowledge", 30f64),
			("wood", 30f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_axe_wood",
		vec![
			("job_woodcutter_production_multiplier", 0.1)
		],
		vec![
			("knowledge", 30f64),
			("wood", 30f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_hoe_wood",
		vec![
			("job_farmer_production_multiplier", 0.1)
		],
		vec![
			("knowledge", 30f64),
			("wood", 30f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_pickaxe_stone",
		vec![
			("job_miner_production_multiplier", 0.2)
		],
		vec![
			("knowledge", 50f64),
			("stone", 50f64),
			("wood", 50f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_axe_stone",
		vec![
			("job_woodcutter_production_multiplier", 0.2)
		],
		vec![
			("knowledge", 50f64),
			("stone", 50f64),
			("wood", 50f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_hoe_stone",
		vec![
			("job_farmer_production_multiplier", 0.2)
		],
		vec![
			("knowledge", 50f64),
			("stone", 50f64),
			("wood", 50f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_pickaxe_copper",
		vec![
			("job_miner_production_multiplier", 0.3)
		],
		vec![
			("copper", 50f64),
			("knowledge", 65f64),
			("wood", 75f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_axe_copper",
		vec![
			("job_woodcutter_production_multiplier", 0.3)
		],
		vec![
			("copper", 50f64),
			("knowledge", 65f64),
			("wood", 75f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_hoe_copper",
		vec![
			("job_farmer_production_multiplier", 0.3)
		],
		vec![
			("copper", 50f64),
			("knowledge", 65f64),
			("wood", 75f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_pickaxe_iron",
		vec![
			("job_miner_production_multiplier", 0.4)
		],
		vec![
			("iron", 50f64),
			("knowledge", 65f64),
			("wood", 75f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_axe_iron",
		vec![
			("job_woodcutter_production_multiplier", 0.4)
		],
		vec![
			("iron", 50f64),
			("knowledge", 65f64),
			("wood", 75f64),
		]
	));

	stuff_manager.load_upgrade(UpgradeAsset::new(
		"tool_hoe_iron",
		vec![
			("job_farmer_production_multiplier", 0.4)
		],
		vec![
			("iron", 50f64),
			("knowledge", 65f64),
			("wood", 75f64),
		]
	));

}