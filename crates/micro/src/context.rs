pub(crate) mod graphics;
pub(crate) mod text;

mod push;

pub use push::*;

use winit::{
	application::ApplicationHandler,
	dpi::{PhysicalSize, Size},
	event::WindowEvent,
	event_loop::{ActiveEventLoop, EventLoop},
	window::{Fullscreen, Window, WindowId},
};

use std::{
	fmt::Debug,
	ops::{Deref, DerefMut},
	path::Path,
	sync::Arc,
};

use glam::{Mat4, UVec2, Vec2, Vec3, uvec2};
use wgpu::{Features, PresentMode, TextureFormat};

use crate::{
	App, WindowMode, build_window,
	context::graphics::GraphicsContext,
	graphics::{IntoScale2d, IntoScale3d},
	text::TextContext,
};

type AppConstructor<A> = dyn FnMut(&mut Context) -> Result<A, anyhow::Error> + 'static;

/// Starts a Micro application. The app constructor should return a value of a type
/// that implements [`App`].
pub fn run<A, F>(settings: ContextSettings, app_constructor: F) -> anyhow::Result<()>
where
	A: App,
	F: FnMut(&mut Context) -> anyhow::Result<A> + 'static,
{
	let event_loop = EventLoop::new()?;
	let mut app_handler = MicroAppHandler::new(settings, Box::new(app_constructor));
	event_loop.run_app(&mut app_handler)?;
	Ok(())
}

/// Allows you to interact with Micro to check for keyboard inputs,
/// draw graphics, change window settings, etc.
pub struct Context {
	window: Arc<Window>,
	pub(crate) graphics: GraphicsContext,
	pub(crate) text: TextContext,
}

impl Context {
	/// Gets the drawable size of the window (in pixels).
	pub fn window_size(&self) -> UVec2 {
		let PhysicalSize { width, height } = self.window.inner_size();
		UVec2::new(width, height)
	}

	/// Returns the number of pixels per logical point on screen.
	pub fn window_scale(&self) -> f32 {
		self.window
			.current_monitor()
			.expect("could not get monitor for window")
			.scale_factor() as f32
	}

	/// Returns the current window mode (windowed or fullscreen).
	pub fn window_mode(&self) -> WindowMode {
		match self.window.fullscreen() {
			Some(Fullscreen::Borderless(_)) => WindowMode::Fullscreen,
			None => WindowMode::Windowed {
				size: uvec2(
					self.window.inner_size().width,
					self.window.inner_size().height,
				),
			},
			_ => unreachable!("no other fullscreen modes are supported"),
		}
	}

	/// Returns the resolution of the monitor the window is on.
	pub fn monitor_resolution(&self) -> UVec2 {
		let size = self
			.window
			.current_monitor()
			.expect("could not get monitor for window")
			.size();
		UVec2::new(size.width, size.height)
	}

	/* /// Returns `true` if integer scaling is enabled. Only relevant if the
	/// context was set up to use a main canvas.
	pub fn integer_scaling_enabled(&self) -> bool {
		self.integer_scaling_enabled
	} */

	/// Sets the window mode (windowed or fullscreen).
	pub fn set_window_mode(&mut self, window_mode: WindowMode) {
		match window_mode {
			WindowMode::Fullscreen => {
				self.window
					.set_fullscreen(Some(Fullscreen::Borderless(None)));
			}
			WindowMode::Windowed { size } => {
				self.window.set_fullscreen(None);
				let new_size = self.window.request_inner_size(Size::Physical(PhysicalSize {
					width: size.x,
					height: size.y,
				}));
				if let Some(new_size) = new_size {
					self.resize(uvec2(new_size.width, new_size.height));
				}
			}
		}
	}

	/// Sets whether integer scaling is enabled. Only relevant if the
	/// context was set up to use a main canvas.
	/* pub fn set_integer_scaling_enabled(&mut self, enabled: bool) {
		self.integer_scaling_enabled = enabled;
	} */

	/// Returns the current [`PresentMode`].
	pub fn present_mode(&self) -> PresentMode {
		self.graphics.present_mode()
	}

