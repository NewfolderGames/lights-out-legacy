use crate::game::StuffManager;
use super::{ ResourceRenderer, TabRenderer };

pub struct RenderingManager {

	resource_renderer: ResourceRenderer,
	tab_renderer: TabRenderer,

}

impl RenderingManager {

	pub fn new() -> Self {

		Self {

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

}