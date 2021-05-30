use std::collections::{ HashMap, HashSet };
use serde::{ Serialize, Deserialize };
use crate::game::stuff::StuffManager;

#[derive(Serialize, Deserialize)]
pub struct Save {

	pub stuff: SaveStuff,

}

impl Save {

	pub fn new() -> Self {

		Self {

			stuff: SaveStuff::new()

		}

	}

}

#[derive(Serialize, Deserialize)]
pub struct SaveStuff {

	pub buildings: HashMap<String, (i32, bool)>,
	pub resources: HashMap<String, f64>,
	pub stats: HashMap<String, f64>,
	pub technologies: HashSet<String>,
	pub unlocks: HashSet<String>,
	pub upgrades: HashSet<String>,

}

impl SaveStuff {

	pub fn new() -> Self {

		Self {

			buildings: HashMap::new(),
			resources: HashMap::new(),
			stats: HashMap::new(),
			technologies: HashSet::new(),
			unlocks: HashSet::new(),
			upgrades: HashSet::new(),

		}

	}

	pub fn clear(&mut self) {

		self.buildings.clear();
		self.stats.clear();
		self.resources.clear();
		self.technologies.clear();
		self.unlocks.clear();
		self.upgrades.clear();

	}

	pub fn set_buildings(&mut self, stuff_manager: &StuffManager) {

		stuff_manager
			.iter_building()
			.for_each(|(b_name, b)| {

				if !b.is_unlocked() || b.get_count() == 0 { return }
				self.buildings.insert(String::from(b_name), (b.get_count(), b.is_active()));

			});

	}

	pub fn set_resources(&mut self, stuff_manager: &StuffManager) {

		stuff_manager
			.iter_resource()
			.for_each(|(r_name, r)| {

				if !r.is_unlocked() || r.get_count() == 0f64 { return }
				self.resources.insert(String::from(r_name), r.get_count());

			});

	}

	pub fn set_stats(&mut self, stuff_manager: &StuffManager) {

		stuff_manager
			.iter_stat()
			.for_each(|(s_name, s)| {
				
				if s.get_value() == 0f64 { return }
				self.stats.insert(String::from(s_name), s.get_value());
			
			});

	}

	pub fn set_technologies(&mut self, stuff_manager: &StuffManager) {

		stuff_manager
			.iter_technology()
			.for_each(|(t_name, t)| {
				
				if !t.is_unlocked() || !t.is_researched() { return }
				self.technologies.insert(String::from(t_name));
			
			});

	}

	pub fn set_unlocks(&mut self, stuff_manager: &StuffManager) {

		stuff_manager
			.iter_unlock()
			.for_each(|(u_name, u)| {
				
				if !u.is_unlocked() { return }
				self.unlocks.insert(String::from(u_name));
			
			});
		
	}

	pub fn set_upgrades(&mut self, stuff_manager: &StuffManager) {

		stuff_manager
			.iter_upgrade()
			.for_each(|(u_name, u)| {
				
				if !u.is_unlocked() || !u.is_researched() { return }
				self.upgrades.insert(String::from(u_name));
			
			});

	}
	
}

