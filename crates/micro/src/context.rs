pub(crate) mod graphics;

use std::{
	cell::{OnceCell, RefCell},
	collections::HashMap,
	fmt::Debug,
	time::Instant,
};

use backtrace::Backtrace;
use glam::{Affine2, Mat4, UVec2, Vec2};
use palette::LinSrgba;
use sdl2::{
	video::{GLProfile, SwapInterval, Window},
	EventPump, GameControllerSubsystem, Sdl, VideoSubsystem,
};

use crate::{
	build_window, clear,
	egui_integration::{draw_egui_output, egui_raw_input, egui_took_sdl2_event},
	graphics::{Canvas, CanvasSettings, ColorConstants},
	log::setup_logging,
	log_if_err, push_transform,
	time::FrameTimeTracker,
	window::WindowMode,
	window_size, Event, State,
};

use self::graphics::GraphicsContext;

thread_local! {
	static CONTEXT: OnceCell<RefCell<Context>> = OnceCell::new();
}

/// Runs the game. Call this in your `main` function.
pub fn run<S, F, E>(settings: ContextSettings, state_constructor: F)
where
	S: State<E>,
	F: FnMut() -> Result<S, E>,
	E: Debug,
{
	#[cfg(debug_assertions)]
	setup_logging();
	#[cfg(not(debug_assertions))]
	let _guard = setup_logging(&settings);
	std::panic::set_hook(Box::new(|info| {
		tracing::error!("{}\n{:?}", info, Backtrace::new())
	}));
	log_if_err!(run_inner(settings, state_constructor));
}

fn run_inner<S, F, E>(settings: ContextSettings, mut state_constructor: F) -> Result<(), E>
where
	S: State<E>,
	F: FnMut() -> Result<S, E>,
{
	// create contexts and resources
	CONTEXT.with(|ctx| {
		ctx.set(RefCell::new(Context::new(&settings)))
			.unwrap_or_else(|_| panic!("context already initialized"));
	});
	let egui_ctx = egui::Context::default();
	let mut egui_textures = HashMap::new();
	let mut state = state_constructor()?;
	let main_canvas = if let ScalingMode::Pixelated {
		base_size: size, ..
	} = settings.scaling_mode
	{
		Some(Canvas::new(size, CanvasSettings::default()))
	} else {
		None
	};

	let mut last_update_time = Instant::now();

	loop {
		// measure and record delta time
		let now = Instant::now();
		let delta_time = now - last_update_time;
		last_update_time = now;
		Context::with_mut(|ctx| {
			ctx.frame_time_tracker.record(delta_time);
		});

		// poll for events
		let mut events = Context::with_mut(|ctx| ctx.event_pump.poll_iter().collect::<Vec<_>>());

		// create egui UI
		let egui_input = egui_raw_input(&events, delta_time);
		egui_ctx.begin_frame(egui_input);
		state.ui(&egui_ctx)?;
		let egui_output = egui_ctx.end_frame();

		// dispatch events to state
		for event in events
			.drain(..)
			.filter(|event| !egui_took_sdl2_event(&egui_ctx, event))
			.filter_map(Event::from_sdl2_event)
		{
			match event {
				Event::WindowSizeChanged(size) => {
					Context::with_mut(|ctx| ctx.graphics.resize(size))
				}
				Event::Exited => Context::with_mut(|ctx| ctx.should_quit = true),
				_ => {}
			}
			let transform = Context::with(|ctx| ctx.scaling_mode.transform_affine2().inverse());
			state.event(event.transform_mouse_events(transform))?;
		}
		Context::with_mut(|ctx| {
			ctx.egui_wants_keyboard_input = egui_ctx.wants_keyboard_input();
			ctx.egui_wants_mouse_input = egui_ctx.wants_pointer_input();
		});

		// update state
		state.update(delta_time)?;

		// draw state and egui UI
		if let Some(main_canvas) = &main_canvas {
			clear(LinSrgba::BLACK);
			{
				let _scope = main_canvas.render_to();
				state.draw()?;
			}
			let transform = Context::with(|ctx| ctx.scaling_mode.transform_mat4());
			main_canvas.draw().transformed(transform);
		} else {
			let _scope = push_transform(Context::with(|ctx| ctx.scaling_mode.transform_mat4()));
			state.draw()?;
		}
		draw_egui_output(&egui_ctx, egui_output, &mut egui_textures);
		Context::with(|ctx| ctx.window.gl_swap_window());

		Context::with_mut(|ctx| ctx.graphics.delete_unused_resources());

		if Context::with(|ctx| ctx.should_quit) {
			break;
		}
	}
	Ok(())
}

/// The main interface between your game code and functionality provided
/// by the framework.
pub(crate) struct Context {
	_sdl: Sdl,
	pub(crate) video: VideoSubsystem,
	pub(crate) window: Window,
	pub(crate) controller: GameControllerSubsystem,
	pub(crate) event_pump: EventPump,
	pub(crate) egui_wants_keyboard_input: bool,
	pub(crate) egui_wants_mouse_input: bool,
	pub(crate) graphics: GraphicsContext,
	pub(crate) scaling_mode: ScalingMode,
	pub(crate) frame_time_tracker: FrameTimeTracker,
	pub(crate) should_quit: bool,
}

