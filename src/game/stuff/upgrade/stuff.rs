use super::UpgradeAsset;

pub struct Upgrade {

	asset: UpgradeAsset,

	is_upgraded: bool,

	is_unlocked: bool,

}

impl Upgrade {

	pub fn new(asset: UpgradeAsset) -> Self {

		Self {

			asset,
			is_upgraded: false,
			is_unlocked: false

		}

	}

	pub fn get_asset(&self) -> &UpgradeAsset {

		&self.asset

	}

	pub fn is_upgraded(&self) -> bool {

		self.is_upgraded

	}

	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	pub fn upgrade(&mut self) {

		self.is_upgraded = true;

	}

	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}