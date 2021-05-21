pub struct TextAsset {

	pub name: &'static str,
	pub text: &'static str

}

impl TextAsset {

	pub fn new(name: &'static str, text: &'static str) -> Self {

		Self {

			name,
			text

		}

	}

}