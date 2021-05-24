
use web_sys::{ Document, Element, Window };

pub const MAX_LOGS: u32 = 64;

pub struct LogRenderer {

	log_list_element: Element

}

impl LogRenderer {

	pub fn new() -> Self {

		let window: Window = web_sys::window().expect("Window not found.");
		let document: Document = window.document().expect("Document not found.");

		Self {

			log_list_element: document.get_element_by_id("log-list").unwrap()

		}

	}

	pub fn push(&self, log: &str, color: Option<&str>) {

		let window: Window = web_sys::window().expect("Window not found.");
		let document: Document = window.document().expect("Document not found.");

		// Remove old log.

		if self.log_list_element.child_element_count() >= MAX_LOGS {

			self.log_list_element
				.last_element_child()
				.unwrap()
				.remove();

		}

		// Create new log.

		let log_element = document.create_element("li").unwrap();

		log_element.set_class_name("log");
		log_element.set_inner_html(log);
		log_element.set_attribute("style", &["color: ", color.unwrap_or("inherit")].join("")).unwrap();

		self.log_list_element.prepend_with_node_1(&log_element).unwrap();

	}

}