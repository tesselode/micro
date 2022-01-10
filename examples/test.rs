use std::{error::Error, time::Duration};

use glow::HasContext;

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

fn main() -> Result<(), Box<dyn Error>> {
	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;
	let gl_attr = video_subsystem.gl_attr();
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_version(3, 3);
	let window = video_subsystem
		.window("Window", 800, 600)
		.opengl()
		.build()?;
	let _ctx = window.gl_create_context()?;
	let gl_ctx = unsafe {
		glow::Context::from_loader_function(|name| {
			video_subsystem.gl_get_proc_address(name) as *const _
		})
	};
	let mut event_pump = sdl_context.event_pump()?;
	let shader_program;
	let vertex_array;
	unsafe {
		// set up shaders
		let vertex_shader = gl_ctx.create_shader(glow::VERTEX_SHADER)?;
		gl_ctx.shader_source(vertex_shader, VERTEX_SHADER);
		gl_ctx.compile_shader(vertex_shader);
		let fragment_shader = gl_ctx.create_shader(glow::FRAGMENT_SHADER)?;
		gl_ctx.shader_source(fragment_shader, FRAGMENT_SHADER);
		gl_ctx.compile_shader(fragment_shader);
		shader_program = gl_ctx.create_program()?;
		gl_ctx.attach_shader(shader_program, vertex_shader);
		gl_ctx.attach_shader(shader_program, fragment_shader);
		gl_ctx.link_program(shader_program);
		gl_ctx.delete_shader(vertex_shader);
		gl_ctx.delete_shader(fragment_shader);

		// set up vertices and vertex attributes
		vertex_array = gl_ctx.create_vertex_array()?;
		let buffer = gl_ctx.create_buffer()?;
		gl_ctx.bind_vertex_array(Some(vertex_array));
		gl_ctx.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
		let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
		gl_ctx.buffer_data_u8_slice(
			glow::ARRAY_BUFFER,
			bytemuck::bytes_of(&vertices),
			glow::STATIC_DRAW,
		);
		gl_ctx.vertex_attrib_pointer_f32(
			0,
			3,
			glow::FLOAT,
			false,
			(3 * std::mem::size_of::<f32>()).try_into().unwrap(),
			0,
		);
		gl_ctx.enable_vertex_attrib_array(0);
		gl_ctx.bind_buffer(glow::ARRAY_BUFFER, None);
		gl_ctx.bind_vertex_array(None);
	}
	'running: loop {
		unsafe {
			gl_ctx.clear_color(0.2, 0.3, 0.3, 1.0);
			gl_ctx.clear(glow::COLOR_BUFFER_BIT);
			gl_ctx.use_program(Some(shader_program));
			gl_ctx.bind_vertex_array(Some(vertex_array));
			gl_ctx.draw_arrays(glow::TRIANGLES, 0, 3);
		}
		window.gl_swap_window();
		for event in event_pump.poll_iter() {
			if let sdl2::event::Event::Quit { .. } = event {
				break 'running;
			}
		}
		std::thread::sleep(Duration::from_millis(2));
	}
	Ok(())
}
