use super::{ UnlockAsset, UnlockManager };
use super::super::Stuff;

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

}

impl Stuff for Unlock {

	type Asset = UnlockAsset;
	type Manager = UnlockManager;

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

}