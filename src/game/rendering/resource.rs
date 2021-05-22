use std::collections::HashMap;
use web_sys::{ Document, Element, Window };
use crate::game::StuffManager;

struct ResourceElement {

	pub root_element: Element,

	pub capacity_element: Element,
	pub count_element: Element,
	pub production_element: Element,
	pub title_element: Element,

}

struct ResourceCategoryElement {

	pub root_element: Element, 

	pub list_element: Element,
	pub title_element: Element,

}

pub struct ResourceRenderer {

	resource_category_elements: HashMap<&'static str, ResourceCategoryElement>,
	resource_container_element: Element,
	resource_elements: HashMap<&'static str, ResourceElement>,

}

impl ResourceRenderer {

	pub fn new() -> Self {

		let window: Window = web_sys::window().expect("Window not found.");
		let document: Document = window.document().expect("Document not found.");

		Self {

			resource_category_elements: HashMap::new(),
			resource_container_element: document.get_element_by_id("resource-container").unwrap(),
			resource_elements: HashMap::new(),

		}

	}

	pub fn init(&mut self, stuff_manager: &StuffManager) {

		let window: Window = web_sys::window().expect("Window not found.");
		let document: Document = window.document().expect("Document not found.");

		for (name, resource) in stuff_manager.iter_resource() {

			// Create category.

			if !self.resource_category_elements.contains_key(&resource.get_asset().category) {

				let category_element = ResourceCategoryElement {

					list_element: document.create_element("ul").unwrap(),
					title_element: document.create_element("div").unwrap(),
					root_element: document.create_element("div").unwrap(),

				};

				category_element.list_element.set_class_name("resource-category-list");
				category_element.root_element.set_class_name("resource-category");
				category_element.title_element.set_class_name("resource-category-title");

				self.resource_container_element.append_with_node_1(&category_element.root_element).unwrap();
				category_element.root_element.append_with_node_1(&category_element.title_element).unwrap();
				category_element.root_element.append_with_node_1(&category_element.list_element).unwrap();

				category_element.title_element.set_inner_html(stuff_manager.get_text(&["resource_category_", resource.get_asset().category].join("")).unwrap_or("RESOURCE_CATEGORY"));

				self.resource_category_elements.insert(resource.get_asset().category, category_element);

			}

			// Get category.
			
			let category_element = self.resource_category_elements.get(&resource.get_asset().category).unwrap();
			let resource_element = ResourceElement {

				root_element: document.create_element("li").unwrap(),
				count_element: document.create_element("div").unwrap(),
				production_element: document.create_element("div").unwrap(),
				capacity_element: document.create_element("div").unwrap(),
				title_element: document.create_element("div").unwrap(),

			};

			resource_element.root_element.set_class_name("resource");
			resource_element.title_element.set_class_name("resource-title");
			resource_element.count_element.set_class_name("resource-count");
			resource_element.capacity_element.set_class_name("resource-capacity");
			resource_element.production_element.set_class_name("resource-production");

			category_element.list_element.append_with_node_1(&resource_element.root_element).unwrap();
			resource_element.root_element.append_with_node_1(&resource_element.title_element).unwrap();
			resource_element.root_element.append_with_node_1(&resource_element.count_element).unwrap();
			resource_element.root_element.append_with_node_1(&resource_element.capacity_element).unwrap();
			resource_element.root_element.append_with_node_1(&resource_element.production_element).unwrap();

			resource_element.title_element.set_inner_html(stuff_manager.get_text(&[resource.get_asset().name, "_title"].join("")).unwrap_or("RESOURCE_TITLE"));

			self.resource_elements.insert(*name, resource_element);

		}

	}

	pub fn render(&self, stuff_manager: &StuffManager) {

		for (name, resource) in stuff_manager.iter_resource() {

			let resource_element = self.resource_elements.get(*name).unwrap();

			if !resource.is_unlocked() {

				resource_element.root_element.set_class_name("resource locked");

			} else {

				resource_element.root_element.set_class_name("resource unlocked");

				resource_element.count_element.set_inner_html(&resource.get_count().to_string());
				resource_element.capacity_element.set_inner_html(&resource.get_capacity().to_string());
				resource_element.production_element.set_inner_html(&resource.get_production().to_string());

			}

		}

	}

}