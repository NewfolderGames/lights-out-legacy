use super::TechnologyAsset;

pub struct Technology {

	asset: TechnologyAsset,

	is_researched: bool,

	is_unlocked: bool,

}

impl Technology {

	pub fn new(asset: TechnologyAsset) -> Self {

		Self {

			asset,
			is_researched: false,
			is_unlocked: false

		}

	}

	pub fn is_researched(&self) -> bool {

		self.is_researched

	}

	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	pub fn research(&mut self) {

		self.is_researched = true;

	}

	pub fn reset(&mut self) {

		self.is_researched = false;
		self.is_unlocked = false;

	}

	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}