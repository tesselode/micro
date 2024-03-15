use glam::Vec2;

use crate::math::Rect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NineSlice {
	pub texture_region: Rect,
	pub left: f32,
	pub right: f32,
	pub top: f32,
	pub bottom: f32,
}

impl NineSlice {
	pub(crate) fn slices(self, mut display_rect: Rect) -> [Slice; 9] {
		display_rect.size = display_rect
			.size
			.max(Vec2::new(self.left + self.right, self.top + self.bottom));
		let top_left = Slice {
			display_rect: Rect::new(display_rect.top_left, Vec2::new(self.left, self.top)),
			texture_region: Rect::new(self.texture_region.top_left, Vec2::new(self.left, self.top)),
		};
		let top_right = Slice {
			display_rect: Rect::new(
				display_rect.top_right() - Vec2::new(self.right, 0.0),
				Vec2::new(self.left, self.top),
			),
			texture_region: Rect::new(
				self.texture_region.top_right() - Vec2::new(self.right, 0.0),
				Vec2::new(self.left, self.top),
			),
		};
		let bottom_left = Slice {
			display_rect: Rect::new(
				display_rect.bottom_left() - Vec2::new(0.0, self.bottom),
				Vec2::new(self.left, self.bottom),
			),
			texture_region: Rect::new(
				self.texture_region.bottom_left() - Vec2::new(0.0, self.bottom),
				Vec2::new(self.left, self.bottom),
			),
		};
		let bottom_right = Slice {
			display_rect: Rect::new(
				display_rect.bottom_right() - Vec2::new(self.right, self.bottom),
				Vec2::new(self.right, self.bottom),
			),
			texture_region: Rect::new(
				self.texture_region.bottom_right() - Vec2::new(self.right, self.bottom),
				Vec2::new(self.right, self.bottom),
			),
		};
		let left = Slice {
			display_rect: Rect::new(
				display_rect.top_left + Vec2::new(0.0, self.top),
				Vec2::new(self.left, display_rect.size.y - self.top - self.bottom),
			),
			texture_region: Rect::new(
				self.texture_region.top_left + Vec2::new(0.0, self.top),
				Vec2::new(
					self.left,
					self.texture_region.size.y - self.top - self.bottom,
				),
			),
		};
		let right = Slice {
			display_rect: Rect::new(
				display_rect.top_right() + Vec2::new(-self.right, self.top),
				Vec2::new(self.right, display_rect.size.y - self.top - self.bottom),
			),
			texture_region: Rect::new(
				self.texture_region.top_right() + Vec2::new(-self.right, self.top),
				Vec2::new(
					self.right,
					self.texture_region.size.y - self.top - self.bottom,
				),
			),
		};
		let top = Slice {
			display_rect: Rect::new(
				display_rect.top_left + Vec2::new(self.left, 0.0),
				Vec2::new(display_rect.size.x - self.left - self.right, self.top),
			),
			texture_region: Rect::new(
				self.texture_region.top_left + Vec2::new(self.left, 0.0),
				Vec2::new(
					self.texture_region.size.x - self.left - self.right,
					self.top,
				),
			),
		};
		let bottom = Slice {
			display_rect: Rect::new(
				display_rect.bottom_left() + Vec2::new(self.left, -self.bottom),
				Vec2::new(display_rect.size.x - self.left - self.right, self.bottom),
			),
			texture_region: Rect::new(
				self.texture_region.bottom_left() + Vec2::new(self.left, -self.bottom),
				Vec2::new(
					self.texture_region.size.x - self.left - self.right,
					self.bottom,
				),
			),
		};
		let center = Slice {
			display_rect: Rect::from_corners(
				display_rect.top_left + Vec2::new(self.left, self.top),
				display_rect.bottom_right() - Vec2::new(self.right, self.bottom),
			),
			texture_region: Rect::from_corners(
				self.texture_region.top_left + Vec2::new(self.left, self.top),
				self.texture_region.bottom_right() - Vec2::new(self.right, self.bottom),
			),
		};
		[
			top_left,
			top_right,
			bottom_left,
			bottom_right,
			left,
			right,
			top,
			bottom,
			center,
		]
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Slice {
	pub display_rect: Rect,
	pub texture_region: Rect,
}
