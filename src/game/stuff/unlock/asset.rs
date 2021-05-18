use super::Unlockable;

pub struct UnlockAsset {

	pub name: &'static str,
	pub unlocks: Vec<Unlockable>,

}

impl UnlockAsset {

	pub fn new(name: &'static str, unlocks: Vec<Unlockable>) -> Self {

		Self {

			name,
			unlocks

		}

	}

}
