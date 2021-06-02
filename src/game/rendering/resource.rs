use std::rc::Rc;
use std::collections::HashMap;
use wasm_bindgen::{ closure::Closure, JsCast };
use web_sys::{ Document, Element, HtmlElement, Window };
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
	pub button_element: Element, 
	pub list_element: Element,
	pub title_element: Element,
	pub is_unlocked: bool,

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

					root_element: self.web_document.create_element("div").unwrap(),
					button_element: self.web_document.create_element("button").unwrap(),
					title_element: self.web_document.create_element("div").unwrap(),
					list_element: self.web_document.create_element("ul").unwrap(),
					is_unlocked: false

				};

				// Set class name.

				category_element.root_element.set_class_name("resource-category locked");
				category_element.button_element.set_class_name("resource-category-button");
				category_element.list_element.set_class_name("resource-category-list");
				category_element.title_element.set_class_name("resource-category-title");

				// Append.

				category_element.root_element.append_with_node_1(&category_element.button_element).unwrap();
				category_element.root_element.append_with_node_1(&category_element.title_element).unwrap();
				category_element.root_element.append_with_node_1(&category_element.list_element).unwrap();

				// Set inner html.

				category_element.button_element.set_inner_html("Collapse");
				category_element.title_element.set_inner_html(stuff_manager.get_text(&format!("resource_category_{}", resource.get_asset().category)).unwrap_or(&resource.get_asset().category.to_uppercase()));

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

			// Set class name.

			resource_element.root_element.set_class_name("resource");
			resource_element.title_element.set_class_name("resource-title");
			resource_element.count_element.set_class_name("resource-count");
			resource_element.capacity_element.set_class_name("resource-capacity");
			resource_element.production_element.set_class_name("resource-production");

			// Append.

			resource_element.root_element.append_with_node_1(&resource_element.title_element).unwrap();
			resource_element.root_element.append_with_node_1(&resource_element.count_element).unwrap();
			resource_element.root_element.append_with_node_1(&resource_element.capacity_element).unwrap();
			resource_element.root_element.append_with_node_1(&resource_element.production_element).unwrap();

			// Set inner html.

			resource_element.title_element.set_inner_html(stuff_manager.get_text(name).unwrap_or(&name.to_uppercase()));

			self.resource_elements.insert(String::from(name), resource_element);

		}

		// Sort and append.

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
	pub fn render(&mut self, stuff_manager: &StuffManager) {

		for (name, resource) in stuff_manager.iter_resource() {

			let resource_element = self.resource_elements.get(name).unwrap();

			if !resource.is_unlocked() {

				resource_element.root_element.set_class_name("resource locked");

			} else {

				resource_element.root_element.set_class_name("resource");

				self.resource_category_elements
					.get_mut(resource.get_asset().category)
					.map(|c| {
						
						if !c.is_unlocked {

							c.is_unlocked = true;
							c.root_element.set_class_name("resource-category")
					
						}

					});

				resource_element.count_element.set_inner_html(&format_number_scientific(resource.get_count()));
				resource_element.capacity_element.set_inner_html(&format_number_scientific(resource.get_capacity()));
				resource_element.production_element.set_inner_html(&format!("{}/s", &format_number_scientific((resource.get_production() - resource.get_consumption()) * 5f64)));

			}

		}

	}

}