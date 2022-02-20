pub mod color;
pub mod context;
pub mod error;
pub mod image_data;
pub mod mesh;
pub mod shader;
pub mod texture;

use std::time::Duration;

use context::Context;
use error::InitError;
use sdl2::{
	event::Event,
	video::{GLContext, GLProfile, Window},
	EventPump, Sdl, VideoSubsystem,
};

#[allow(unused_variables)]
pub trait State<E> {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), E> {
		Ok(())
	}
}

pub struct Game {
	_sdl: Sdl,
	_video: VideoSubsystem,
	window: Window,
	_gl_ctx: GLContext,
	ctx: Context,
	event_pump: EventPump,
}

impl Game {
	pub fn init() -> Result<Self, InitError> {
		let sdl = sdl2::init()?;
		let video = sdl.video()?;
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(GLProfile::Core);
		gl_attr.set_context_version(3, 3);
		let window = video.window("Test", 800, 600).opengl().build()?;
		let _gl_ctx = window.gl_create_context()?;
		let ctx = Context::new(&video);
		let event_pump = sdl.event_pump()?;
		Ok(Self {
			_sdl: sdl,
			_video: video,
			window,
			_gl_ctx,
			ctx,
			event_pump,
		})
	}

	pub fn run<E, S, F>(&mut self, mut state_constructor: F) -> Result<(), E>
	where
		S: State<E>,
		F: FnMut(&mut Context) -> Result<S, E>,
	{
		let mut state = state_constructor(&mut self.ctx)?;
		'running: loop {
			state.draw(&mut self.ctx)?;
			self.window.gl_swap_window();
			for event in self.event_pump.poll_iter() {
				match event {
					Event::Quit { .. } => {
						break 'running;
					}
					_ => {}
				}
			}
			std::thread::sleep(Duration::from_millis(2));
		}
		Ok(())
	}
}
