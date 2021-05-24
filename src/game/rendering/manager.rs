use crate::game::StuffManager;
use super::*;

pub struct RenderingManager {

	loading_manager: LoadingRenderer,
	resource_renderer: ResourceRenderer,
	tab_renderer: TabRenderer,

}

impl RenderingManager {

	pub fn new() -> Self {

		Self {

			loading_manager: LoadingRenderer::new(),
			resource_renderer: ResourceRenderer::new(),
			tab_renderer: TabRenderer::new(),
			
		}

	}

	pub fn change_tab(&mut self, name: &str, stuff_manager: &StuffManager) {

		self.tab_renderer.select(name);
		self.render(stuff_manager);

	}
	
	pub fn init(&mut self, stuff_manager: &StuffManager) {

		self.resource_renderer.init(stuff_manager);
		self.tab_renderer.init(stuff_manager);

	}

	pub fn render(&mut self, stuff_manager: &StuffManager) {

		self.resource_renderer.render(stuff_manager);
		self.tab_renderer.render(stuff_manager);

	}

	pub fn set_loading(&self, loading: bool) {

		self.loading_manager.set_loading(loading);

	}

	pub fn set_loading_description(&self, description: &str) {

		self.loading_manager.set_loading_description(description);

	}

}