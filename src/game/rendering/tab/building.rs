use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{ closure::Closure, JsCast };
use web_sys::{ Document, Element, HtmlElement, Window };
use crate::game::stuff::{ Stuff, StuffManager };
use crate::utils::number::format_number_scientific;
use super::Tab;

struct BuildingModifierElement {

	pub root_element: Element,
	pub name_element: Element,
	pub value_element: Element

}

struct BuildingPriceElement {

	pub root_element: Element,
	pub name_element: Element,
	pub count_element: Element

}

struct BuildingElement {

	pub is_active: bool,
	pub is_unlocked: bool,

	pub root_element: Element,
	pub toggle_element: Element,
	pub title_element: Element,
	pub description_element: Element,
	pub modifier_container_element: Element,
	pub modifier_elements: HashMap<String, BuildingModifierElement>,
	pub price_container_element: Element,
	pub price_elements: HashMap<String, BuildingPriceElement>,

}

struct BuildingCategoryElement {

	pub root_element: Element, 
	pub button_element: Element,
	pub list_element: Element,
	pub title_element: Element,
	pub is_unlocked: bool,

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

		tab_button_element.set_class_name("button");
		tab_button_element.set_inner_html(stuff_manager.get_text("ui_tab_building").unwrap_or("TAB_BUILDING"));
		tab_button_element.set_attribute("onclick", "Game.ui_change_tab('Building')").unwrap();

		tab_list_element.append_with_node_1(&tab_button_element).unwrap();

		// Buildings.

		let mut building_category_elements = HashMap::new();
		let mut building_elements = HashMap::new();

		for (name, building) in stuff_manager.iter_building() {

			// Create category.

			if !building_category_elements.contains_key(building.get_asset().category) {

				let category_element = BuildingCategoryElement {

					root_element: document.create_element("div").unwrap(),
					button_element: document.create_element("button").unwrap(),
					list_element: document.create_element("ul").unwrap(),
					title_element: document.create_element("div").unwrap(),
					is_unlocked: false

				};

				// Set class name.

				category_element.root_element.set_class_name("building-category locked");
				category_element.button_element.set_class_name("building-category-button");
				category_element.list_element.set_class_name("building-category-list");
				category_element.title_element.set_class_name("building-category-title");

				// Append.

				category_element.root_element.append_with_node_1(&category_element.button_element).unwrap();
				category_element.root_element.append_with_node_1(&category_element.title_element).unwrap();
				category_element.root_element.append_with_node_1(&category_element.list_element).unwrap();

				// Set inner html.

				category_element.button_element.set_inner_html("Collapse");
				category_element.title_element.set_inner_html(stuff_manager.get_text(&format!("building_category_{}", building.get_asset().category)).unwrap_or(&format!("BUILDING_CATEGORY_{}", building.get_asset().category.to_uppercase())));

				// Set click event.

				let closure_root_element = category_element.root_element.clone();
				let closure_button_element = category_element.button_element.clone();
				let closure = Closure::wrap(Box::new(move || {

					let root_element_class_list = closure_root_element.class_list();
					root_element_class_list.toggle("collapsed").unwrap();
					closure_button_element.set_inner_html(if root_element_class_list.contains("collapsed") { "Open" } else { "Collapse" });

				}) as Box<dyn Fn()>);
				category_element.button_element.dyn_ref::<HtmlElement>().unwrap().set_onclick(Some(closure.as_ref().unchecked_ref()));
				closure.forget();

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

			// Set class name.

			building_element.root_element.set_class_name("building locked");
			building_element.toggle_element.set_class_name("building-toggle");
			building_element.title_element.set_class_name("building-title");
			building_element.description_element.set_class_name("building-description");
			building_element.modifier_container_element.set_class_name("building-modifier-container");
			building_element.price_container_element.set_class_name("building-price-container");

			// Append.

			building_element.root_element.append_with_node_1(&building_element.toggle_element).unwrap();
			building_element.root_element.append_with_node_1(&building_element.title_element).unwrap();
			building_element.root_element.append_with_node_1(&building_element.description_element).unwrap();
			building_element.root_element.append_with_node_1(&building_element.modifier_container_element).unwrap();
			building_element.root_element.append_with_node_1(&building_element.price_container_element).unwrap();

			// Set inner html.

			building_element.title_element.set_inner_html(stuff_manager.get_text(&format!("{}_title", name)).unwrap_or(&format!("{}_TITLE", name.to_uppercase())));
			building_element.description_element.set_inner_html(stuff_manager.get_text(&format!("{}_description", name)).unwrap_or(&format!("{}_DESCRIPTION", name.to_uppercase())));

			// Set click event.

			building_element.title_element.set_attribute("onclick", &format!("Game.purchase_building('{}')", name)).unwrap();
			building_element.toggle_element.set_attribute("onclick", &format!("Game.toggle_building('{}')", name)).unwrap();

			// Modifiers.

			for (modifier_name, modifier_value) in building.get_base_modifiers() {

				let modifier_element = BuildingModifierElement {

					root_element: document.create_element("div").unwrap(),
					name_element: document.create_element("div").unwrap(),
					value_element: document.create_element("div").unwrap()

				};

				// Set class name.

				modifier_element.root_element.set_class_name("building-modifier");
				modifier_element.name_element.set_class_name("building-modifier-name");
				modifier_element.value_element.set_class_name("building-modifier-value");

				// Set inner html.

				modifier_element.name_element.set_inner_html(stuff_manager.get_text(&format!("{}", modifier_name)).unwrap_or(&format!("{}", modifier_name.to_uppercase())));
				modifier_element.value_element.set_inner_html(&format_number_scientific(*modifier_value));

				// Append.

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

				// Set class.

				price_element.root_element.set_class_name("building-price");
				price_element.name_element.set_class_name("building-resource-name");
				price_element.count_element.set_class_name("building-resource-count");

				// Set inner html.

				price_element.name_element.set_inner_html(stuff_manager.get_text(&format!("{}", resource_name)).unwrap_or(&format!("{}", resource_name.to_uppercase())));
				price_element.count_element.set_inner_html(&format_number_scientific(*resource_count));

				// Append.

				building_element.price_container_element.append_with_node_1(&price_element.root_element).unwrap();
				price_element.root_element.append_with_node_1(&price_element.name_element).unwrap();
				price_element.root_element.append_with_node_1(&price_element.count_element).unwrap();

				building_element.price_elements.insert(String::from(resource_name), price_element);

			}

			building_elements.insert(String::from(name), building_element);

		}

		// Sort and append building.

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

				// Category.

				self.building_category_elements
					.get_mut(building.get_asset().category)
					.map(|c| {

						if !c.is_unlocked {

							c.is_unlocked = true;
							c.root_element.set_class_name("building-category");

						}

					});
				
				// Building.

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