pub struct StatAsset {

	pub name: &'static str,
	pub category: &'static str,

}

impl StatAsset {

	pub fn new(name: &'static str, category: &'static str) -> Self {

		Self {

			name,
			category

		}

	}

}