use glam::UVec2;

pub enum WindowMode {
	Fullscreen,
	Windowed { size: UVec2 },
}