	/// Returns the desired maximum number of frames that can be queued up
	/// ahead of time.
	pub fn desired_maximum_frame_latency(&self) -> u32 {
		self.graphics.desired_maximum_frame_latency()
	}

	/// Returns the texture format of the window surface.
	pub fn surface_format(&self) -> TextureFormat {
		self.graphics.surface_format()
	}

	/**
	Returns the size in pixels of the current render target, which is
	either:
	- The current canvas being rendered to if there is one, or
	- The window
	*/
	pub fn current_render_target_size(&self) -> UVec2 {
		self.graphics.current_render_target_size()
	}

	/// Sets the [`PresentMode`].
	pub fn set_present_mode(&mut self, present_mode: PresentMode) {
		self.graphics.set_present_mode(present_mode);
	}

	/// Sets the desired maximum number of frames that can be queued up
	/// ahead of time. Higher values can stabilize framerates, but increase
	/// input lag.
	pub fn set_desired_maximum_frame_latency(&mut self, frames: u32) {
		self.graphics.set_desired_maximum_frame_latency(frames);
	}

	/// Returns the sample counts for MSAA that the graphics card supports.
	pub fn supported_sample_counts(&self) -> &[u32] {
		// TODO: figure out if this function needs a TextureFormat argument to be accurate
		&self.graphics.supported_sample_counts
	}

	/// Sets the color the window surface will be cleared to at the start
	/// of each frame.
	/* pub fn set_clear_color(&mut self, color: impl Into<LinSrgb>) {
		let color = color.into();
		self.clear_color = color;
		if self.main_canvas_size.is_none() {
			self.graphics.clear_color = color;
		}
	} */

	/// Pushes a set of graphics settings that will be used for upcoming
	/// drawing operations. Returns an object which, when dropped, will
	/// restore the previous set of graphics settings.
	pub fn push(&mut self, push: impl Into<Push>) -> OnDrop<'_> {
		self.graphics.push_graphics_state(push.into());
		OnDrop { ctx: self }
	}

	/// Pushes a transformation that translates all drawing operations by the
	/// specified amount along the X and Y axes.
	pub fn push_translation_2d(&mut self, translation: impl Into<Vec2>) -> OnDrop<'_> {
		self.push(Mat4::from_translation(translation.into().extend(0.0)))
	}

	/// Pushes a transformation that translates all drawing operations by the
	/// specified amount along the X, Y, and Z axes.
	pub fn push_translation_3d(&mut self, translation: impl Into<Vec3>) -> OnDrop<'_> {
		self.push(Mat4::from_translation(translation.into()))
	}

	/// Pushes a transformation that translates all drawing operations by the
	/// specified amount along the X axis.
	pub fn push_translation_x(&mut self, translation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_translation(Vec3::new(translation, 0.0, 0.0)))
	}

	/// Pushes a transformation that translates all drawing operations by the
	/// specified amount along the Y axis.
	pub fn push_translation_y(&mut self, translation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_translation(Vec3::new(0.0, translation, 0.0)))
	}

	/// Pushes a transformation that translates all drawing operations by the
	/// specified amount along the Z axis.
	pub fn push_translation_z(&mut self, translation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_translation(Vec3::new(0.0, 0.0, translation)))
	}

	/// Pushes a transformation that scales all drawing operations by the
	/// specified amount along the X and Y axes.
	pub fn push_scale_2d(&mut self, scale: impl IntoScale2d) -> OnDrop<'_> {
		self.push(Mat4::from_scale(scale.into_scale_2d().extend(0.0)))
	}

	/// Pushes a transformation that scales all drawing operations by the
	/// specified amount along the X, Y, and Z axes.
	pub fn push_scale_3d(&mut self, scale: impl IntoScale3d) -> OnDrop<'_> {
		self.push(Mat4::from_scale(scale.into_scale_3d()))
	}

	/// Pushes a transformation that scales all drawing operations by the
	/// specified amount along the X axis.
	pub fn push_scale_x(&mut self, scale: f32) -> OnDrop<'_> {
		self.push(Mat4::from_scale(Vec3::new(scale, 1.0, 1.0)))
	}

	/// Pushes a transformation that scales all drawing operations by the
	/// specified amount along the Y axis.
	pub fn push_scale_y(&mut self, scale: f32) -> OnDrop<'_> {
		self.push(Mat4::from_scale(Vec3::new(1.0, scale, 1.0)))
	}

	/// Pushes a transformation that scales all drawing operations by the
	/// specified amount along the Z axis.
	pub fn push_scale_z(&mut self, scale: f32) -> OnDrop<'_> {
		self.push(Mat4::from_scale(Vec3::new(1.0, 1.0, scale)))
	}

	/// Pushes a transformation that rotates all drawing operations by the
	/// specified amount around the X axis.
	pub fn push_rotation_x(&mut self, rotation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_rotation_x(rotation))
	}

	/// Pushes a transformation that rotates all drawing operations by the
	/// specified amount around the Y axis.
	pub fn push_rotation_y(&mut self, rotation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_rotation_y(rotation))
	}

	/// Pushes a transformation that rotates all drawing operations by the
	/// specified amount around the Z axis.
	pub fn push_rotation_z(&mut self, rotation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_rotation_z(rotation))
	}

	pub fn load_font_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
		self.text.load_font_file(path)
	}

	pub fn load_fonts_dir(&mut self, path: impl AsRef<Path>) {
		self.text.load_fonts_dir(path);
	}

	async fn new(settings: &ContextSettings, window: Arc<Window>) -> anyhow::Result<Self> {
		let graphics = GraphicsContext::new(window.clone(), settings);
		let text = TextContext::new(&graphics);
		Ok(Self {
			window,
			graphics,
			text,
		})
	}

	fn resize(&mut self, size: UVec2) {
		self.graphics.resize(size);
	}

	fn render(&mut self) {
		self.graphics.present();
	}
}

