use std::collections::HashMap;
use std::rc::Rc;
use web_sys::{ Document, Window };
use crate::game::stuff::StuffManager;
use super::*;

/// A tab manager.
pub struct TabManager {

	web_window: Rc<Window>,
	web_document: Rc<Document>,

	selected: String,
	tabs: HashMap<&'static str, Box<dyn Tab>>

}

impl TabManager {

	/// Creates a new tab manager.
	pub fn new(window: Rc<Window>, document: Rc<Document>) -> Self {

		Self {

			web_window: window.clone(),
			web_document: document.clone(),
			selected: String::from("Lighthouse"),
			tabs: HashMap::new(),

		}

	}

	/// Initialize the tab manager.
	pub fn init(&mut self, stuff_manager: &StuffManager) {

		let mut lighthouse = Box::new(LighthouseTab::new(self.web_window.clone(), self.web_document.clone(), stuff_manager));
		lighthouse.set_selected(true);

		self.tabs.insert("Lighthouse", lighthouse);
		self.tabs.insert("Stats", Box::new(StatTab::new(self.web_window.clone(), self.web_document.clone(), stuff_manager)));

	}

	/// Renders a tab.
	pub fn render(&self, stuff_manager: &StuffManager) {

		for (_, tab) in self.tabs.iter() {

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