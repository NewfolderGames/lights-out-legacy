use std::collections::HashMap;
use std::rc::Rc;
use web_sys::{ Document, Element, Window };
use crate::game::stuff::{ Stuff, StuffManager };
use crate::utils::number::format_number_scientific;
use super::Tab;

struct BuildingModifierElement {

	root_element: Element,
	name_element: Element,
	value_element: Element

}

struct BuildingPriceElement {

	root_element: Element,
	name_element: Element,
	count_element: Element

}

struct BuildingElement {

	is_active: bool,
	is_unlocked: bool,

	root_element: Element,
	toggle_element: Element,
	title_element: Element,
	description_element: Element,
	modifier_container_element: Element,
	modifier_elements: HashMap<String, BuildingModifierElement>,
	price_container_element: Element,
	price_elements: HashMap<String, BuildingPriceElement>,

}

struct BuildingCategoryElement {

	pub root_element: Element, 

	pub list_element: Element,
	pub title_element: Element,

}

/// A building tab.
pub struct BuildingTab {

	web_window: Rc<Window>,
	web_document: Rc<Document>,

	tab_element: Element,
	tab_button_element: Element,

	building_category_elements: HashMap<String, BuildingCategoryElement>,
	building_elements: HashMap<String, BuildingElement>,

	is_selected: bool,

}

impl BuildingTab {

	/// Creates a new tab.
	pub fn new(window: Rc<Window>, document: Rc<Document>, stuff_manager: &StuffManager) -> Self {

		let tab_list_element = document.get_element_by_id("tab-list").expect("Element id 'tab-list' not found.");

		// Tab.

		let tab_element = document.get_element_by_id("tab-building").unwrap();
		let tab_button_element = document.create_element("div").unwrap();

		tab_button_element.set_attribute("onclick", "window.Game.change_tab('Building')").unwrap();
		tab_button_element.set_inner_html(stuff_manager.get_text("ui_tab_building").unwrap_or("TAB_BUILDING"));
		tab_button_element.set_class_name("button");

		tab_list_element.append_with_node_1(&tab_button_element).unwrap();

		// Buildings.

		let mut building_category_elements = HashMap::new();
		let mut building_elements = HashMap::new();

		for (name, building) in stuff_manager.iter_building() {

			// Create category.

			if !building_category_elements.contains_key(building.get_asset().category) {

				let category_element = BuildingCategoryElement {

					list_element: document.create_element("ul").unwrap(),
					title_element: document.create_element("div").unwrap(),
					root_element: document.create_element("div").unwrap(),

				};

				category_element.root_element.set_class_name("building-category locked");
				category_element.list_element.set_class_name("building-category-list");
				category_element.title_element.set_class_name("building-category-title");

				category_element.root_element.append_with_node_1(&category_element.title_element).unwrap();
				category_element.root_element.append_with_node_1(&category_element.list_element).unwrap();

				category_element.title_element.set_inner_html(stuff_manager.get_text(&format!("building_category_{}", building.get_asset().category)).unwrap_or(&format!("BUILDING_CATEGORY_{}", building.get_asset().category.to_uppercase())));

				building_category_elements.insert(String::from(building.get_asset().category), category_element);

			}

			// Create building.

			let mut building_element = BuildingElement {

				is_active: building.is_active(),
				is_unlocked: building.is_unlocked(),
				toggle_element: document.create_element("button").unwrap(),
				root_element: document.create_element("div").unwrap(),
				title_element: document.create_element("div").unwrap(),
				description_element: document.create_element("div").unwrap(),
				modifier_container_element: document.create_element("div").unwrap(),
				modifier_elements: HashMap::new(),
				price_container_element: document.create_element("div").unwrap(),
				price_elements: HashMap::new()

			};

			building_element.root_element.set_class_name("building locked");
			building_element.toggle_element.set_class_name("building-toggle");
			building_element.title_element.set_class_name("building-title");
			building_element.description_element.set_class_name("building-description");
			building_element.modifier_container_element.set_class_name("building-modifier-container");
			building_element.price_container_element.set_class_name("building-price-container");

			building_element.root_element.append_with_node_1(&building_element.toggle_element).unwrap();
			building_element.root_element.append_with_node_1(&building_element.title_element).unwrap();
			building_element.root_element.append_with_node_1(&building_element.description_element).unwrap();
			building_element.root_element.append_with_node_1(&building_element.modifier_container_element).unwrap();
			building_element.root_element.append_with_node_1(&building_element.price_container_element).unwrap();

			building_element.title_element.set_inner_html(stuff_manager.get_text(&format!("{}_title", name)).unwrap_or(&format!("{}_TITLE", name.to_uppercase())));
			building_element.description_element.set_inner_html(stuff_manager.get_text(&format!("{}_description", name)).unwrap_or(&format!("{}_DESCRIPTION", name.to_uppercase())));

			building_element.title_element.set_attribute("onclick", &format!("window.Game.purchase_building('{}')", name)).unwrap();
			building_element.toggle_element.set_attribute("onclick", &format!("window.Game.toggle_building('{}')", name)).unwrap();

			// Modifiers.

			for (modifier_name, modifier_value) in building.get_base_modifiers() {

				let modifier_element = BuildingModifierElement {

					root_element: document.create_element("div").unwrap(),
					name_element: document.create_element("div").unwrap(),
					value_element: document.create_element("div").unwrap()

				};

				modifier_element.root_element.set_class_name("building-modifier");
				modifier_element.name_element.set_class_name("building-modifier-name");
				modifier_element.value_element.set_class_name("building-modifier-value");

				modifier_element.name_element.set_inner_html(stuff_manager.get_text(&format!("{}", modifier_name)).unwrap_or(&format!("{}", modifier_name.to_uppercase())));
				modifier_element.value_element.set_inner_html(&format_number_scientific(*modifier_value));

				building_element.modifier_container_element.append_with_node_1(&modifier_element.root_element).unwrap();
				modifier_element.root_element.append_with_node_1(&modifier_element.name_element).unwrap();
				modifier_element.root_element.append_with_node_1(&modifier_element.value_element).unwrap();

				building_element.modifier_elements.insert(String::from(modifier_name), modifier_element);

			}

			// Price

			for (resource_name, resource_count) in building.get_price() {

				let price_element = BuildingPriceElement {

					root_element: document.create_element("div").unwrap(),
					name_element: document.create_element("div").unwrap(),
					count_element: document.create_element("div").unwrap()

				};

				price_element.root_element.set_class_name("building-price");
				price_element.name_element.set_class_name("building-resource-name");
				price_element.count_element.set_class_name("building-resource-count");

				price_element.name_element.set_inner_html(stuff_manager.get_text(&format!("{}", resource_name)).unwrap_or(&format!("{}", resource_name.to_uppercase())));
				price_element.count_element.set_inner_html(&format_number_scientific(*resource_count));

				building_element.price_container_element.append_with_node_1(&price_element.root_element).unwrap();
				price_element.root_element.append_with_node_1(&price_element.name_element).unwrap();
				price_element.root_element.append_with_node_1(&price_element.count_element).unwrap();

				building_element.price_elements.insert(String::from(resource_name), price_element);

			}

			building_elements.insert(String::from(name), building_element);

		}

		// Append building.

		let mut sorted_building_elements: Vec<(&String, &BuildingElement)> = building_elements.iter().collect();
		let mut sorted_building_category_elements: Vec<(&String, &BuildingCategoryElement)> = building_category_elements.iter().collect();

		sorted_building_elements.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));
		sorted_building_category_elements.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));

		for (name, element) in sorted_building_elements.iter() {

			let building = stuff_manager.get_building(name).unwrap();
			let category_element = building_category_elements.get(building.get_asset().category).unwrap();

			category_element.list_element.append_with_node_1(&element.root_element).unwrap();

		}

		for (_, element) in sorted_building_category_elements.iter() {

			tab_element.append_with_node_1(&element.root_element).unwrap();

		}

		Self {

			web_document: document.clone(),
			web_window: window.clone(),
			tab_element,
			tab_button_element,
			building_category_elements,
			building_elements,
			is_selected: false,

		}

	}

}

