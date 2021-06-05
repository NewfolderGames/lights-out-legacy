
/// A resource asset.
pub struct ResourceAsset {

	pub name: &'static str,
	pub category: &'static str,
	pub capacity: f64,

}

impl ResourceAsset {

	/// Creates a new resource asset.
	pub fn new(name: &'static str, category: &'static str, capacity: f64) -> Self {

		Self {

			name,
			category,
			capacity

		}

	}

}