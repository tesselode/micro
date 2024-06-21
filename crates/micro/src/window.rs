use glam::UVec2;
use sdl2::{video::Window, VideoSubsystem};

use crate::ContextSettings;

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

pub(crate) fn build_window(video: &VideoSubsystem, settings: &ContextSettings) -> Window {
	let window_size = match settings.window_mode {
		// doesn't matter because we're going to set the window to fullscreen
		WindowMode::Fullscreen => UVec2::new(800, 600),
		WindowMode::Windowed { size } => size,
	};
	let mut window_builder = video.window(&settings.window_title, window_size.x, window_size.y);
	window_builder.allow_highdpi();
	if settings.window_mode == WindowMode::Fullscreen {
		window_builder.fullscreen_desktop();
	}
	window_builder.opengl();
	if settings.resizable {
		window_builder.resizable();
	}
	window_builder.build().expect("error building window")
}
