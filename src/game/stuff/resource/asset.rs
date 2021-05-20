pub struct ResourceAsset {

	pub name: &'static str,

	pub category: &'static str,
	pub capacity: f64,

}

impl ResourceAsset {

	pub fn new(name: &'static str, category: &'static str, capacity: f64) -> Self {

		Self {

			name,
			category,
			capacity,

		}

	}

}