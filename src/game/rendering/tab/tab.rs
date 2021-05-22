use std::collections::HashMap;
use web_sys::{ Document, Element, Window };
use crate::game::StuffManager;
use super::LighthouseTab;

pub trait Tab {

	fn is_selected(&self) -> bool;
	fn render(&self, stuff_manager: &StuffManager);
	fn set_selected(&mut self, selected: bool);

}

pub struct TabRenderer {

	selected: String,
	tabs: HashMap<&'static str, Box<dyn Tab>>

}

impl TabRenderer {

	pub fn new() -> Self {

		Self {

			selected: String::from("Lighthouse"),
			tabs: HashMap::new(),

		}

	}

	pub fn init(&mut self, stuff_manager: &StuffManager) {

		let mut lighthouse = Box::new(LighthouseTab::new());
		lighthouse.set_selected(true);

		self.tabs.insert("Lighthouse", lighthouse);

	}

	pub fn render(&self, stuff_manager: &StuffManager) {

		for (_, tab) in self.tabs.iter() {

			tab.render(stuff_manager);

		}

	}

	pub fn select(&mut self, name: &str) {

		self.selected = String::from(name);

		for (name, tab) in self.tabs.iter_mut() {

			tab.set_selected(*name == self.selected);

		}
		
	}

}