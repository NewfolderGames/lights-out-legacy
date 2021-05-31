use std::collections::hash_map::{ HashMap, Iter };
use super::super::{ ModifierStorage, ResourceStorage, Stuff, StuffAsset, StuffStorage };

/// A job data.
pub struct Job {

	asset: JobAsset,

	base_modifier: Vec<(String, f64)>,
	calculated_modifiers: Vec<(String, f64)>,
	count: i32,

	is_active: bool,
	is_unlocked: bool,

}

/// A job asset.
pub struct JobAsset {

	pub name: &'static str,

	pub modifiers: Box<dyn Fn(&ModifierStorage, &ResourceStorage) -> Vec<(&'static str, f64)>>,
	pub deficit: Box<dyn Fn(&ResourceStorage) -> bool>,

}

impl JobAsset {

	/// Creates a new job asset.
	pub fn new(
		name: &'static str,
		modifiers: Box<dyn Fn(&ModifierStorage, &ResourceStorage) -> Vec<(&'static str, f64)>>,
		deficit: Box<dyn Fn(&ResourceStorage) -> bool>
	) -> Self {

		Self {

			name,
			modifiers,
			deficit

		}

	}

}

impl StuffAsset for JobAsset {

	const NAME: &'static str = "asset_job";

}

/// A job storage.
pub struct JobStorage {

	jobs: HashMap<String, Job>,

}