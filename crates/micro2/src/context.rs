pub(crate) mod graphics;

use std::{cell::RefCell, sync::OnceLock, time::Instant};

use glam::Mat4;
use sdl3::{EventPump, GamepadSubsystem, video::Window};
use wgpu::{Features, PresentMode};

use crate::{
	App, Event, FrameTimeTracker, WindowMode, build_window,
	context::graphics::GraphicsContext,
	graphics::{BlendMode, Shader, StencilState},
	math::URect,
};

pub fn run<A, F>(settings: ContextSettings, mut app_constructor: F)
where
	A: App,
	F: FnMut() -> A,
{
	Context::init(settings);
	// let egui_ctx = egui::Context::default();
	// let mut egui_textures = HashMap::new();
	let mut app = app_constructor();

	let mut last_update_time = Instant::now();

	loop {
		// measure and record delta time
		let now = Instant::now();
		let delta_time = now - last_update_time;
		last_update_time = now;
		Context::with_mut(|ctx| ctx.frame_time_tracker.record(delta_time));

		// poll for events
		let span = tracy_client::span!("poll events");
		let mut events = Context::with_mut(|ctx| ctx.event_pump.poll_iter().collect::<Vec<_>>());
		drop(span);

		// create egui UI
		/* let span = tracy_client::span!("create egui UI");
		let egui_input = egui_raw_input(&ctx, &events, delta_time);
		egui_ctx.begin_pass(egui_input);
		app.debug_ui(&mut ctx, &egui_ctx)?;
		let egui_output = egui_ctx.end_pass();
		drop(span); */

		// dispatch events to state
		let span = tracy_client::span!("dispatch events");
		for event in events
			.drain(..)
			// .filter(|event| !egui_took_sdl3_event(&egui_ctx, event))
			.filter_map(Event::from_sdl3_event)
		{
			match event {
				Event::WindowSizeChanged(size) => {
					Context::with_mut(|ctx| ctx.graphics.resize(size))
				}
				Event::Exited => Context::with_mut(|ctx| ctx.should_quit = true),
				_ => {}
			}
			app.event(event);
		}
		drop(span);
		// ctx.egui_wants_keyboard_input = egui_ctx.wants_keyboard_input();
		// ctx.egui_wants_mouse_input = egui_ctx.wants_pointer_input();

		// update state
		let span = tracy_client::span!("update");
		app.update(delta_time);
		drop(span);

		// draw state and egui UI
		let span = tracy_client::span!("draw");

		drop(span);
		app.draw();
		/* let span = tracy_client::span!("draw egui UI");
		draw_egui_output(&mut ctx, &egui_ctx, egui_output, &mut egui_textures);
		drop(span); */
		Context::with_mut(|ctx| ctx.graphics.present());

		tracy_client::frame_mark();

		if Context::with(|ctx| ctx.should_quit) {
			break;
		}
	}
}

thread_local! {
	pub(crate) static CONTEXT: OnceLock<RefCell<Context>> = const { OnceLock::new() };
}

pub(crate) struct Context {
	pub(crate) gamepad: GamepadSubsystem,
	pub(crate) event_pump: EventPump,
	// `graphics` needs to be before `window`, since it holds
	// a `Surface` that must be dropped before the `Window`
	pub(crate) graphics: GraphicsContext,
	pub(crate) window: Window,
	pub(crate) frame_time_tracker: FrameTimeTracker,
	pub(crate) should_quit: bool,
}

impl Context {
	pub(crate) fn with<T>(mut f: impl FnMut(&Context) -> T) -> T {
		CONTEXT.with(|ctx| f(&ctx.get().unwrap().borrow()))
	}

	pub(crate) fn with_mut<T>(mut f: impl FnMut(&mut Context) -> T) -> T {
		CONTEXT.with(|ctx| f(&mut ctx.get().unwrap().borrow_mut()))
	}

	fn init(settings: ContextSettings) {
		CONTEXT.with(|ctx| {
			ctx.set(RefCell::new(Self::new(settings)))
				.unwrap_or_else(|_| panic!("context is already initialized"));
		});
	}

	fn new(settings: ContextSettings) -> Self {
		let sdl = sdl3::init().expect("error initializing SDL");
		let video = sdl.video().expect("error initializing video subsystem");
		let gamepad = sdl
			.gamepad()
			.expect("error initializing controller subsystem");
		let window = build_window(&video, &settings);
		let event_pump = sdl.event_pump().expect("error creating event pump");
		let graphics = GraphicsContext::new(&window, &settings);

		Context {
			window,
			gamepad,
			event_pump,
			// egui_wants_keyboard_input: false,
			// egui_wants_mouse_input: false,
			frame_time_tracker: FrameTimeTracker::new(),
			graphics,
			should_quit: false,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	pub window_title: String,
	pub window_mode: WindowMode,
	pub resizable: bool,
	pub present_mode: PresentMode,
	pub max_queued_frames: u32,
	pub required_graphics_features: Features,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_mode: WindowMode::default(),
			resizable: false,
			present_mode: PresentMode::AutoVsync,
			max_queued_frames: 1,
			required_graphics_features: Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Push {
	pub transform: Option<Mat4>,
	pub shader: Option<Shader>,
	pub blend_mode: Option<BlendMode>,
	pub stencil_state: Option<StencilState>,
	pub enable_depth_testing: Option<bool>,
	pub scissor_rect: Option<URect>,
}

impl From<Mat4> for Push {
	fn from(transform: Mat4) -> Self {
		Self {
			transform: Some(transform),
			..Default::default()
		}
	}
}

impl From<&Shader> for Push {
	fn from(shader: &Shader) -> Self {
		Self {
			shader: Some(shader.clone()),
			..Default::default()
		}
	}
}

impl From<BlendMode> for Push {
	fn from(blend_mode: BlendMode) -> Self {
		Self {
			blend_mode: Some(blend_mode),
			..Default::default()
		}
	}
}

impl From<StencilState> for Push {
	fn from(stencil_state: StencilState) -> Self {
		Self {
			stencil_state: Some(stencil_state),
			..Default::default()
		}
	}
}

#[must_use]
pub struct OnDrop;

impl Drop for OnDrop {
	fn drop(&mut self) {
		Context::with_mut(|ctx| ctx.graphics.pop_graphics_state());
	}
}
