use super::Unlockable;

/// An unlock asset.
pub struct UnlockAsset {

	pub name: &'static str,
	pub unlocks: Vec<Unlockable>,

}

impl UnlockAsset {

	/// Creates a new unlock asset.
	pub fn new(name: &'static str, unlocks: Vec<Unlockable>) -> Self {

		Self {

			name,
			unlocks

		}

	}

}
