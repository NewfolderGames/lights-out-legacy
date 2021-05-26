use std::rc::Rc;
use web_sys::{ Document, Window };
use crate::game::stuff::StuffManager;
use super::{ LoadingManager, LogManager, ResourceManager, TabManager };

/// A rendering manager.
pub struct RenderingManager {

	// Web stuffs.

	web_window: Rc<Window>,
	web_document: Rc<Document>,

	// Managers.

	loading_manager: LoadingManager,
	log_manager: LogManager,
	resource_manager: ResourceManager,
	tab_manager: TabManager,

}

impl RenderingManager {

	/// Creates a new rendering manager.
	pub fn new(window: Rc<Window>, document: Rc<Document>) -> Self {

		Self {

			web_window: window.clone(), 
			web_document: document.clone(),
			loading_manager: LoadingManager::new(window.clone(), document.clone()),
			log_manager: LogManager::new(window.clone(), document.clone()),
			resource_manager: ResourceManager::new(window.clone(), document.clone()),
			tab_manager: TabManager::new(window.clone(), document.clone()),

		}

	}

	/// Changes the tab.
	pub fn change_tab(&mut self, name: &str, stuff_manager: &StuffManager) {

		self.tab_manager.select(name);
		self.tab_manager.render(stuff_manager);

	}

	/// Initialize managers.
	pub fn init(&mut self, stuff_manager: &StuffManager) {

		self.resource_manager.init(stuff_manager);
		self.tab_manager.init(stuff_manager);

	}

	/// Pushs a log to log-container.
	pub fn push_log(&self, log: &str, color: Option<&str>) {

		self.log_manager.push(log, color);

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