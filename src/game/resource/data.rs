/// A loadable resource data.
pub struct ResourceData {

	pub name: &'static str,
	pub title: &'static str,
	pub description: &'static str,
	pub image: &'static str,
	pub category: &'static str,
	
	pub is_hidden: bool,
	pub is_unlocked: bool,

	pub default_capacity: f64,

}

// Constructor.

impl ResourceData {

	/// Creates a new resource data.
	pub fn new(name: &'static str, title: &'static str, description: &'static str, image: &'static str, category: &'static str, is_hidden: bool, is_unlocked: bool, default_capacity: f64) -> Self {

		Self {

			name,
			title,
			description,
			image,
			category,
			is_hidden,
			is_unlocked,
			default_capacity,

		}

	}

}