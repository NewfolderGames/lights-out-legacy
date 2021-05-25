use web_sys::{ Document, Element, Window };

pub struct LoadingRenderer {

	loading_element: Element,

	title_element: Element,
	description_element: Element,

}

impl LoadingRenderer {

	pub fn new() -> Self {

		let window: Window = web_sys::window().expect("Window not found.");
		let document: Document = window.document().expect("Document not found.");

		Self {

			loading_element: document.get_element_by_id("loading").unwrap(),
			title_element: document.get_element_by_id("loading-title").unwrap(),
			description_element: document.get_element_by_id("loading-description").unwrap()

		}

	}

	pub fn set_loading(&self, loading: bool) {

		self.loading_element.set_class_name(if loading { "active" } else { "" });

	}

	pub fn set_loading_title(&self, title: &str) {

		self.title_element.set_inner_html(title);

	}

	pub fn set_loading_description(&self, description: &str) {

		self.description_element.set_inner_html(description);

	}

}