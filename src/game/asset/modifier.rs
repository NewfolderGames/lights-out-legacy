pub struct ModifierAsset {

	pub name: &'static str,
	pub title: &'static str,

	pub default_value: f64,

}

impl ModifierAsset {

	pub fn new(name: &'static str, title: &'static str, default_value: f64) -> Self {

		Self {

			name,
			title,
			default_value

		}

	}

}