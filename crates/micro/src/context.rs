pub(crate) mod graphics;

use std::{
	collections::HashMap,
	fmt::Debug,
	time::{Duration, Instant},
};

use backtrace::Backtrace;
use glam::{Affine2, IVec2, UVec2, Vec2};
use glow::HasContext;
use palette::LinSrgba;
use sdl2::{
	video::{FullscreenType, GLProfile, SwapInterval, Window, WindowPos},
	EventPump, GameControllerSubsystem, IntegerOrSdlError, Sdl, VideoSubsystem,
};

use crate::{
	build_window,
	egui_integration::{draw_egui_output, egui_raw_input, egui_took_sdl2_event},
	error::SdlError,
	graphics::{Canvas, CanvasSettings, ColorConstants, StencilAction, StencilTest},
	input::{Gamepad, MouseButton, Scancode},
	log::setup_logging,
	log_if_err,
	time::FrameTimeTracker,
	window::WindowMode,
	Event, State,
};

use self::graphics::GraphicsContext;

pub fn run<S, F, E>(settings: ContextSettings, state_constructor: F)
where
	S: State<E>,
	F: FnMut(&mut Context) -> Result<S, E>,
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
	F: FnMut(&mut Context) -> Result<S, E>,
{
	// create contexts and resources
	let mut ctx = Context::new(&settings);
	let egui_ctx = egui::Context::default();
	let mut egui_textures = HashMap::new();
	let mut state = state_constructor(&mut ctx)?;
	let main_canvas = if let ScalingMode::Pixelated {
		base_size: size, ..
	} = settings.scaling_mode
	{
		Some(Canvas::new(&ctx, size, CanvasSettings::default()))
	} else {
		None
	};

	let mut last_update_time = Instant::now();

	loop {
		// measure and record delta time
		let now = Instant::now();
		let delta_time = now - last_update_time;
		last_update_time = now;
		ctx.frame_time_tracker.record(delta_time);

		// poll for events
		let mut events = ctx.event_pump.poll_iter().collect::<Vec<_>>();

		// create egui UI
		let egui_input = egui_raw_input(&ctx, &events);
		egui_ctx.begin_frame(egui_input);
		state.ui(&mut ctx, &egui_ctx)?;
		let egui_output = egui_ctx.end_frame();

		// dispatch events to state
		for event in events
			.drain(..)
			.filter(|event| !egui_took_sdl2_event(&egui_ctx, event))
			.filter_map(Event::from_sdl2_event)
		{
			match event {
				Event::WindowSizeChanged(size) => ctx.graphics.resize(size),
				Event::Exited => {
					ctx.should_quit = true;
				}
				_ => {}
			}
			let transform = ctx.scaling_mode.transform(&ctx).inverse();
			state.event(&mut ctx, event.transform_mouse_events(transform))?;
		}

		// update state
		state.update(&mut ctx, delta_time)?;

		// draw state and egui UI
		if let Some(main_canvas) = &main_canvas {
			ctx.clear(LinSrgba::BLACK);
			main_canvas.render_to(&mut ctx, |ctx| -> Result<(), E> {
				state.draw(ctx)?;
				Ok(())
			})?;
			let transform = ctx.scaling_mode.transform(&ctx);
			main_canvas.draw(&ctx, transform);
		} else {
			ctx.with_transform(ctx.scaling_mode.transform(&ctx), |ctx| -> Result<(), E> {
				state.draw(ctx)?;
				Ok(())
			})?;
		}
		draw_egui_output(&mut ctx, &egui_ctx, egui_output, &mut egui_textures);
		ctx.window.gl_swap_window();

		if ctx.should_quit {
			break;
		}

		std::thread::sleep(Duration::from_millis(2));
	}
	Ok(())
}

pub struct Context {
	_sdl: Sdl,
	video: VideoSubsystem,
	window: Window,
	controller: GameControllerSubsystem,
	event_pump: EventPump,
	pub(crate) graphics: GraphicsContext,
	scaling_mode: ScalingMode,
	frame_time_tracker: FrameTimeTracker,
	should_quit: bool,
}

impl Context {
	pub fn window_size(&self) -> UVec2 {
		let (width, height) = self.window.size();
		UVec2::new(width, height)
	}

	pub fn window_mode(&self) -> WindowMode {
		match self.window.fullscreen_state() {
			FullscreenType::Off => WindowMode::Windowed {
				size: self.window_size(),
			},
			FullscreenType::True => WindowMode::Fullscreen,
			FullscreenType::Desktop => WindowMode::Fullscreen,
		}
	}

	pub fn swap_interval(&self) -> SwapInterval {
		self.video.gl_get_swap_interval()
	}

	pub fn monitor_resolution(&self) -> Result<UVec2, SdlError> {
		let display_index = self.window.display_index()?;
		let display_mode = self.video.desktop_display_mode(display_index)?;
		Ok(UVec2::new(display_mode.w as u32, display_mode.h as u32))
	}

	pub fn set_window_mode(&mut self, window_mode: WindowMode) -> Result<(), SdlError> {
		match window_mode {
			WindowMode::Fullscreen => {
				self.window.set_fullscreen(FullscreenType::Desktop)?;
			}
			WindowMode::Windowed { size } => {
				self.window.set_fullscreen(FullscreenType::Off)?;
				self.window
					.set_size(size.x, size.y)
					.map_err(|err| match err {
						IntegerOrSdlError::IntegerOverflows(_, _) => panic!("integer overflow"),
						IntegerOrSdlError::SdlError(err) => SdlError(err),
					})?;
				self.window
					.set_position(WindowPos::Centered, WindowPos::Centered);
			}
		}
		Ok(())
	}

