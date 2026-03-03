use std::sync::Arc;

use glam::UVec2;
use winit::{
	dpi::{PhysicalSize, Size},
	event_loop::ActiveEventLoop,
	window::{Fullscreen, Window},
};

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

pub(crate) fn build_window(
	event_loop: &ActiveEventLoop,
	settings: &ContextSettings,
) -> Arc<Window> {
	let window_size = match settings.window_mode {
		// doesn't matter because we're going to set the window to fullscreen
		WindowMode::Fullscreen => UVec2::new(1280, 720),
		WindowMode::Windowed { size } => size,
	};
	let window = event_loop
		.create_window(
			Window::default_attributes()
				.with_inner_size(Size::Physical(PhysicalSize {
					width: window_size.x,
					height: window_size.y,
				}))
				.with_resizable(settings.resizable)
				.with_title(&settings.window_title)
				.with_fullscreen(
					(settings.window_mode == WindowMode::Fullscreen)
						.then_some(Fullscreen::Borderless(None)),
				),
		)
		.expect("error creating window");
	Arc::new(window)
}
