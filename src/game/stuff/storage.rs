use std::any::Any;
use std::collections::hash_map::Iter;
use super::Stuff;

/// A stuff storage.
pub trait StuffStorage<S> : Any where S: Stuff {

	/// Creates a new storage.
	fn new() -> Self;

	/// Returns a reference to a stuff.
	fn get(&self, name: &str) -> Option<&S>;

	/// Returns a reference to a stuff's asset.
	fn get_asset(&self, name: &str) -> Option<&S::Asset>;

	/// Returns a mutable reference to a stuff.
	fn get_mut(&mut self, name: &str) -> Option<&mut S>;

	/// Returns an iterator.
	fn iter(&self) -> Iter<String, S>;

	/// Loads an asset into the storage.
	fn load(&mut self, asset: S::Asset);

	/// Resets the storage.
	fn reset(&mut self);

}