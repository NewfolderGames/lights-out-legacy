use std::collections::HashMap;
use std::rc::Rc;
use web_sys::{ Document, Element, Window };
use crate::game::stuff::StuffManager;
use crate::utils::number::format_number_scientific;
use super::Tab;

struct TechnologyPriceElement {

	root_element: Element,
	resource_element: Element,
	count_element: Element

}

struct TechnologyElement {

	is_researched: bool,
	is_unlocked: bool,

	root_element: Element,
	title_element: Element,
	description_element: Element,
	price_container_element: Element,
	price_elements: HashMap<String, TechnologyPriceElement>,

}

/// A technology tab.
pub struct TechnologyTab {

	web_window: Rc<Window>,
	web_document: Rc<Document>,

	tab_element: Element,
	tab_button_element: Element,

	locked_element: Element,
	researchable_element: Element,
	researched_element: Element,
	technology_elements: HashMap<String, TechnologyElement>,

	is_selected: bool,

}

impl TechnologyTab {

	pub fn new(window: Rc<Window>, document: Rc<Document>, stuff_manager: &StuffManager) -> Self {

		let tab_list_element = document.get_element_by_id("tab-list").expect("Element id 'tab-list' not found.");

		// Tab.

		let tab_element = document.get_element_by_id("tab-technology").unwrap();
		let tab_button_element = document.create_element("div").unwrap();

		tab_button_element.set_attribute("onclick", "window.Game.change_tab('Technology')").unwrap();
		tab_button_element.set_inner_html(stuff_manager.get_text("ui_tab_technology").unwrap_or("TAB_TECHNOLOGY"));
		tab_button_element.set_class_name("button");

		tab_list_element.append_with_node_1(&tab_button_element).unwrap();

		// Technologies.

		let locked_element = document.get_element_by_id("tab-technology-locked").unwrap();
		let researchable_element = document.get_element_by_id("tab-technology-researchable").unwrap();
		let researched_element = document.get_element_by_id("tab-technology-researched").unwrap();

		let mut technology_elements = HashMap::new();

		for (name, technology) in stuff_manager.iter_technology() {

			let mut technology_element = TechnologyElement {

				is_researched: technology.is_researched(),
				is_unlocked: technology.is_unlocked(),
				root_element: document.create_element("div").unwrap(),
				title_element: document.create_element("div").unwrap(),
				description_element: document.create_element("div").unwrap(),
				price_container_element: document.create_element("div").unwrap(),
				price_elements: HashMap::new()

			};

			technology_element.root_element.set_class_name("technology");
			technology_element.title_element.set_class_name("technology-title");
			technology_element.description_element.set_class_name("technology-description");
			technology_element.price_container_element.set_class_name("technology-price-container");

			technology_element.title_element.set_inner_html(stuff_manager.get_text(&format!("{}_title", name)).unwrap_or(&format!("{}_TITLE", name.to_uppercase())));
			technology_element.description_element.set_inner_html(stuff_manager.get_text(&format!("{}_description", name)).unwrap_or(&format!("{}_DESCRIPTION", name.to_uppercase())));

			technology_element.root_element.set_attribute("onclick", &format!("window.Game.purchase_technology('{}')", name)).unwrap();

			// Price.

			for (resource_name, resource_count) in technology.get_price() {

				let price_element = TechnologyPriceElement {
					
					root_element: document.create_element("div").unwrap(),
					resource_element: document.create_element("div").unwrap(),
					count_element: document.create_element("div").unwrap()

				};

				price_element.root_element.set_class_name("technology-price");
				price_element.resource_element.set_class_name("technology-resource-name");
				price_element.count_element.set_class_name("technology-resource-count");

				price_element.resource_element.set_inner_html(stuff_manager.get_text(resource_name).unwrap_or(&resource_name.to_uppercase()));
				price_element.count_element.set_inner_html(&format_number_scientific(*resource_count));

				technology_element.price_container_element.append_with_node_1(&price_element.root_element).unwrap();
				price_element.root_element.append_with_node_1(&price_element.resource_element).unwrap();
				price_element.root_element.append_with_node_1(&price_element.count_element).unwrap();

				technology_element.price_elements.insert(String::from(resource_name), price_element);

			}
			
			// Append.

			if technology.is_researched() { researched_element.append_with_node_1(&technology_element.root_element).unwrap() }
			else if technology.is_unlocked() { researchable_element.append_with_node_1(&technology_element.root_element).unwrap() }
			else { locked_element.append_with_node_1(&technology_element.root_element).unwrap() }

			technology_element.root_element.append_with_node_1(&technology_element.title_element).unwrap();
			technology_element.root_element.append_with_node_1(&technology_element.description_element).unwrap();
			technology_element.root_element.append_with_node_1(&technology_element.price_container_element).unwrap();

			technology_elements.insert(String::from(name), technology_element);

		}

		Self {

			web_document: document.clone(),
			web_window: window.clone(),
			tab_element,
			tab_button_element,
			locked_element,
			researchable_element,
			researched_element,
			technology_elements,
			is_selected: false,

		}

	}

}

impl Tab for TechnologyTab {

	fn is_selected(&self) -> bool {
		
		self.is_selected

	}

	fn render(&mut self, stuff_manager: &StuffManager) {

		// Tab.

		if !stuff_manager.is_feature_unlocked("feature_tab_technology") {

			self.tab_element.set_class_name("tab locked");
			self.tab_button_element.set_class_name("button locked");
			return;

		} else {

			self.tab_element.set_class_name(if self.is_selected { "tab active" } else { "tab" });
			self.tab_button_element.set_class_name(if self.is_selected { "button active" } else { "button" });

		}

		// Techology.

		for (name, technology) in stuff_manager.iter_technology() {

			let technology_element = self.technology_elements.get_mut(name).unwrap();

			// Move technology.

			if technology.is_unlocked() && !technology_element.is_unlocked {

				technology_element.is_unlocked = true;
				self.researchable_element.append_with_node_1(&technology_element.root_element).unwrap();

			}

			if technology.is_researched() && !technology_element.is_researched {

				technology_element.is_researched = true;
				self.researched_element.append_with_node_1(&technology_element.root_element).unwrap();

			}

			// Update price.

			if !technology.is_unlocked() { continue; }

			for (resource_name, resource_count) in technology.get_price() {

				let price_element = technology_element.price_elements.get(resource_name).unwrap();
				let current_resource_count = stuff_manager.get_resource(resource_name).unwrap().get_count();

				if current_resource_count >= *resource_count {

					price_element.count_element.set_class_name("technology-resuorce-count");
					price_element.count_element.set_inner_html(&format_number_scientific(*resource_count));

				} else {

					price_element.count_element.set_class_name("technology-resource-count needs_more");
					price_element.count_element.set_inner_html(&format!("{} / {}", format_number_scientific(current_resource_count), format_number_scientific(*resource_count)));

				}

			}

		}

	}

	fn set_selected(&mut self, selected: bool) {
		
		self.is_selected = selected;

	}

}