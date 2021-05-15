pub struct BuildingAsset {

	pub name: &'static str,
	pub title: &'static str,
	pub description: &'static str,
	pub image: &'static str,
	pub category: &'static str,
	
	pub is_hidden: bool,
	pub is_unlocked: bool,

	pub modifiers: Box<dyn Fn() -> Vec<(String, f64)>>,
	pub price: Vec<(String, f64)>,
	pub price_multiplier: f64,

}

impl BuildingAsset {

	pub fn new(name: &'static str, title: &'static str, description: &'static str, image: &'static str, category: &'static str, is_hidden: bool, is_unlocked: bool, modifiers: Box<dyn Fn() -> Vec<(String, f64)>>, price: Vec<(String, f64)>, price_multiplier: f64) -> Self {

		Self {

			name,
			title,
			description,
			image,
			category,
			is_hidden,
			is_unlocked,
			modifiers,
			price,
			price_multiplier,

		}

	}

}