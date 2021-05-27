use crate::game::stuff::StuffManager;

pub trait Tab {

	/// Returns `true` if the tab is selected.
	fn is_selected(&self) -> bool;

	/// Renders tab.
	fn render(&mut self, stuff_manager: &StuffManager);

	/// Selects this tab.
	fn set_selected(&mut self, selected: bool);

}
