use std::rc::Rc;
use std::collections::HashMap;
use wasm_bindgen::{ closure::Closure, JsCast };
use web_sys::{ Document, Element, HtmlElement };
use crate::game::stuff::StuffManager;
use crate::utils::number::format_number_scientific;

struct ResourceElement {

	pub root_element: Element,
	pub capacity_element: Element,
	pub count_element: Element,
	pub production_element: Element,
	pub title_element: Element,
	pub is_unlocked: bool,

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

	resource_category_elements: HashMap<String, ResourceCategoryElement>,
	resource_container_element: Element,
	resource_elements: HashMap<String, ResourceElement>,

}

impl ResourceManager {

	/// Create a new resource manager.
	pub fn new(document: Rc<Document>, stuff_manager: &StuffManager) -> Self {
	
		let mut resource_elements = HashMap::new();
		let mut resource_category_elements = HashMap::new();
		let resource_container_element = document.get_element_by_id("resource-container").unwrap();

		for (name, resource) in stuff_manager.iter_resource() {

			// Create category.

			if !resource_category_elements.contains_key(resource.get_category()) {

				let category_element = ResourceCategoryElement {

					root_element: document.create_element("div").unwrap(),
					button_element: document.create_element("button").unwrap(),
					title_element: document.create_element("div").unwrap(),
					list_element: document.create_element("ul").unwrap(),
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
				category_element.title_element.set_inner_html(stuff_manager.get_text_string(&format!("resource_category_{}", resource.get_category())).unwrap_or(&format!("RESOURCE_CATEGORY_{}", resource.get_category().to_uppercase())));

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

				// Insert.

				resource_category_elements.insert(String::from(resource.get_category()), category_element);

			}

			// Create resource.
			
			let resource_element = ResourceElement {

				root_element: document.create_element("li").unwrap(),
				count_element: document.create_element("div").unwrap(),
				production_element: document.create_element("div").unwrap(),
				capacity_element: document.create_element("div").unwrap(),
				title_element: document.create_element("div").unwrap(),
				is_unlocked: false,

			};

			// Set class name.

			resource_element.root_element.set_class_name("resource locked");
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

			resource_element.title_element.set_inner_html(stuff_manager.get_text_string(&format!("resource_{}", name)).unwrap_or(&format!("RESOURCE_{}", name.to_uppercase())));

			// Insert.

			resource_elements.insert(String::from(name), resource_element);

		}

		// Sort and append.

		let mut sorted_resource_elements: Vec<(&String, &ResourceElement)> = resource_elements.iter().collect();
		let mut sorted_resource_category_elements: Vec<(&String, &ResourceCategoryElement)> = resource_category_elements.iter().collect();

		sorted_resource_elements.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));
		sorted_resource_category_elements.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));

		for (name, element) in sorted_resource_elements.iter() {

			let resource = stuff_manager.get_resource(name).unwrap();
			let category_element = resource_category_elements.get(resource.get_category()).unwrap();

			category_element.list_element.append_with_node_1(&element.root_element).unwrap();

		}

		for (_, element) in sorted_resource_category_elements.iter() {

			resource_container_element.append_with_node_1(&element.root_element).unwrap();

		}

		Self {
	
			resource_category_elements,
			resource_container_element,
			resource_elements,
	
		}
	
	}

	/// Renders resource container.
	pub fn render(&mut self, stuff_manager: &StuffManager) {

		for (name, resource) in stuff_manager.iter_resource() {

			let resource_element = self.resource_elements.get_mut(name).unwrap();

			if resource.is_unlocked() && !resource_element.is_unlocked {

				resource_element.is_unlocked = true;
				resource_element.root_element.class_list().remove_1("locked").unwrap();

				self.resource_category_elements
					.get_mut(resource.get_category())
					.map(|c| {
						
						if !c.is_unlocked {

							c.is_unlocked = true;
							c.root_element.class_list().remove_1("locked").unwrap();
					
						}

					});

			}

			if resource_element.is_unlocked {

				resource_element.count_element.set_inner_html(&format_number_scientific(resource.get_count()));
				resource_element.capacity_element.set_inner_html(&format_number_scientific(resource.get_capacity()));
				resource_element.production_element.set_inner_html(&format!("{}/s", &format_number_scientific((resource.get_production() - resource.get_consumption()) * 5f64)));

			}

		}

	}

}
