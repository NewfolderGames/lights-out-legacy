use super::FeatureAsset;

pub struct Feature {

	asset: FeatureAsset,

	is_unlocked: bool,

}

impl Feature {

	pub fn new(asset: FeatureAsset) -> Self {

		Self {

			asset,
			is_unlocked: false

		}

	}

	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	pub fn set_unlock(&mut self, unlock: bool) {

		self.is_unlocked = unlock;

	}

}