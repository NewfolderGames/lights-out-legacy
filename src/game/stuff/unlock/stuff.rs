use super::UnlockAsset;

pub struct Unlock {

	asset: UnlockAsset,

	is_unlocked: bool
	
}

impl Unlock {

	pub fn new(asset: UnlockAsset) -> Self {

		Self {

			asset,
			is_unlocked: false

		}

	}

	pub fn get_asset(&self) -> &UnlockAsset {

		&self.asset

	}

}
