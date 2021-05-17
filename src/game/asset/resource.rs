pub struct ResourceAsset {

	pub name: &'static str,
	pub title: &'static str,
	pub description: &'static str,
	pub image: &'static str,

	pub category: &'static str,

	pub default_capacity: f64,

}

impl ResourceAsset {

	pub fn new(name: &'static str, title: &'static str, description: &'static str, image: &'static str, category: &'static str, default_capacity: f64) -> Self {

		Self {

			name,
			title,
			description,
			image,
			category,
			default_capacity,

		}

	}

}