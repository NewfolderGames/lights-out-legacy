use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ Stuff, StuffAsset, StuffStorage };

/// A feature data.
pub struct Feature {

	asset: FeatureAsset,

	is_unlocked: bool

}

impl Feature {

	/// Returns `true` if the feature is unlocked.
	pub fn is_unlocked(&self) -> bool {

		self.is_unlocked

	}

	/// Unlocks the feature.
	pub fn unlock(&mut self) {

		self.is_unlocked = true;

	}

}

impl Stuff for Feature {

	type Asset = FeatureAsset;
	type Storage = FeatureStorage;

	fn new(asset: FeatureAsset) -> Self {

		Self {

			asset,
			is_unlocked: false,

		}

	}

	fn get_asset(&self) -> &Self::Asset {
		
		&self.asset

	}

	fn reset(&mut self) {
		
		self.is_unlocked = false;

	}

}

/// A resource asset.
pub struct FeatureAsset {

	pub name: &'static str,

}

impl FeatureAsset {

	/// Creates a new feature asset.
	pub fn new(name: &'static str) -> Self {

		Self {

			name

		}

	}

}

impl StuffAsset for FeatureAsset {

	const NAME: &'static str = "asset_feature";

}

/// A feature storage.
pub struct FeatureStorage {

	features: HashMap<String, Feature>

}

impl FeatureStorage {

	/// Unlocks a feature.
	pub fn unlock(&mut self, name: &str) {

		self.features
			.get_mut(name)
			.map(|f| f.unlock());

	}

}

impl StuffStorage<Feature> for FeatureStorage {

	fn new() -> Self {

		Self {

			features: HashMap::new(),

		}

	}

	fn get(&self, name: &str) -> Option<&Feature> {
		
		self.features.get(name)

	}

	fn get_asset(&self, name: &str) -> Option<&FeatureAsset> {
		
		self.features
			.get(name)
			.map(|b| b.get_asset())

	}

	fn get_mut(&mut self, name: &str) -> Option<&mut Feature> {
		
		self.features.get_mut(name)

	}

	fn iter(&self) -> Iter<String, Feature> {
		
		self.features.iter()

	}

	fn load(&mut self, asset: FeatureAsset) {
		
		self.features.insert(String::from(asset.name), Feature::new(asset));

	}

	fn reset(&mut self) {
		
		self.features
			.iter_mut()
			.for_each(|(_, r)| r.reset());

	}

}