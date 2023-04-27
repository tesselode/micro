use glam::UVec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowMode {
	Fullscreen,
	Windowed { size: UVec2 },
}

impl Default for WindowMode {
	fn default() -> Self {
		Self::Windowed {
			size: UVec2::new(800, 600),
		}
	}
}
