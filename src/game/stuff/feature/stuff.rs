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

	pub fn reset(&mut self) {
		
		self.is_unlocked = false;
		
	}

	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}