use std::collections::HashMap;
use std::rc::Rc;
use web_sys::{ Document, Element, Window };
use crate::game::stuff::{ Stuff, StuffManager };
use crate::utils::number::format_number_scientific;
use super::Tab;

struct StatElement {

	pub root_element: Element, 

	pub title_element: Element,
	pub value_element: Element,

}

struct StatCategoryElement {

	pub root_element: Element, 

	pub list_element: Element,
	pub title_element: Element,

}

/// A stats tab.
pub struct StatTab {

	web_window: Rc<Window>,
	web_document: Rc<Document>,

	tab_element: Element,
	tab_button_element: Element,

	stat_category_elements: HashMap<String, StatCategoryElement>,
	stat_elements: HashMap<String, StatElement>,

	is_selected: bool,

}

impl StatTab {

	/// Creates a new tab.
	pub fn new(window: Rc<Window>, document: Rc<Document>, stuff_manager: &StuffManager) -> Self {

		let tab_list_element = document.get_element_by_id("tab-list").expect("Element id 'tab-list' not found.");

		// Tab.

		let tab_element = document.get_element_by_id("tab-stats").unwrap();
		let tab_button_element = document.create_element("div").unwrap();

		tab_button_element.set_attribute("onclick", "window.Game.change_tab('Stats')").unwrap();
		tab_button_element.set_inner_html(stuff_manager.get_text("ui_tab_stats").unwrap_or("TAB_STATS"));
		tab_button_element.set_class_name("button");

		tab_list_element.append_with_node_1(&tab_button_element).unwrap();

		// Stats.

		let mut stat_category_elements = HashMap::new();
		let mut stat_elements= HashMap::new();

		for (name, stat) in stuff_manager.iter_stat() {

			// Create category.

			if !stat_category_elements.contains_key(stat.get_asset().category) {

				let category_element = StatCategoryElement {

					list_element: document.create_element("ul").unwrap(),
					title_element: document.create_element("div").unwrap(),
					root_element: document.create_element("div").unwrap(),

				};

				category_element.root_element.set_class_name("stat-category locked");
				category_element.list_element.set_class_name("stat-category-list");
				category_element.title_element.set_class_name("stat-category-title");

				category_element.root_element.append_with_node_1(&category_element.title_element).unwrap();
				category_element.root_element.append_with_node_1(&category_element.list_element).unwrap();

				category_element.title_element.set_inner_html(stuff_manager.get_text(&["stat_category_", stat.get_asset().category].join("")).unwrap_or("STAT_CATEGORY"));

				stat_category_elements.insert(String::from(stat.get_asset().category), category_element);

			}

			// Create stat.

			let stat_element = StatElement {
				
				root_element: document.create_element("li").unwrap(),
				title_element: document.create_element("div").unwrap(),
				value_element: document.create_element("div").unwrap(),

			};

			stat_element.root_element.set_class_name("stat");
			stat_element.title_element.set_class_name("stat-title");
			stat_element.value_element.set_class_name("stat-value");

			stat_element.root_element.append_with_node_1(&stat_element.title_element).unwrap();
			stat_element.root_element.append_with_node_1(&stat_element.value_element).unwrap();

			stat_element.title_element.set_inner_html(stuff_manager.get_text(&[stat.get_asset().name, "_title"].join("")).unwrap_or("STAT_TITLE"));

			stat_elements.insert(String::from(name), stat_element);

		}

		// Append stats.

		let mut sorted_stat_elements: Vec<(&String, &StatElement)> = stat_elements.iter().collect();
		let mut sorted_stat_category_elements: Vec<(&String, &StatCategoryElement)> = stat_category_elements.iter().collect();

		sorted_stat_elements.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));
		sorted_stat_category_elements.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));

		for (name, element) in sorted_stat_elements.iter() {

			let stat = stuff_manager.get_stat(name).unwrap();
			let category_element = stat_category_elements.get(stat.get_asset().category).unwrap();

			category_element.list_element.append_with_node_1(&element.root_element).unwrap();

		}

		for (_, element) in sorted_stat_category_elements.iter() {

			tab_element.append_with_node_1(&element.root_element).unwrap();

		}

		Self {

			web_document: document.clone(),
			web_window: window.clone(),
			tab_element,
			tab_button_element,
			stat_category_elements,
			stat_elements,
			is_selected: false,

		}

	}

}

impl Tab for StatTab {

	fn is_selected(&self) -> bool {
		
		self.is_selected

	}

	fn render(&mut self, stuff_manager: &StuffManager) {

		if !stuff_manager.is_feature_unlocked("feature_tab_stats") {

			self.tab_element.set_class_name("tab locked");
			self.tab_button_element.set_class_name("button locked");
			return;

		} else {

			self.tab_element.set_class_name(if self.is_selected { "tab active" } else { "tab" });
			self.tab_button_element.set_class_name(if self.is_selected { "button active" } else { "button" });

		}

		for (name, stat) in stuff_manager.iter_stat() {

			let stat_element = self.stat_elements.get(name).unwrap();

			self.stat_category_elements
				.get(stat.get_asset().category)
				.map(|c| c.root_element.set_class_name("stat-category"));

			stat_element.title_element.set_inner_html(if stat.get_value() <= 0f64 { "???" } else { stuff_manager.get_text(&[stat.get_asset().name, "_title"].join("")).unwrap_or("STAT_TITLE") });
			stat_element.value_element.set_inner_html(&format_number_scientific(stat.get_value()));

		}

	}

	fn set_selected(&mut self, selected: bool) {
		
		self.is_selected = selected;

	}

}