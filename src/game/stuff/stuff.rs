use std::any::Any;
use super::{ StuffAsset, StuffStorage };

pub trait Stuff : Any + Sized {

	type Asset: StuffAsset;
	type Storage: StuffStorage<Self>;

	/// Creates a new stuff.
	fn new(asset: Self::Asset) -> Self;

	/// Returns the stuff's asset.
	fn get_asset(&self) -> &Self::Asset;

	/// Resets the stuff to original state.
	fn reset(&mut self);

}