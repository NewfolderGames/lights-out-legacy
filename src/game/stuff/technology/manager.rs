use std::collections::{ HashMap, hash_map::Iter };
use super::{ Technology, TechnologyAsset };

pub struct TechnologyManager {

	technologies: HashMap<String, Technology>,

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

		let name = String::from(asset.name);
		let techonology = Technology::new(asset);

		self.technologies.insert(name, techonology);

	}

	pub fn iter(&self) -> Iter<String, Technology> {

		self.technologies.iter()

	}

	pub fn set_unlock(&mut self, name: &str, unlock: bool) {

		self.technologies
			.get_mut(name)
			.map(|u| u.set_unlock(unlock));

	}

}