impl Tab for BuildingTab {

	fn is_selected(&self) -> bool {
		
		self.is_selected

	}

	fn render(&mut self, stuff_manager: &StuffManager) {

		// Tab.

		if !stuff_manager.is_feature_unlocked("feature_tab_building") {

			self.tab_element.set_class_name("tab locked");
			self.tab_button_element.set_class_name("button locked");
			return;

		} else {

			self.tab_element.set_class_name(if self.is_selected { "tab active" } else { "tab" });
			self.tab_button_element.set_class_name(if self.is_selected { "button active" } else { "button" });

		}

		if !self.is_selected { return }

		// Buildings.

		for (name, building) in stuff_manager.iter_building() {

			let building_element = self.building_elements.get(name).unwrap();

			if !building.is_unlocked() {

				building_element.root_element.set_class_name("building locked");

			} else {

				self.building_category_elements
					.get(building.get_asset().category)
					.map(|c| c.root_element.set_class_name("building-category"));
				
				building_element.root_element.set_class_name("building");
				building_element.toggle_element.set_class_name(if building.is_active() { "building-toggle enabled" } else { "building-toggle disabled" });
				building_element.toggle_element.set_inner_html(if building.is_active() { "Enabled" } else { "Disabled" });
				building_element.title_element.set_inner_html(&format!("{} ({})", stuff_manager.get_text(&format!("{}_title", name)).unwrap_or(&format!("{}_TITLE", name.to_uppercase())), building.get_count()));

				// Modifiers.

				for (modifier_name, modifier_value) in building.get_base_modifiers() {

					let modifier_element = building_element.modifier_elements.get(modifier_name).unwrap();

					if *modifier_value == 0f64 {

						modifier_element.root_element.set_class_name("building-modifier locked");

					} else {

						modifier_element.root_element.set_class_name("building-modifier");
						modifier_element.value_element.set_inner_html(&format_number_scientific(*modifier_value));
	
					}

				}

				// Price.

				for (resource_name, resource_count) in building.get_price() {

					let price_element = building_element.price_elements.get(resource_name).unwrap();
					let current_resource_count = stuff_manager.get_resource(resource_name).unwrap().get_count();
	
					if current_resource_count >= *resource_count {
	
						price_element.count_element.set_class_name("building-resource-count");
						price_element.count_element.set_inner_html(&format_number_scientific(*resource_count));
	
					} else {
	
						price_element.count_element.set_class_name("building-resource-count needs_more");
						price_element.count_element.set_inner_html(&format!("{} / {}", format_number_scientific(current_resource_count), format_number_scientific(*resource_count)));
	
					}
	
				}

			}

			

		}

	}

	fn set_selected(&mut self, selected: bool) {
		
		self.is_selected = selected;

	}

}