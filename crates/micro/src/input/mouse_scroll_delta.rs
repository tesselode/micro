use glam::{Vec2, vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseScrollDelta {
	Line(Vec2),
	Pixel(Vec2),
}

impl From<winit::event::MouseScrollDelta> for MouseScrollDelta {
	fn from(value: winit::event::MouseScrollDelta) -> Self {
		match value {
			winit::event::MouseScrollDelta::LineDelta(x, y) => Self::Line(vec2(x, y)),
			winit::event::MouseScrollDelta::PixelDelta(physical_position) => {
				Self::Pixel(vec2(physical_position.x as f32, physical_position.y as f32))
			}
		}
	}
}
