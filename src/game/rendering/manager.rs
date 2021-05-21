use crate::game::stuff::StuffManager;
use super::ResourceRenderer;

pub struct RenderingManager {

	resource_renderer: ResourceRenderer,

}

impl RenderingManager {

	pub fn new() -> Self {

		Self {

			resource_renderer: ResourceRenderer::new()
			
		}

	}
	
	pub fn init(&mut self, stuff_manager: &StuffManager) {

		self.resource_renderer.init(stuff_manager);

	}

	pub fn tick(&mut self, stuff_manager: &StuffManager) {

		self.resource_renderer.render(stuff_manager);

	}

}