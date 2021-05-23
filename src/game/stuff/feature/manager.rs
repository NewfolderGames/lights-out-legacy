use std::collections::{ HashMap, hash_map::Iter };
use super::{ Feature, FeatureAsset };

pub struct FeatureManager {

	features: HashMap<String, Feature>,

}

impl FeatureManager {

	pub fn new() -> Self {

		Self {

			features: HashMap::new(),

		}

	}

	pub fn get(&self, name: &str) -> Option<&Feature> {

		self.features.get(name)

	}

	pub fn get_mut(&mut self, name: &str) -> Option<&mut Feature> {

		self.features.get_mut(name)

	}

	pub fn load(&mut self, asset: FeatureAsset) {

		let name = String::from(asset.name);
		let techonology = Feature::new(asset);

		self.features.insert(name, techonology);

	}

	pub fn iter(&self) -> Iter<String, Feature> {

		self.features.iter()

	}

	pub fn set_unlock(&mut self, name: &str, unlock: bool) {

		self.features
			.get_mut(name)
			.map(|u| u.set_unlock(unlock));

	}

}