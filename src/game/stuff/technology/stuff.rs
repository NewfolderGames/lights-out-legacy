use std::collections::HashMap;
use super::TechnologyAsset;

pub struct Technology {

	asset: TechnologyAsset,

	price: HashMap<String, f64>,
	is_researched: bool,

	is_unlocked: bool,

}

impl Technology {

	pub fn new(asset: TechnologyAsset) -> Self {

		Self {

			asset,
			price: HashMap::new(),
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

	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}