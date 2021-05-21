use std::collections::{ HashMap, hash_map::Iter };
use super::{ Feature, FeatureAsset };

pub struct FeatureManager {

	features: HashMap<&'static str, Feature>,

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

		let name = asset.name;
		let techonology = Feature::new(asset);

		self.features.insert(name, techonology);

	}

	pub fn iter(&self) -> Iter<&'static str, Feature> {

		self.features.iter()

	}

	pub fn unlock(&mut self, name: &str) {

		self.features
			.get_mut(name)
			.map(|u| u.unlock());

	}

}