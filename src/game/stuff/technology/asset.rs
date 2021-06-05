/// A technology asset.
pub struct TechnologyAsset {

	pub name: &'static str,
	pub price: Vec<(&'static str, f64)>,

}

impl TechnologyAsset {

	/// Creates a new technology asset.
	pub fn new(name: &'static str, price: Vec<(&'static str, f64)>) -> Self {

		Self {

			name,
			price,

		}

	}

}