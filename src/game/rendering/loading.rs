use std::rc::Rc;
use web_sys::{ Document, Element };

pub struct LoadingManager {

	root_element: Element,
	title_element: Element,
	description_element: Element,

}

impl LoadingManager {

	/// Create a new loading manager.
	pub fn new(document: Rc<Document>) -> Self {
	
		Self {
	
			root_element: document.get_element_by_id("loading").unwrap(),
			title_element: document.get_element_by_id("loading-title").unwrap(),
			description_element: document.get_element_by_id("loading-description").unwrap()
	
		}
	
	}

	/// Sets loading state.
	pub fn set_loading(&self, loading: bool) {

		if loading {

			self.root_element.class_list().add_1("active").unwrap();

		} else {

			self.root_element.class_list().remove_1("active").unwrap();

		}

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