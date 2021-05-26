use std::rc::Rc;
use std::collections::HashMap;
use web_sys::{ Document, Element, Window };
use crate::game::stuff::{ Stuff, StuffManager };
use crate::utils::number::format_number_scientific;
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

/// A resource rendering manager.
pub struct ResourceManager {

	web_window: Rc<Window>,
	web_document: Rc<Document>,

	resource_category_elements: HashMap<String, ResourceCategoryElement>,
	resource_container_element: Element,
	resource_elements: HashMap<String, ResourceElement>,

}

impl ResourceManager {

	/// Create a new resource manager.
	pub fn new(window: Rc<Window>, document: Rc<Document>) -> Self {
	
		Self {
	
			web_window: window.clone(), 
			web_document: document.clone(),
			resource_category_elements: HashMap::new(),
			resource_container_element: document.get_element_by_id("resource-container").unwrap(),
			resource_elements: HashMap::new(),
	
		}
	
	}

	/// Initialize the manager.
	pub fn init(&mut self, stuff_manager: &StuffManager) {

		for (name, resource) in stuff_manager.iter_resource() {

			// Create category.

			if !self.resource_category_elements.contains_key(resource.get_asset().category) {

				let category_element = ResourceCategoryElement {

					list_element: self.web_document.create_element("ul").unwrap(),
					title_element: self.web_document.create_element("div").unwrap(),
					root_element: self.web_document.create_element("div").unwrap(),

				};

				category_element.root_element.set_class_name("resource-category locked");
				category_element.list_element.set_class_name("resource-category-list");
				category_element.title_element.set_class_name("resource-category-title");

				
				// self.resource_container_element.append_with_node_1(&category_element.root_element).unwrap();
				category_element.root_element.append_with_node_1(&category_element.title_element).unwrap();
				category_element.root_element.append_with_node_1(&category_element.list_element).unwrap();

				category_element.title_element.set_inner_html(stuff_manager.get_text(&["resource_category_", resource.get_asset().category].join("")).unwrap_or("RESOURCE_CATEGORY"));

				self.resource_category_elements.insert(String::from(resource.get_asset().category), category_element);

			}

			// Create resource.
			
			let resource_element = ResourceElement {

				root_element: self.web_document.create_element("li").unwrap(),
				count_element: self.web_document.create_element("div").unwrap(),
				production_element: self.web_document.create_element("div").unwrap(),
				capacity_element: self.web_document.create_element("div").unwrap(),
				title_element: self.web_document.create_element("div").unwrap(),

			};

			resource_element.root_element.set_class_name("resource");
			resource_element.title_element.set_class_name("resource-title");
			resource_element.count_element.set_class_name("resource-count");
			resource_element.capacity_element.set_class_name("resource-capacity");
			resource_element.production_element.set_class_name("resource-production");

			resource_element.root_element.append_with_node_1(&resource_element.title_element).unwrap();
			resource_element.root_element.append_with_node_1(&resource_element.count_element).unwrap();
			resource_element.root_element.append_with_node_1(&resource_element.capacity_element).unwrap();
			resource_element.root_element.append_with_node_1(&resource_element.production_element).unwrap();

			resource_element.title_element.set_inner_html(stuff_manager.get_text(&[resource.get_asset().name, "_title"].join("")).unwrap_or("RESOURCE_TITLE"));

			self.resource_elements.insert(String::from(name), resource_element);

		}

		// Append.

		let mut sorted_resource_elements: Vec<(&String, &ResourceElement)> = self.resource_elements.iter().collect();
		let mut sorted_resource_category_elements: Vec<(&String, &ResourceCategoryElement)> = self.resource_category_elements.iter().collect();

		sorted_resource_elements.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));
		sorted_resource_category_elements.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));

		for (name, element) in sorted_resource_elements.iter() {

			let resource = stuff_manager.get_resource(name).unwrap();
			let category_element = self.resource_category_elements.get(resource.get_asset().category).unwrap();

			category_element.list_element.append_with_node_1(&element.root_element).unwrap();

		}

		for (_, element) in sorted_resource_category_elements.iter() {

			self.resource_container_element.append_with_node_1(&element.root_element).unwrap();

		}
		
	}

	/// Renders resources.
	pub fn render(&self, stuff_manager: &StuffManager) {

		for (name, resource) in stuff_manager.iter_resource() {

			let resource_element = self.resource_elements.get(name).unwrap();

			if !resource.is_unlocked() {

				resource_element.root_element.set_class_name("resource locked");

			} else {

				resource_element.root_element.set_class_name("resource");

				self.resource_category_elements
					.get(resource.get_asset().category)
					.map(|c| c.root_element.set_class_name("resource-category"));

				resource_element.count_element.set_inner_html(&format_number_scientific(resource.get_count()));
				resource_element.capacity_element.set_inner_html(&format_number_scientific(resource.get_capacity()));
				resource_element.production_element.set_inner_html(&format_number_scientific(resource.get_production() - resource.get_consumption()));

			}

		}

	}

}