/// Settings for starting an application.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	/// The title of the application window.
	pub window_title: String,
	/// The size and fullscreen state of the window.
	pub window_mode: WindowMode,
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
			window_mode: WindowMode::default(),
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

/// Restores the previous graphics settings when dropped. Returned by
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

/* fn main_canvas_transform(canvas_size: UVec2, window_size: UVec2, integer_scale: bool) -> Mat4 {
	let max_horizontal_scale = window_size.x as f32 / canvas_size.x as f32;
	let max_vertical_scale = window_size.y as f32 / canvas_size.y as f32;
	let mut scale = max_horizontal_scale.min(max_vertical_scale);
	if integer_scale {
		scale = scale.floor();
	}
	Mat4::from_translation((window_size.as_vec2() / 2.0).extend(0.0))
		* Mat4::from_scale(Vec3::splat(scale))
		* Mat4::from_translation((-canvas_size.as_vec2() / 2.0).extend(0.0))
} */

struct MicroAppHandler<A: App> {
	settings: ContextSettings,
	status: Status<A>,
}

impl<A: App> MicroAppHandler<A> {
	pub fn new(settings: ContextSettings, app_constructor: Box<AppConstructor<A>>) -> Self {
		Self {
			status: Status::Uninitialized { app_constructor },
			settings,
		}
	}
}

impl<A: App> ApplicationHandler for MicroAppHandler<A> {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let Status::Uninitialized { app_constructor } = &mut self.status else {
			return;
		};
		let window = build_window(event_loop, &self.settings);
		let mut ctx = pollster::block_on(Context::new(&self.settings, window))
			.expect("error creating context");
		let app = app_constructor(&mut ctx).expect("error initializing app");
		self.status = Status::Initialized { app, ctx };
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		let Status::Initialized { app, ctx } = &mut self.status else {
			return;
		};

		match event {
			WindowEvent::CloseRequested => event_loop.exit(),
			WindowEvent::Resized(size) => ctx.resize(uvec2(size.width, size.height)),
			WindowEvent::RedrawRequested => {
				app.draw(ctx).expect("error while drawing");
				ctx.render();
			}
			_ => {}
		}
	}
}

enum Status<A: App> {
	Uninitialized {
		app_constructor: Box<AppConstructor<A>>,
	},
	Initialized {
		app: A,
		ctx: Context,
	},
}
