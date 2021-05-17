pub struct UnlockAsset {

	pub name: &'static str,
	pub unlocks: Vec<UnlockStuff>,

}

impl UnlockAsset {

	pub fn new(name: &'static str, unlocks: Vec<UnlockStuff>) -> Self {

		Self {

			name,
			unlocks

		}

	}

}

pub enum UnlockStuff {

	Building(&'static str),
	Resource(&'static str),

}