use super::StatAsset;

pub struct Stat {

	asset: StatAsset,

	value: f64,

}

impl Stat {

	pub fn new(asset: StatAsset) -> Self {

		Self {

			asset,
			value: 0f64,

		}

	}

	pub fn get_asset(&self) -> &StatAsset {

		&self.asset

	}

	pub fn add(&mut self, amount: f64) {

		self.value += amount;

	}

	pub fn get_value(&self) -> f64 {

		self.value

	}

	pub fn reset(&mut self) {

		self.value = 0f64;

	}

	pub fn set_value(&mut self, value: f64) {

		self.value = value;

	}

}
