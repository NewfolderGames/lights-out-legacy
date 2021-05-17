pub struct ResourceAsset {

	pub name: &'static str,
	pub title: &'static str,
	pub description: &'static str,
	pub image: &'static str,

	pub category: &'static str,

	pub capacity: Box<dyn Fn() -> f64>,
	pub production: Box<dyn Fn() -> f64>,

}

impl ResourceAsset {

	pub fn new(name: &'static str, title: &'static str, description: &'static str, image: &'static str, category: &'static str, capacity: Box<dyn Fn() -> f64>, production: Box<dyn Fn() -> f64>) -> Self {

		Self {

			name,
			title,
			description,
			image,
			category,
			capacity,
			production

		}

	}

}