use std::rc::Rc;
use web_sys::{ Document, Element, Window };

pub struct LoadingManager {

	web_window: Rc<Window>,
	web_document: Rc<Document>,

	root_element: Element,
	title_element: Element,
	description_element: Element,

}

impl LoadingManager {

	/// Create a new loading manager.
	pub fn new(window: Rc<Window>, document: Rc<Document>) -> Self {
	
		Self {
	
			web_window: window.clone(), 
			web_document: document.clone(),
			root_element: document.get_element_by_id("loading").unwrap(),
			title_element: document.get_element_by_id("loading-title").unwrap(),
			description_element: document.get_element_by_id("loading-description").unwrap()
	
		}
	
	}

	/// Sets loading state.
	pub fn set_loading(&self, loading: bool) {

		self.root_element.set_class_name(if loading { "active" } else { "" });

	}

	/// Sets loading title.
	pub fn set_loading_title(&self, title: &str) {

		self.title_element.set_inner_html(title);

	}

	/// Sets loading description.
	pub fn set_loading_description(&self, description: &str) {

		self.description_element.set_inner_html(description);

	}

}