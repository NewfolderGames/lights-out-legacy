use std::rc::Rc;
use web_sys::{ Document, Element };

pub const MAX_LOGS: u32 = 64;

pub struct LogManager {

	web_document: Rc<Document>,
	
	root_element: Element

}

impl LogManager {

	/// Create a new loading manager.
	pub fn new(document: Rc<Document>) -> Self {
	
		Self {
	
			web_document: document.clone(),
			root_element: document.get_element_by_id("log-list").unwrap(),
	
		}
	
	}

	pub fn push(&self, log: &str, class: &str) {

		// Remove old log.

		if self.root_element.child_element_count() >= MAX_LOGS {

			self.root_element
				.last_element_child()
				.unwrap()
				.remove();

		}

		// Create new log.

		let log_element = self.web_document.create_element("li").unwrap();

		log_element.set_class_name(&format!("log {}", class));
		log_element.set_inner_html(log);

		self.root_element.prepend_with_node_1(&log_element).unwrap();

	}

}
