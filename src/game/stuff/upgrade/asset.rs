/// An upgrade asset.
pub struct UpgradeAsset {

	pub name: &'static str,

	pub modifiers:  Vec<(&'static str, f64)>,
	pub price: Vec<(&'static str, f64)>,

}

impl UpgradeAsset {

	/// Creates a new upgrade asset.
	pub fn new(
		name: &'static str,
		modifiers: Vec<(&'static str, f64)>,
		price: Vec<(&'static str, f64)>
	) -> Self {

		Self {

			name,
			modifiers,
			price,

		}

	}

}
