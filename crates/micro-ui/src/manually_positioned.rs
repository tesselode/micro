use micro::{Context, math::Vec2};

use crate::{
	LayoutResult, Widget, WidgetInspector, WidgetMouseState, common_functions,
	common_widget_trait_functions,
};

#[derive(Debug)]
pub struct ManuallyPositioned {
	children: Vec<Box<dyn Widget>>,
	child_positions: Vec<ChildPosition>,
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

	pub fn child(
		mut self,
		position: impl Into<ChildPosition>,
		child: impl Widget + 'static,
	) -> Self {
		self.children.push(Box::new(child));
		self.child_positions.push(position.into());
		self
	}

	pub fn children(
		mut self,
		children: impl IntoIterator<Item = (impl Into<ChildPosition>, impl Widget + 'static)>,
	) -> Self {
		for (position, child) in children {
			self.children.push(Box::new(child));
			self.child_positions.push(position.into());
		}
		self
	}

	pub fn child_if<P: Into<ChildPosition>, W: Widget + 'static>(
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
		children: impl IntoIterator<Item = (impl Into<ChildPosition>, impl Widget + 'static)>,
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

	pub fn child_if_some<T, P: Into<ChildPosition>, W: Widget + 'static>(
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

	pub fn children_if_some<
		T,
		W: IntoIterator<Item = (impl Into<ChildPosition>, impl Widget + 'static)>,
	>(
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
			child_positions: self
				.child_positions
				.iter()
				.map(|child_position| child_position.get(allotted_size_from_parent))
				.collect(),
		}
	}
}

pub enum ChildPosition {
	Static(Vec2),
	Dynamic(Box<dyn Fn(Vec2) -> Vec2>),
}

impl ChildPosition {
	fn get(&self, size: Vec2) -> Vec2 {
		match self {
			ChildPosition::Static(position) => *position,
			ChildPosition::Dynamic(f) => f(size),
		}
	}
}

impl std::fmt::Debug for ChildPosition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Static(arg0) => f.debug_tuple("Static").field(arg0).finish(),
			Self::Dynamic(_) => f.debug_tuple("Dynamic").finish(),
		}
	}
}

impl<T> From<T> for ChildPosition
where
	T: Fn(Vec2) -> Vec2 + 'static,
{
	fn from(value: T) -> Self {
		Self::Dynamic(Box::new(value))
	}
}

impl From<Vec2> for ChildPosition {
	fn from(value: Vec2) -> Self {
		Self::Static(value)
	}
}

impl From<(f32, f32)> for ChildPosition {
	fn from(value: (f32, f32)) -> Self {
		Self::Static(value.into())
	}
}

impl From<[f32; 2]> for ChildPosition {
	fn from(value: [f32; 2]) -> Self {
		Self::Static(value.into())
	}
}
