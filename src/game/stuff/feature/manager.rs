use std::collections::{ HashMap };

/// A feature manager.
pub struct FeatureManager {

	features: HashMap<String, bool>

}

impl FeatureManager {

	/// Creates a new feature manager.
	pub fn new() -> Self {

		Self {

			features: HashMap::new()

		}

	}

	/// Returns `true` if the feature is unlocked.
	pub fn is_unlocked(&self, name: &str) -> bool {

		self.features
			.get(name)
			.copied()
			.unwrap_or(false)

	}
	
	/// Loads a feature.
	pub fn load(&mut self, name: &str) {

		self.features.insert(String::from(name), false);

	}

	/// Resets the feature manager.
	pub fn reset(&mut self) {
		
		self.features
			.iter_mut()
			.for_each(|(_, r)| *r = false);

	}

	/// Unlocks a feature.
	pub fn unlock(&mut self, name: &str) {

		self.features
			.get_mut(name)
			.map(|v| *v = true);

	}

}