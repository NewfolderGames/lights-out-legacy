use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{ closure::Closure, JsCast };
use web_sys::{ Document, Element, HtmlElement, Window };
use crate::game::stuff::StuffManager;
use crate::utils::number::format_number_scientific;
use super::Tab;

struct TechnologyPriceElement {

	pub root_element: Element,
	pub resource_element: Element,
	pub count_element: Element

}

struct TechnologyElement {

	pub is_researched: bool,
	pub is_unlocked: bool,

	pub root_element: Element,
	pub title_element: Element,
	pub description_element: Element,
	pub price_container_element: Element,
	pub price_elements: HashMap<String, TechnologyPriceElement>,

}

struct TechnologyCategoryElement {

	pub root_element: Element,
	pub button_element: Element,
	pub title_element: Element,
	pub list_element: Element

}


/// A technology tab.
pub struct TechnologyTab {

	web_window: Rc<Window>,
	web_document: Rc<Document>,

	tab_element: Element,
	tab_button_element: Element,

	locked_element: TechnologyCategoryElement,
	researchable_element: TechnologyCategoryElement,
	researched_element: TechnologyCategoryElement,
	technology_elements: HashMap<String, TechnologyElement>,

	is_selected: bool,

}

impl TechnologyTab {

	// Creates a new technology tab.
	pub fn new(window: Rc<Window>, document: Rc<Document>, stuff_manager: &StuffManager) -> Self {

		let tab_list_element = document.get_element_by_id("tab-list").expect("Element id 'tab-list' not found.");

		// Tab.

		let tab_element = document.get_element_by_id("tab-technology").unwrap();
		let tab_button_element = document.create_element("div").unwrap();

		tab_button_element.set_attribute("onclick", "Game.ui_change_tab('Technology')").unwrap();
		tab_button_element.set_inner_html(stuff_manager.get_text("ui_tab_technology").unwrap_or("TAB_TECHNOLOGY"));
		tab_button_element.set_class_name("button");

		tab_list_element.append_with_node_1(&tab_button_element).unwrap();

		// Categories.

		let locked_element = TechnologyCategoryElement {

			root_element: document.get_element_by_id("tab-technology-locked").unwrap(),
			button_element: document.create_element("button").unwrap(),
			title_element: document.create_element("div").unwrap(),
			list_element: document.create_element("ul").unwrap(),

		};
		let researchable_element = TechnologyCategoryElement {

			root_element: document.get_element_by_id("tab-technology-researchable").unwrap(),
			button_element: document.create_element("button").unwrap(),
			title_element: document.create_element("div").unwrap(),
			list_element: document.create_element("ul").unwrap(),

		};
		let researched_element = TechnologyCategoryElement {

			root_element: document.get_element_by_id("tab-technology-researched").unwrap(),
			button_element: document.create_element("button").unwrap(),
			title_element: document.create_element("div").unwrap(),
			list_element: document.create_element("ul").unwrap(),

		};

		// Set class name.

		locked_element.root_element.set_class_name("technology-category");
		locked_element.button_element.set_class_name("technology-category-button");
		locked_element.title_element.set_class_name("technology-category-title");
		locked_element.list_element.set_class_name("technology-category-list");

		researchable_element.root_element.set_class_name("technology-category");
		researchable_element.button_element.set_class_name("technology-category-button");
		researchable_element.title_element.set_class_name("technology-category-title");
		researchable_element.list_element.set_class_name("technology-category-list");

		researched_element.root_element.set_class_name("technology-category");
		researched_element.button_element.set_class_name("technology-category-button");
		researched_element.title_element.set_class_name("technology-category-title");
		researched_element.list_element.set_class_name("technology-category-list");

		// Append

		locked_element.root_element.append_with_node_1(&locked_element.button_element).unwrap();
		locked_element.root_element.append_with_node_1(&locked_element.title_element).unwrap();
		locked_element.root_element.append_with_node_1(&locked_element.list_element).unwrap();

		researchable_element.root_element.append_with_node_1(&researchable_element.button_element).unwrap();
		researchable_element.root_element.append_with_node_1(&researchable_element.title_element).unwrap();
		researchable_element.root_element.append_with_node_1(&researchable_element.list_element).unwrap();

		researched_element.root_element.append_with_node_1(&researched_element.button_element).unwrap();
		researched_element.root_element.append_with_node_1(&researched_element.title_element).unwrap();
		researched_element.root_element.append_with_node_1(&researched_element.list_element).unwrap();

		// Set inner html.

		locked_element.button_element.set_inner_html("Collapse");
		locked_element.title_element.set_inner_html("Locked technologies");

		researchable_element.button_element.set_inner_html("Collapse");
		researchable_element.title_element.set_inner_html("Researchable technologies");

		researched_element.button_element.set_inner_html("Collapse");
		researched_element.title_element.set_inner_html("Researched technologies");

		// Set click event.

		let closure_root_element = locked_element.root_element.clone();
		let closure_button_element = locked_element.button_element.clone();
		let closure = Closure::wrap(Box::new(move || {

			let root_element_class_list = closure_root_element.class_list();
			root_element_class_list.toggle("collapsed").unwrap();
			closure_button_element.set_inner_html(if root_element_class_list.contains("collapsed") { "Open" } else { "Collapse" });

		}) as Box<dyn Fn()>);
		locked_element.button_element.dyn_ref::<HtmlElement>().unwrap().set_onclick(Some(closure.as_ref().unchecked_ref()));
		closure.forget();

		let closure_root_element = researchable_element.root_element.clone();
		let closure_button_element = researchable_element.button_element.clone();
		let closure = Closure::wrap(Box::new(move || {

			let root_element_class_list = closure_root_element.class_list();
			root_element_class_list.toggle("collapsed").unwrap();
			closure_button_element.set_inner_html(if root_element_class_list.contains("collapsed") { "Open" } else { "Collapse" });

		}) as Box<dyn Fn()>);
		researchable_element.button_element.dyn_ref::<HtmlElement>().unwrap().set_onclick(Some(closure.as_ref().unchecked_ref()));
		closure.forget();

		let closure_root_element = researched_element.root_element.clone();
		let closure_button_element = researched_element.button_element.clone();
		let closure = Closure::wrap(Box::new(move || {

			let root_element_class_list = closure_root_element.class_list();
			root_element_class_list.toggle("collapsed").unwrap();
			closure_button_element.set_inner_html(if root_element_class_list.contains("collapsed") { "Open" } else { "Collapse" });

		}) as Box<dyn Fn()>);
		researched_element.button_element.dyn_ref::<HtmlElement>().unwrap().set_onclick(Some(closure.as_ref().unchecked_ref()));
		closure.forget();

		// Technologies.
		
		let mut technology_elements = HashMap::new();

		for (name, technology) in stuff_manager.iter_technology() {

			let mut technology_element = TechnologyElement {

				is_researched: technology.is_researched(),
				is_unlocked: technology.is_unlocked(),
				root_element: document.create_element("li").unwrap(),
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

			technology_element.title_element.set_attribute("onclick", &format!("Game.purchase_technology('{}')", name)).unwrap();

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

			if technology.is_researched() { researched_element.list_element.append_with_node_1(&technology_element.root_element).unwrap() }
			else if technology.is_unlocked() { researchable_element.list_element.append_with_node_1(&technology_element.root_element).unwrap() }
			else { locked_element.list_element.append_with_node_1(&technology_element.root_element).unwrap() }

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

		if !self.is_selected { return }

		// Techology.

		for (name, technology) in stuff_manager.iter_technology() {

			let technology_element = self.technology_elements.get_mut(name).unwrap();

			// Move technology.

			if technology.is_unlocked() && !technology_element.is_unlocked {

				technology_element.is_unlocked = true;
				self.researchable_element.list_element.append_with_node_1(&technology_element.root_element).unwrap();

			}

			if technology.is_researched() && !technology_element.is_researched {

				technology_element.is_researched = true;
				self.researched_element.list_element.append_with_node_1(&technology_element.root_element).unwrap();

			}

			if !technology_element.is_unlocked{ continue; }
			
			// Update price.

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