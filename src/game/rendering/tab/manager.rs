use std::collections::HashMap;
use std::rc::Rc;
use web_sys::Document;
use crate::game::stuff::StuffManager;
use super::*;

/// A tab manager.
pub struct TabManager {

	selected: String,
	tabs: HashMap<String, Box<dyn Tab>>

}

impl TabManager {

	/// Creates a new tab manager.
	pub fn new(document: Rc<Document>, stuff_manager: &StuffManager) -> Self {

		let mut tabs: HashMap<String, Box<dyn Tab>> = HashMap::new();

		let mut lighthouse = Box::new(LighthouseTab::new(document.clone(), stuff_manager));
		lighthouse.set_selected(true);

		tabs.insert(String::from("Lighthouse"), lighthouse);
		tabs.insert(String::from("Building"), Box::new(BuildingTab::new(document.clone(), stuff_manager)));
		tabs.insert(String::from("Technology"), Box::new(TechnologyTab::new(document.clone(), stuff_manager)));
		tabs.insert(String::from("Upgrade"), Box::new(UpgradeTab::new(document.clone(), stuff_manager)));
		tabs.insert(String::from("Stats"), Box::new(StatTab::new(document.clone(), stuff_manager)));

		Self {

			selected: String::from("Lighthouse"),
			tabs,

		}

	}

	/// Renders tabs.
	pub fn render(&mut self, stuff_manager: &StuffManager) {

		for (_, tab) in self.tabs.iter_mut() {

			tab.render(stuff_manager);

		}

	}

	/// Selects a tab.
	pub fn select(&mut self, name: &str) {

		self.selected = String::from(name);

		for (name, tab) in self.tabs.iter_mut() {

			tab.set_selected(*name == self.selected);

		}
		
	}

}