use std::rc::Rc;
use web_sys::Document;
use crate::game::stuff::StuffManager;
use super::{ LoadingManager, LogManager, ResourceManager, TabManager };

/// A rendering manager.
pub struct RenderingManager {

	loading_manager: LoadingManager,
	log_manager: LogManager,
	resource_manager: ResourceManager,
	tab_manager: TabManager,

}

impl RenderingManager {

	/// Creates a new rendering manager.
	pub fn new(document: Rc<Document>, stuff_manager: &StuffManager) -> Self {

		Self {

			loading_manager: LoadingManager::new(document.clone()),
			log_manager: LogManager::new(document.clone()),
			resource_manager: ResourceManager::new(document.clone(), stuff_manager),
			tab_manager: TabManager::new(document.clone(), stuff_manager),

		}

	}

	/// Changes the tab.
	pub fn change_tab(&mut self, name: &str, stuff_manager: &StuffManager) {

		self.tab_manager.select(name);
		self.tab_manager.render(stuff_manager);

	}

	/// Pushs a log to log-container.
	pub fn push_log(&self, log: &str, class: &str) {

		self.log_manager.push(log, class);

	}

	/// Renders game.
	pub fn render(&mut self, stuff_manager: &StuffManager) {

		self.resource_manager.render(stuff_manager);
		self.tab_manager.render(stuff_manager);

	}

	/// Sets loading state.
	pub fn set_loading(&self, loading: bool) {

		self.loading_manager.set_loading(loading);

	}

	/// Sets loading desciption.
	pub fn set_loading_description(&self, description: &str) {

		self.loading_manager.set_loading_description(description);

	}

}