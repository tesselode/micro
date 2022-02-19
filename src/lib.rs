use std::time::Duration;

use error::RunError;
use glow::HasContext;
use sdl2::{event::Event, video::GLProfile};

pub mod error;

pub fn run() -> Result<(), RunError> {
	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;
	let gl_attr = video_subsystem.gl_attr();
	gl_attr.set_context_profile(GLProfile::Core);
	gl_attr.set_context_version(3, 3);
	let window = video_subsystem.window("Test", 800, 600).opengl().build()?;
	let ctx = window.gl_create_context()?;
	let gl_ctx = unsafe {
		glow::Context::from_loader_function(|name| {
			video_subsystem.gl_get_proc_address(name) as *const _
		})
	};
	let mut event_pump = sdl_context.event_pump()?;
	'running: loop {
		unsafe {
			gl_ctx.clear_color(0.6, 0.0, 0.8, 1.0);
			gl_ctx.clear(glow::COLOR_BUFFER_BIT);
		}
		window.gl_swap_window();
		for event in event_pump.poll_iter() {
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
