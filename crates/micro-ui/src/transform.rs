use micro::{
	Context,
	math::{Mat4, Vec2, vec3},
};

use crate::{with_child_fns, with_sizing_fns};

use super::{LayoutResult, Sizing, Widget, WidgetMouseEventChannel};

#[derive(Debug)]
pub struct Transform {
	sizing: Sizing,
	origin: Vec2,
	transform: Mat4,
	children: Vec<Box<dyn Widget>>,
	mouse_event_channel: Option<WidgetMouseEventChannel>,
}

impl Transform {
	pub fn new(transform: impl Into<Mat4>) -> Self {
		Self {
			sizing: Sizing::SHRINK,
			origin: Vec2::ZERO,
			transform: transform.into(),
			children: vec![],
			mouse_event_channel: None,
		}
	}

	pub fn translation(translation: impl Into<Vec2>) -> Self {
		Self::new(Mat4::from_translation(translation.into().extend(0.0)))
	}

	pub fn translation_x(translation: f32) -> Self {
		Self::new(Mat4::from_translation(vec3(translation, 0.0, 0.0)))
	}

	pub fn translation_y(translation: f32) -> Self {
		Self::new(Mat4::from_translation(vec3(0.0, translation, 0.0)))
	}

	pub fn scale(scale: impl Into<Vec2>) -> Self {
		Self::new(Mat4::from_scale(scale.into().extend(1.0)))
	}

	pub fn scale_x(scale: f32) -> Self {
		Self::new(Mat4::from_scale(vec3(scale, 1.0, 1.0)))
	}

	pub fn scale_y(scale: f32) -> Self {
		Self::new(Mat4::from_scale(vec3(1.0, scale, 1.0)))
	}

	pub fn rotation(rotation: f32) -> Self {
		Self::new(Mat4::from_rotation_z(rotation))
	}

	pub fn with_origin(self, origin: impl Into<Vec2>) -> Self {
		Self {
			origin: origin.into(),
			..self
		}
	}

	pub fn with_mouse_event_channel(self, channel: &WidgetMouseEventChannel) -> Self {
		Self {
			mouse_event_channel: Some(channel.clone()),
			..self
		}
	}

	with_child_fns!();
	with_sizing_fns!();
}

impl Widget for Transform {
	fn name(&self) -> &'static str {
		"transform"
	}

	fn children(&self) -> &[Box<dyn Widget>] {
		&self.children
	}

	fn transform(&self, size: Vec2) -> Mat4 {
		let origin_transform = Mat4::from_translation((size * -self.origin).extend(0.0));
		origin_transform.inverse() * self.transform * origin_transform
	}

	fn mouse_event_channel(&self) -> Option<&WidgetMouseEventChannel> {
		self.mouse_event_channel.as_ref()
	}

	fn allotted_size_for_next_child(
		&self,
		allotted_size_from_parent: Vec2,
		_previous_child_sizes: &[Vec2],
	) -> Vec2 {
		let _span = tracy_client::span!();
		self.sizing
			.allotted_size_for_children(allotted_size_from_parent)
	}

	fn layout(
		&self,
		_ctx: &mut Context,
		allotted_size_from_parent: Vec2,
		child_sizes: &[Vec2],
	) -> LayoutResult {
		let _span = tracy_client::span!();
		LayoutResult {
			size: self
				.sizing
				.final_parent_size(allotted_size_from_parent, child_sizes.iter().copied()),
			child_positions: std::iter::repeat(Vec2::ZERO)
				.take(child_sizes.len())
				.collect(),
		}
	}
}
