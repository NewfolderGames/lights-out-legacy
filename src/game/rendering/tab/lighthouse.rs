use std::rc::Rc;
use web_sys::{ Document, Element, Window };
use crate::game::stuff::StuffManager;
use super::Tab;

/// A lighthouse tab.
pub struct LighthouseTab {

	web_window: Rc<Window>,
	web_document: Rc<Document>,

	tab_element: Element,
	tab_button_element: Element,

	button_examine_element: Element,
	button_gather_element: Element,
	button_search_element: Element,
	button_ligtsout_element: Element,

	is_selected: bool,

}

impl LighthouseTab {

	/// Creates a new tab.
	pub fn new(window: Rc<Window>, document: Rc<Document>, stuff_manager: &StuffManager) -> Self {

		let tab_list_element = document.get_element_by_id("tab-list").expect("Element id 'tab-list' not found.");

		// Tab.

		let tab_element = document.get_element_by_id("tab-lighthouse").unwrap();
		let tab_button_element = document.create_element("div").unwrap();

		tab_button_element.set_attribute("onclick", "window.Game.change_tab('Lighthouse')").unwrap();
		tab_button_element.set_inner_html(stuff_manager.get_text("ui_tab_lighthouse").unwrap_or("TAB_LIGHTHOUSE"));
		tab_button_element.set_class_name("button");

		tab_list_element.append_with_node_1(&tab_button_element).unwrap();

		// Buttons.

		let buttons_element = document.get_element_by_id("tab-lighthouse-buttons").expect("Element id 'tab-lighthouse-buttons' not found.");

		let button_examine_element = document.create_element("button").unwrap();
		let button_gather_element = document.create_element("button").unwrap();
		let button_search_element = document.create_element("button").unwrap();
		let button_ligtsout_element = document.create_element("button").unwrap();

		button_examine_element.set_inner_html(stuff_manager.get_text("ui_tab_lighthouse_button_examine").unwrap_or("UI_TAB_LIGHTHOUSE_BUTTON_EXAMINE"));
		button_gather_element.set_inner_html(stuff_manager.get_text("ui_tab_lighthouse_button_gather").unwrap_or("UI_TAB_LIGHTHOUSE_BUTTON_GATHER"));
		button_search_element.set_inner_html(stuff_manager.get_text("ui_tab_lighthouse_button_search").unwrap_or("UI_TAB_LIGHTHOUSE_BUTTON_SEARCH"));
		button_ligtsout_element.set_inner_html(stuff_manager.get_text("ui_tab_lighthouse_button_lightsout").unwrap_or("UI_TAB_LIGHTHOUSE_BUTTON_LIGHTSOUT"));

		button_examine_element.set_class_name("button");
		button_gather_element.set_class_name("button");
		button_search_element.set_class_name("button");
		button_ligtsout_element.set_class_name("button");

		button_examine_element.set_attribute("onclick", "window.Game.lighthouse_examine()").unwrap();
		button_gather_element.set_attribute("onclick", "window.Game.lighthouse_gather()").unwrap();
		button_search_element.set_attribute("onclick", "window.Game.lighthouse_search()").unwrap();
		button_ligtsout_element.set_attribute("onclick", "window.Game.lighthouse_lightsout()").unwrap();

		buttons_element.append_with_node_1(&button_examine_element).unwrap();
		buttons_element.append_with_node_1(&button_gather_element).unwrap();
		buttons_element.append_with_node_1(&button_search_element).unwrap();
		buttons_element.append_with_node_1(&button_ligtsout_element).unwrap();

		Self {

			web_document: document.clone(),
			web_window: window.clone(),
			tab_element,
			tab_button_element,
			button_examine_element,
			button_gather_element,
			button_search_element,
			button_ligtsout_element,
			is_selected: false,

		}

	}

}

impl Tab for LighthouseTab {

	fn is_selected(&self) -> bool {
		
		self.is_selected

	}

	fn render(&mut self, stuff_manager: &StuffManager) {

		// Tab.

		if !stuff_manager.is_feature_unlocked("feature_tab_lighthouse") {

			self.tab_element.set_class_name("tab locked");
			self.tab_button_element.set_class_name("button locked");
			return;

		} else {

			self.tab_element.set_class_name(if self.is_selected { "tab active" } else { "tab" });
			self.tab_button_element.set_class_name(if self.is_selected { "button active" } else { "button" });

		}

		if !self.is_selected { return }

		// Buttons.

		self.button_examine_element.set_class_name(if stuff_manager.is_feature_unlocked("feature_lighthouse_examine") { "button" } else { "button locked" });
		self.button_gather_element.set_class_name(if stuff_manager.is_feature_unlocked("feature_lighthouse_gather") { "button" } else { "button locked" });
		self.button_search_element.set_class_name(if stuff_manager.is_feature_unlocked("feature_lighthouse_search") { "button" } else { "button locked" });
		self.button_ligtsout_element.set_class_name(if stuff_manager.is_feature_unlocked("feature_lighthouse_lightsout") { "button" } else { "button locked" });

	}

	fn set_selected(&mut self, selected: bool) {
		
		self.is_selected = selected;

	}

}