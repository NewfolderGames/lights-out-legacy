use std::rc::Rc;
use super::ModifierData;

pub struct Modifier {

	data: Rc<ModifierData>,

	value: f64,

	is_dirty: bool,

}

// Constructor.

impl Modifier {

	/// Creates a new modifier.
	pub fn new(data: Rc<ModifierData>) -> Self {

		let value = data.default_value;

		Self {

			data,
			value,
			is_dirty: true,

		}

	}

}

// Numbers.

impl Modifier {

	/// Resets value to default.
	pub fn reset(&mut self) {

		self.value = self.data.default_value;

	}

	/// Returns the value.
	pub fn value(&self) -> f64 {

		self.value

	}

}

// Other.

impl Modifier {

	/// Returns modifier data.
	pub fn data(&self) -> Rc<ModifierData> {

		self.data.clone()

	}

	/// Checks whether value changed or not.
	pub fn is_dirty(&self) -> bool {

		self.is_dirty

	}

	/// Clears dirty state.
	pub fn clear_dirty(&mut self) {

		self.is_dirty = false;

	}

}