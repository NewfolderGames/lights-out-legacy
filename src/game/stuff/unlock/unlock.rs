use super::{ UnlockAsset, Unlockable };

/// An unlock data.
pub struct Unlock {

	asset: UnlockAsset,
	is_unlocked: bool,

}

impl Unlock {

	/// Creates a new unlock.
	pub fn new(asset: UnlockAsset) -> Self {

		Self {

			asset,
			is_unlocked: false

		}

	}

	/// Returns the unlocks.
	pub fn get_unlocks(&self) -> &Vec<Unlockable> {

		&self.asset.unlocks

	}

	/// Returns `true` if unlocked.
	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	/// Resets the unlock.
	pub fn reset(&mut self) {

		self.is_unlocked = false;

	}

	/// Unlocks the unlock.
	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}