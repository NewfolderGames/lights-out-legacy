use std::collections::{ HashMap, hash_map::Iter };
use super::{ Technology, TechnologyAsset };

pub struct TechnologyManager {

	technologies: HashMap<&'static str, Technology>,

}

impl TechnologyManager {

	pub fn new() -> Self {

		Self {

			technologies: HashMap::new(),

		}

	}

	pub fn get(&self, name: &str) -> Option<&Technology> {

		self.technologies.get(name)

	}

	pub fn get_mut(&mut self, name: &str) -> Option<&mut Technology> {

		self.technologies.get_mut(name)

	}

	pub fn load(&mut self, asset: TechnologyAsset) {

		let name = asset.name;
		let techonology = Technology::new(asset);

		self.technologies.insert(name, techonology);

	}

	pub fn iter(&self) -> Iter<&'static str, Technology> {

		self.technologies.iter()

	}

	pub fn unlock(&mut self, name: &str) {

		self.technologies
			.get_mut(name)
			.map(|u| u.unlock());

	}

}