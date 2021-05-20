pub struct ModifierAsset {

	pub name: &'static str,

}

impl ModifierAsset {

	pub fn new(name: &'static str) -> Self {

		Self {

			name,

		}

	}

}