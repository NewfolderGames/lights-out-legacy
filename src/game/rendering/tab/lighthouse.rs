use web_sys::{ Document, Element, Window };
use crate::game::StuffManager;
use super::Tab;

pub struct LighthouseTab {

	tab_element: Element,
	button_element: Element,

	is_selected: bool,

}

impl LighthouseTab {

	pub fn new() -> Self {

		let window: Window = web_sys::window().expect("Window not found.");
		let document: Document = window.document().expect("Document not found.");
		let tab_list_element = document.get_element_by_id("tab-list").expect("Element id 'tab-list' not found.");

		// Tab list button.

		let button_element = document.create_element("div").unwrap();

		button_element.set_attribute("onclick", "window.game.change_tab('Lighthouse')").unwrap();
		button_element.set_inner_html("Lighthouse");
		button_element.set_class_name("tab-button");

		tab_list_element.append_with_node_1(&button_element).unwrap();

		// Tab.

		let tab_element = document.get_element_by_id("tab-lighthouse").unwrap();

		Self {

			tab_element,
			button_element,
			is_selected: false,

		}

	}

}

impl Tab for LighthouseTab {

	fn is_selected(&self) -> bool {
		
		self.is_selected

	}

	fn render(&self, stuff_manager: &StuffManager) {

		if !stuff_manager.is_feature_unlocked("feature_tab_lighthouse") {

			self.tab_element.set_class_name("tab-button locked");
			self.button_element.set_class_name("tab-button locked");
			return;

		}

		if !self.is_selected {

			self.tab_element.set_class_name("tab-button inactive");
			self.button_element.set_class_name("tab-button inactive");

		} else {

			self.tab_element.set_class_name("tab-button active");
			self.button_element.set_class_name("tab-button active");

		}

	}

	fn set_selected(&mut self, selected: bool) {
		
		self.is_selected = selected;

	}

}