use indexmap::IndexMap;

pub struct ChildPathGenerator {
	num_widgets: IndexMap<&'static str, usize>,
}

impl ChildPathGenerator {
	pub fn new() -> Self {
		Self {
			num_widgets: IndexMap::new(),
		}
	}

	pub fn generate(&mut self, widget_name: &'static str) -> String {
		let num_widgets_with_name = self.num_widgets.entry(widget_name).or_default();
		let path = format!("{}{}", widget_name, *num_widgets_with_name);
		*num_widgets_with_name += 1;
		path
	}
}

impl Default for ChildPathGenerator {
	fn default() -> Self {
		Self::new()
	}
}