	pub fn set_swap_interval(&mut self, swap_interval: SwapInterval) -> Result<(), SdlError> {
		self.video.gl_set_swap_interval(swap_interval)?;
		Ok(())
	}

	pub fn clear(&self, color: LinSrgba) {
		unsafe {
			self.graphics
				.gl
				.clear_color(color.red, color.green, color.blue, color.alpha);
			self.graphics.gl.stencil_mask(0xFF);
			self.graphics.gl.clear_stencil(0);
			self.graphics
				.gl
				.clear(glow::COLOR_BUFFER_BIT | glow::STENCIL_BUFFER_BIT);
			self.graphics.gl.stencil_mask(0x00);
		}
	}

	pub fn clear_stencil(&self) {
		unsafe {
			self.graphics.gl.stencil_mask(0xFF);
			self.graphics.gl.clear_stencil(0);
			self.graphics.gl.clear(glow::STENCIL_BUFFER_BIT);
			self.graphics.gl.stencil_mask(0x00);
		}
	}

	pub fn with_transform<T>(
		&mut self,
		transform: Affine2,
		f: impl FnOnce(&mut Context) -> T,
	) -> T {
		self.graphics.transform_stack.push(transform);
		let returned_value = f(self);
		self.graphics.transform_stack.pop();
		returned_value
	}

	pub fn write_to_stencil<T>(
		&mut self,
		action: StencilAction,
		f: impl FnOnce(&mut Context) -> T,
	) -> T {
		unsafe {
			self.graphics.gl.color_mask(false, false, false, false);
			self.graphics.gl.enable(glow::STENCIL_TEST);
			let op = action.as_glow_stencil_op();
			self.graphics.gl.stencil_op(op, op, op);
			let reference = match action {
				StencilAction::Replace(value) => value,
				_ => 0,
			};
			self.graphics
				.gl
				.stencil_func(glow::ALWAYS, reference.into(), 0xFF);
			self.graphics.gl.stencil_mask(0xFF);
		}
		let returned_value = f(self);
		unsafe {
			self.graphics.gl.color_mask(true, true, true, true);
			self.graphics.gl.disable(glow::STENCIL_TEST);
		}
		returned_value
	}

	pub fn with_stencil<T>(
		&mut self,
		test: StencilTest,
		reference: u8,
		f: impl FnOnce(&mut Context) -> T,
	) -> T {
		unsafe {
			self.graphics.gl.enable(glow::STENCIL_TEST);
			self.graphics
				.gl
				.stencil_op(glow::KEEP, glow::KEEP, glow::KEEP);
			self.graphics
				.gl
				.stencil_func(test.as_glow_stencil_func(), reference.into(), 0xFF);
			self.graphics.gl.stencil_mask(0x00);
		}
		let returned_value = f(self);
		unsafe {
			self.graphics.gl.disable(glow::STENCIL_TEST);
		}
		returned_value
	}

	pub fn is_key_down(&self, scancode: Scancode) -> bool {
		self.event_pump
			.keyboard_state()
			.is_scancode_pressed(scancode.into())
	}

	pub fn is_mouse_button_down(&self, mouse_button: MouseButton) -> bool {
		self.event_pump
			.mouse_state()
			.is_mouse_button_pressed(mouse_button.into())
	}

	pub fn mouse_position(&self) -> IVec2 {
		let mouse_state = self.event_pump.mouse_state();
		let untransformed = IVec2::new(mouse_state.x(), mouse_state.y());
		self.scaling_mode
			.transform(self)
			.inverse()
			.transform_point2(untransformed.as_vec2())
			.as_ivec2()
	}

	pub fn game_controller(&self, index: u32) -> Option<Gamepad> {
		match self.controller.open(index) {
			Ok(controller) => Some(Gamepad(controller)),
			Err(error) => match error {
				IntegerOrSdlError::IntegerOverflows(_, _) => {
					panic!("integer overflow when getting controller")
				}
				IntegerOrSdlError::SdlError(_) => None,
			},
		}
	}

	pub fn average_frame_time(&self) -> Duration {
		self.frame_time_tracker.average()
	}

	pub fn fps(&self) -> f32 {
		1.0 / self.average_frame_time().as_secs_f32()
	}

	pub fn quit(&mut self) {
		self.should_quit = true;
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
	fn transform(&self, ctx: &Context) -> Affine2 {
		match self {
			ScalingMode::None => Affine2::IDENTITY,
			ScalingMode::Smooth { base_size } => {
				let max_horizontal_scale = ctx.window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = ctx.window_size().y as f32 / base_size.y as f32;
				let scale = max_horizontal_scale.min(max_vertical_scale);
				Affine2::from_translation(ctx.window_size().as_vec2() / 2.0)
					* Affine2::from_scale(Vec2::splat(scale))
					* Affine2::from_translation(-base_size.as_vec2() / 2.0)
			}
			ScalingMode::Pixelated {
				base_size,
				integer_scale,
			} => {
				let max_horizontal_scale = ctx.window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = ctx.window_size().y as f32 / base_size.y as f32;
				let mut scale = max_horizontal_scale.min(max_vertical_scale);
				if *integer_scale {
					scale = scale.floor();
				}
				Affine2::from_translation((ctx.window_size().as_vec2() / 2.0).round())
					* Affine2::from_scale(Vec2::splat(scale))
					* Affine2::from_translation((-base_size.as_vec2() / 2.0).round())
			}
		}
	}
}
