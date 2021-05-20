pub struct UpgradeAsset {

	pub name: &'static str,

	pub price: Vec<(&'static str, f64)>,
	pub modifiers: Vec<(&'static str, f64)>,

}

impl UpgradeAsset {

	pub fn new(name: &'static str, price: Vec<(&'static str, f64)>, modifiers: Vec<(&'static str, f64)>) -> Self {

		Self {

			name,
			price,
			modifiers

		}

	}

}