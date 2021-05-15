use std::rc::Rc;
use crate::game::asset::ModifierAsset;

pub struct Modifier {

	asset: Rc<ModifierAsset>,

	value: f64,

	is_dirty: bool,

}

impl Modifier {

	pub fn new(asset: Rc<ModifierAsset>) -> Self {

		let value = asset.default_value;

		Self {

			asset,
			value,
			is_dirty: true,

		}

	}

	pub fn clear_dirty(&mut self) {

		self.is_dirty = false;

	}

	pub fn get_asset(&self) -> Rc<ModifierAsset> {

		self.asset.clone()

	}

	pub fn get_value(&self) -> f64 {

		self.value

	}

	pub fn is_dirty(&self) -> bool {

		self.is_dirty

	}

	pub fn reset(&mut self) {

		self.value = self.asset.default_value;
		self.is_dirty = true;

	}

}