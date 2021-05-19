use super::ModifierAsset;

pub struct Modifier {

	asset: ModifierAsset,

	value: f64,

}

impl Modifier {

	pub fn new(asset: ModifierAsset) -> Self {

		Self {

			asset,
			value: 0f64,

		}

	}

	pub fn get_asset(&self) -> &ModifierAsset {

		&self.asset

	}

	pub fn get_value(&self) -> f64 {

		self.value

	}

	pub fn set_value(&mut self, value: f64) {

		self.value = value;

	}

}