impl Context {
	pub(crate) fn with<T>(mut f: impl FnMut(&Self) -> T) -> T {
		CONTEXT.with(|ctx| {
			let ctx = &ctx
				.get()
				.unwrap_or_else(|| panic!("context not initialized"))
				.borrow();
			f(ctx)
		})
	}

	pub(crate) fn with_mut<T>(mut f: impl FnMut(&mut Self) -> T) -> T {
		CONTEXT.with(|ctx| {
			let ctx = &mut ctx
				.get()
				.unwrap_or_else(|| panic!("context not initialized"))
				.borrow_mut();
			f(ctx)
		})
	}

	fn new(settings: &ContextSettings) -> Self {
		let sdl = sdl2::init().expect("error initializing SDL");
		let video = sdl.video().expect("error initializing video subsystem");
		let controller = sdl
			.game_controller()
			.expect("error initializing controller subsystem");
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(GLProfile::Core);
		gl_attr.set_context_version(3, 3);
		gl_attr.set_stencil_size(8);
		gl_attr.set_framebuffer_srgb_compatible(true);
		let window = build_window(&video, settings);
		let _sdl_gl_ctx = window
			.gl_create_context()
			.expect("error creating OpenGL context");
		video
			.gl_set_swap_interval(settings.swap_interval)
			.expect("error setting swap interval");
		let event_pump = sdl.event_pump().expect("error creating event pump");
		let graphics = GraphicsContext::new(&video, &window);
		Self {
			_sdl: sdl,
			video,
			window,
			controller,
			event_pump,
			egui_wants_keyboard_input: false,
			egui_wants_mouse_input: false,
			graphics,
			scaling_mode: settings.scaling_mode,
			frame_time_tracker: FrameTimeTracker::new(),
			should_quit: false,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	pub window_title: String,
	pub window_mode: WindowMode,
	pub resizable: bool,
	pub swap_interval: SwapInterval,
	pub scaling_mode: ScalingMode,
	pub qualifier: &'static str,
	pub organization_name: &'static str,
	pub app_name: &'static str,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_mode: WindowMode::default(),
			resizable: false,
			swap_interval: SwapInterval::VSync,
			scaling_mode: ScalingMode::default(),
			qualifier: "com",
			organization_name: "Tesselode",
			app_name: "Game",
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ScalingMode {
	#[default]
	None,
	Smooth {
		base_size: UVec2,
	},
	Pixelated {
		base_size: UVec2,
		integer_scale: bool,
	},
}

impl ScalingMode {
	pub(crate) fn transform_affine2(&self) -> Affine2 {
		match self {
			ScalingMode::None => Affine2::IDENTITY,
			ScalingMode::Smooth { base_size } => {
				let max_horizontal_scale = window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = window_size().y as f32 / base_size.y as f32;
				let scale = max_horizontal_scale.min(max_vertical_scale);
				Affine2::from_translation(window_size().as_vec2() / 2.0)
					* Affine2::from_scale(Vec2::splat(scale))
					* Affine2::from_translation(-base_size.as_vec2() / 2.0)
			}
			ScalingMode::Pixelated {
				base_size,
				integer_scale,
			} => {
				let max_horizontal_scale = window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = window_size().y as f32 / base_size.y as f32;
				let mut scale = max_horizontal_scale.min(max_vertical_scale);
				if *integer_scale {
					scale = scale.floor();
				}
				Affine2::from_translation((window_size().as_vec2() / 2.0).round())
					* Affine2::from_scale(Vec2::splat(scale))
					* Affine2::from_translation((-base_size.as_vec2() / 2.0).round())
			}
		}
	}

	fn transform_mat4(&self) -> Mat4 {
		match self {
			ScalingMode::None => Mat4::IDENTITY,
			ScalingMode::Smooth { base_size } => {
				let max_horizontal_scale = window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = window_size().y as f32 / base_size.y as f32;
				let scale = max_horizontal_scale.min(max_vertical_scale);
				Mat4::from_translation((window_size().as_vec2() / 2.0).extend(0.0))
					* Mat4::from_scale(Vec2::splat(scale).extend(1.0))
					* Mat4::from_translation((-base_size.as_vec2() / 2.0).extend(0.0))
			}
			ScalingMode::Pixelated {
				base_size,
				integer_scale,
			} => {
				let max_horizontal_scale = window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = window_size().y as f32 / base_size.y as f32;
				let mut scale = max_horizontal_scale.min(max_vertical_scale);
				if *integer_scale {
					scale = scale.floor();
				}
				Mat4::from_translation((window_size().as_vec2() / 2.0).extend(0.0))
					* Mat4::from_scale(Vec2::splat(scale).extend(1.0))
					* Mat4::from_translation((-base_size.as_vec2() / 2.0).extend(0.0))
			}
		}
	}
}

#[must_use]
pub struct OnDrop {
	pub(crate) on_drop: fn(),
}

impl Drop for OnDrop {
	fn drop(&mut self) {
		(self.on_drop)();
	}
}
