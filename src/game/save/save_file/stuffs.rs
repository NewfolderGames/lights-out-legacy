use std::collections::{ HashMap, HashSet };
use serde::{ Serialize, Deserialize };
use crate::game::stuff::StuffManager;

/// A stuff save file.
#[derive(Serialize, Deserialize)]
pub struct Stuffs {

	pub buildings: Option<HashMap<String, Buildings>>,
	pub resources: Option<HashMap<String, f64>>,
	pub stats: Option<HashMap<String, f64>>,
	pub technologies: Option<HashSet<String>>,
	pub unlocks: Option<HashSet<String>>,
	pub upgrades: Option<HashSet<String>>,

}

impl Stuffs {

	/// Creates a new stuff save file.
	pub fn new() -> Self {

		Self {

			buildings: Some(HashMap::new()),
			resources: Some(HashMap::new()),
			stats: Some(HashMap::new()),
			technologies: Some(HashSet::new()),
			unlocks: Some(HashSet::new()),
			upgrades: Some(HashSet::new()),

		}

	}

	/// Clears the save file.
	pub fn clear(&mut self) {

		self.buildings.get_or_insert(HashMap::new()).clear();
		self.resources.get_or_insert(HashMap::new()).clear();
		self.stats.get_or_insert(HashMap::new()).clear();
		self.technologies.get_or_insert(HashSet::new()).clear();
		self.unlocks.get_or_insert(HashSet::new()).clear();
		self.upgrades.get_or_insert(HashSet::new()).clear();

	}
	
	/// Loads the save data into the manager.
	pub fn load(&mut self, stuff_manager: &mut StuffManager) {

		self.buildings
			.get_or_insert(HashMap::new())
			.iter()
			.for_each(|(b_name, b)| {

				stuff_manager.set_building(b_name, b.count);
				if !b.is_active { stuff_manager.toggle_building(b_name) }

			});

		self.resources
			.get_or_insert(HashMap::new())
			.iter()
			.for_each(|(r_name, r_count)| stuff_manager.set_resource(r_name, *r_count));

		self.stats
			.get_or_insert(HashMap::new())
			.iter()
			.for_each(|(s_name, s_value)| stuff_manager.set_stat(s_name, *s_value));

		self.technologies
			.get_or_insert(HashSet::new())
			.iter()
			.for_each(|t_name| stuff_manager.research_technology(t_name));

		self.unlocks
			.get_or_insert(HashSet::new())
			.iter()
			.for_each(|u_name| stuff_manager.unlock(u_name));

		self.upgrades
			.get_or_insert(HashSet::new())
			.iter()
			.for_each(|u_name| stuff_manager.research_upgrade(u_name));
		
	}

	pub fn save(&mut self, stuff_manager: &StuffManager) {

		stuff_manager
			.iter_building()
			.for_each(|(b_name, b)| {

				if !b.is_unlocked() || b.get_count() == 0 { return }
				self.buildings
					.get_or_insert(HashMap::new())
					.insert(String::from(b_name), Buildings::new(b.get_count(), b.is_active()));

			});
		
		stuff_manager
			.iter_resource()
			.for_each(|(r_name, r)| {

				if !r.is_unlocked() || r.get_count() == 0f64 { return }
				self.resources
					.get_or_insert(HashMap::new())
					.insert(String::from(r_name), r.get_count());

			});

		stuff_manager
			.iter_stat()
			.for_each(|(s_name, s)| {
				
				if s.get_value() == 0f64 { return }
				self.stats
					.get_or_insert(HashMap::new())
					.insert(String::from(s_name), s.get_value());
			
			});

		stuff_manager
			.iter_technology()
			.for_each(|(t_name, t)| {
				
				if !t.is_unlocked() || !t.is_researched() { return }
				self.technologies
					.get_or_insert(HashSet::new())
					.insert(String::from(t_name));
			
			});

		stuff_manager
			.iter_unlock()
			.for_each(|(u_name, u)| {
				
				if !u.is_unlocked() { return }
				self.unlocks
					.get_or_insert(HashSet::new())
					.insert(String::from(u_name));
			
			});

		stuff_manager
			.iter_upgrade()
			.for_each(|(u_name, u)| {
				
				if !u.is_unlocked() || !u.is_researched() { return }
				self.upgrades
					.get_or_insert(HashSet::new())
					.insert(String::from(u_name));
			
			});

	}

}

#[derive(Serialize, Deserialize)]
pub struct Buildings {

	pub count: i32,
	pub is_active: bool,

}

impl Buildings {

	pub fn new(count: i32, is_active: bool) -> Self {

		Self {

			count,
			is_active

		}

	}

}