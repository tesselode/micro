// pub(crate) mod graphics;
// pub(crate) mod text;

// mod push;

// pub use push::*;

use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::{ActiveEventLoop, EventLoop},
	window::{Window, WindowAttributes, WindowId},
};

use std::{fmt::Debug, sync::Arc};

use glam::{UVec2, uvec2};
use wgpu::{Features, PresentMode};

// use crate::WindowMode;

pub fn run(settings: ContextSettings) -> anyhow::Result<()> {
	let event_loop = EventLoop::new()?;
	let mut app_handler = MicroAppHandler::new();
	event_loop.run_app(&mut app_handler);
	Ok(())
}

pub struct Context {
	window: Arc<Window>,
}

impl Context {
	pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
		Ok(Self { window })
	}

	pub fn resize(&mut self, size: UVec2) {}

	pub fn render(&mut self) {
		self.window.request_redraw();
	}
}

/// Settings for starting an application.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	/// The title of the application window.
	pub window_title: String,
	/// The size and fullscreen state of the window.
	// pub window_mode: WindowMode,
	/// Whether the window is resizable.
	pub resizable: bool,
	pub main_canvas: Option<MainCanvasSettings>,
	/// The [`PresentMode`] used by the application.
	pub present_mode: PresentMode,
	/// The desired maximum number of frames that can be queued up
	/// ahead of time. Higher values can stabilize framerates, but increase
	/// input lag.
	pub desired_maximum_frame_latency: u32,
	/// A bitset of graphics features the application will use.
	pub required_graphics_features: Features,
	/// Whether dev tools should be enabled or not.
	pub dev_tools_mode: DevToolsMode,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			// window_mode: WindowMode::default(),
			resizable: false,
			main_canvas: None,
			present_mode: PresentMode::AutoVsync,
			desired_maximum_frame_latency: 1,
			required_graphics_features: Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
			dev_tools_mode: DevToolsMode::default(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MainCanvasSettings {
	pub size: UVec2,
	pub integer_scaling_enabled: bool,
}

/// Configures whether dev tools will be available on this run of the
/// application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DevToolsMode {
	/// Dev tools are disabled.
	Disabled,
	/// Dev tools are enabled.
	Enabled {
		/// Whether the dev tools UI should be shown by default.
		/// This can be toggled at runtime by pressing F1.
		show_by_default: bool,
	},
}

impl DevToolsMode {
	fn initial_state(self) -> DevToolsState {
		match self {
			DevToolsMode::Disabled => DevToolsState::Disabled,
			DevToolsMode::Enabled { show_by_default } => DevToolsState::Enabled {
				visible: show_by_default,
			},
		}
	}
}

impl Default for DevToolsMode {
	fn default() -> Self {
		Self::Enabled {
			show_by_default: true,
		}
	}
}

/// Whether the dev tools are currently enabled, and if so, whether the UI
/// is currently visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DevToolsState {
	/// Dev tools are disabled.
	Disabled,
	/// Dev tools are enabled.
	Enabled {
		/// Whether the dev tools UI is visible or not.
		visible: bool,
	},
}

/* /// Restores the previous graphics settings when dropped. Returned by
/// the `Context::push_*` functions.
#[must_use]
pub struct OnDrop<'a> {
	ctx: &'a mut Context,
}

impl Drop for OnDrop<'_> {
	fn drop(&mut self) {
		self.ctx.graphics.pop_graphics_state();
	}
}

impl Deref for OnDrop<'_> {
	type Target = Context;

	fn deref(&self) -> &Self::Target {
		self.ctx
	}
}

impl DerefMut for OnDrop<'_> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.ctx
	}
}

fn main_canvas_transform(canvas_size: UVec2, window_size: UVec2, integer_scale: bool) -> Mat4 {
	let max_horizontal_scale = window_size.x as f32 / canvas_size.x as f32;
	let max_vertical_scale = window_size.y as f32 / canvas_size.y as f32;
	let mut scale = max_horizontal_scale.min(max_vertical_scale);
	if integer_scale {
		scale = scale.floor();
	}
	Mat4::from_translation((window_size.as_vec2() / 2.0).extend(0.0))
		* Mat4::from_scale(Vec3::splat(scale))
		* Mat4::from_translation((-canvas_size.as_vec2() / 2.0).extend(0.0))
}
 */

struct MicroAppHandler {
	context: Option<Context>,
}

impl MicroAppHandler {
	pub fn new() -> Self {
		Self { context: None }
	}
}

impl ApplicationHandler for MicroAppHandler {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window = Arc::new(
			event_loop
				.create_window(WindowAttributes::default())
				.expect("error creating window"),
		);
		self.context =
			Some(pollster::block_on(Context::new(window)).expect("error creating context"));
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		let Some(context) = &mut self.context else {
			return;
		};

		match event {
			WindowEvent::CloseRequested => event_loop.exit(),
			WindowEvent::Resized(size) => context.resize(uvec2(size.width, size.height)),
			WindowEvent::RedrawRequested => context.render(),
			_ => {}
		}
	}
}
