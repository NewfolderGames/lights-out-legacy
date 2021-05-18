pub struct ModifierAsset {

	pub name: &'static str,
	pub title: &'static str,

}

impl ModifierAsset {

	pub fn new(name: &'static str, title: &'static str) -> Self {

		Self {

			name,
			title,

		}

	}

}