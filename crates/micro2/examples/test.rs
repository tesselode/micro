use bytemuck::{Pod, Zeroable};
use glam::{Vec2, vec2};
use micro2::{
	App, Context, ContextSettings, Event,
	color::ColorConstants,
	graphics::{Shader, Vertex2d, mesh::Mesh},
	input::Scancode,
};
use palette::LinSrgba;

const VERTICES: &[Vertex2d] = &[
	Vertex2d {
		position: vec2(-0.0868241, 0.49240386),
		texture_coords: vec2(0.0, 0.0),
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(-0.49513406, 0.06958647),
		texture_coords: vec2(0.0, 0.0),
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(-0.21918549, -0.44939706),
		texture_coords: vec2(0.0, 0.0),
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(0.35966998, -0.3473291),
		texture_coords: vec2(0.0, 0.0),
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
	Vertex2d {
		position: vec2(0.44147372, 0.2347359),
		texture_coords: vec2(0.0, 0.0),
		color: LinSrgba::new(0.5, 0.0, 0.5, 1.0),
	},
];

const INDICES: &[u32] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

fn main() {
	micro2::run(ContextSettings::default(), Test::new);
}

struct Test {
	mesh: Mesh,
}

impl Test {
	fn new(ctx: &mut Context) -> Self {
		Self {
			mesh: Mesh::new(ctx, VERTICES, INDICES),
		}
	}
}

impl App for Test {
	fn event(&mut self, ctx: &mut Context, event: Event) {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			ctx.quit();
		}
	}

	fn draw(&mut self, ctx: &mut Context) {
		self.mesh
			.color(LinSrgba::RED)
			.translated_2d((1.0, 1.0))
			.draw(ctx);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
struct ShaderParams {
	translation: Vec2,
}
