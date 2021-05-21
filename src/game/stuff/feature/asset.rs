pub struct FeatureAsset {

	pub name: &'static str

}

impl FeatureAsset {

	pub fn new(name: &'static str) -> Self {

		Self {

			name

		}

	}

}