use micro::{Context, math::Vec2};

use crate::{
	LayoutResult, Widget, WidgetInspector, WidgetMouseState, common_functions,
	common_widget_trait_functions,
};

#[derive(Debug)]
pub struct ManuallyPositioned {
	children: Vec<Box<dyn Widget>>,
	child_positions: Vec<Vec2>,
	mouse_state: Option<WidgetMouseState>,
	inspector: Option<WidgetInspector>,
}

impl ManuallyPositioned {
	pub fn new() -> Self {
		Self {
			children: vec![],
			child_positions: vec![],
			mouse_state: None,
			inspector: None,
		}
	}

	pub fn child(mut self, position: impl Into<Vec2>, child: impl Widget + 'static) -> Self {
		self.children.push(Box::new(child));
		self.child_positions.push(position.into());
		self
	}

	pub fn children(
		mut self,
		children: impl IntoIterator<Item = (impl Into<Vec2>, impl Widget + 'static)>,
	) -> Self {
		for (position, child) in children {
			self.children.push(Box::new(child));
			self.child_positions.push(position.into());
		}
		self
	}

	pub fn child_if<P: Into<Vec2>, W: Widget + 'static>(
		mut self,
		condition: bool,
		child: impl FnOnce() -> (P, W),
	) -> Self {
		if condition {
			let (position, child) = child();
			self.children.push(Box::new(child));
			self.child_positions.push(position.into());
		}
		self
	}

	pub fn children_if(
		mut self,
		condition: bool,
		children: impl IntoIterator<Item = (impl Into<Vec2>, impl Widget + 'static)>,
	) -> Self {
		if !condition {
			return self;
		}
		for (position, child) in children {
			self.children.push(Box::new(child));
			self.child_positions.push(position.into());
		}
		self
	}

	pub fn child_if_some<T, P: Into<Vec2>, W: Widget + 'static>(
		mut self,
		value: &Option<T>,
		child: impl FnOnce(&T) -> (P, W),
	) -> Self {
		if let Some(value) = value {
			let (position, child) = child(value);
			self.children.push(Box::new(child));
			self.child_positions.push(position.into());
		}
		self
	}

	pub fn children_if_some<T, W: IntoIterator<Item = (impl Into<Vec2>, impl Widget + 'static)>>(
		mut self,
		value: &Option<T>,
		children: impl FnOnce(&T) -> W,
	) -> Self {
		if let Some(value) = value {
			for (position, child) in children(value) {
				self.children.push(Box::new(child));
				self.child_positions.push(position.into());
			}
		}
		self
	}

	common_functions!();
}

impl Default for ManuallyPositioned {
	fn default() -> Self {
		Self::new()
	}
}

impl Widget for ManuallyPositioned {
	common_widget_trait_functions!();

	fn name(&self) -> &'static str {
		"manually positioned"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&self.children
	}

	fn allotted_size_for_next_child(
		&self,
		_allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
	) -> Vec2 {
		Vec2::INFINITY
	}

	fn layout(
		&self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		_child_sizes: &[Vec2],
	) -> LayoutResult {
		LayoutResult {
			size: allotted_size_from_parent,
			child_positions: self.child_positions.clone(),
		}
	}
}
