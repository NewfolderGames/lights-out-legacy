use std::rc::Rc;
use super::ModifierAsset;

pub struct Modifier {

	data: Rc<ModifierAsset>,

	value: f64,

	is_dirty: bool,

}

impl Modifier {

	pub fn new(data: Rc<ModifierAsset>) -> Self {

		let value = data.default_value;

		Self {

			data,
			value,
			is_dirty: true,

		}

	}

}

impl Modifier {

	pub fn reset(&mut self) {

		self.value = self.data.default_value;

	}

	pub fn value(&self) -> f64 {

		self.value

	}

}

impl Modifier {

	pub fn data(&self) -> Rc<ModifierAsset> {

		self.data.clone()

	}

	pub fn is_dirty(&self) -> bool {

		self.is_dirty

	}

	pub fn clear_dirty(&mut self) {

		self.is_dirty = false;

	}

}