use std::rc::Rc;
use crate::game::asset::ModifierAsset;

pub struct Modifier {

	asset: Rc<ModifierAsset>,

	value: f64,

}

impl Modifier {

	pub fn new(asset: Rc<ModifierAsset>) -> Self {

		Self {

			asset,
			value: 0f64,

		}

	}

	pub fn get_asset(&self) -> Rc<ModifierAsset> {

		self.asset.clone()

	}

	pub fn get_value(&self) -> f64 {

		self.value

	}

	pub fn set_value(&mut self, value: f64) {

		self.value = value;

	}

}