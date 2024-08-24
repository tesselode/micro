use glam::{vec3, Mat4, Vec2};

use crate::{with_child_fns, with_sizing_fns, Context};

use super::{Sizing, Widget};

#[derive(Debug)]
pub struct Transform {
	sizing: Sizing,
	origin: Vec2,
	transform: Mat4,
	children: Vec<Box<dyn Widget>>,
	size: Option<Vec2>,
}

impl Transform {
	pub fn new(transform: impl Into<Mat4>) -> Self {
		Self {
			sizing: Sizing::MIN,
			origin: Vec2::ZERO,
			transform: transform.into(),
			children: vec![],
			size: None,
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

	with_child_fns!();
	with_sizing_fns!();
}

impl Widget for Transform {
	fn size(&mut self, ctx: &mut Context, allotted_size: Vec2) -> Vec2 {
		let allotted_size_for_children = self.sizing.allotted_size_for_children(allotted_size);
		let child_sizes = self
			.children
			.iter_mut()
			.map(|child| child.size(ctx, allotted_size_for_children));
		let parent_size = self.sizing.final_parent_size(allotted_size, child_sizes);
		self.size = Some(parent_size);
		parent_size
	}

	fn draw(&self, ctx: &mut Context) -> anyhow::Result<()> {
		let origin_transform =
			Mat4::from_translation((self.size.unwrap() * -self.origin).extend(0.0));
		let ctx =
			&mut ctx.push_transform(origin_transform.inverse() * self.transform * origin_transform);
		for child in &self.children {
			child.draw(ctx)?;
		}
		Ok(())
	}
}
