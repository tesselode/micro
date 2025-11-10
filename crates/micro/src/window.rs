use glam::UVec2;
use sdl3::{VideoSubsystem, video::Window};

use crate::context::ContextSettings;

/// The size and type of a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowMode {
	/// The window covers the entire screen.
	Fullscreen,
	/// The window covers a portion of the screen.
	Windowed {
		/// How big the window is.
		size: UVec2,
	},
}

impl Default for WindowMode {
	fn default() -> Self {
		Self::Windowed {
			size: UVec2::new(1280, 720),
		}
	}
}

pub(crate) fn build_window(video: &VideoSubsystem, settings: &ContextSettings) -> Window {
	let window_size = match settings.window_mode {
		// doesn't matter because we're going to set the window to fullscreen
		WindowMode::Fullscreen => UVec2::new(1280, 720),
		WindowMode::Windowed { size } => size,
	};
	let mut window_builder = video.window(&settings.window_title, window_size.x, window_size.y);
	if settings.window_mode == WindowMode::Fullscreen {
		window_builder.fullscreen();
	}
	if settings.resizable {
		window_builder.resizable();
	}
	window_builder.build().expect("error building window")
}
