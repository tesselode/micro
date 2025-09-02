use bytemuck::{Pod, Zeroable};
use glam::{Vec2, vec2};
use micro2::{
	App, Context, ContextSettings, Event,
	color::ColorConstants,
	graphics::{
		BlendAlphaMode, BlendMode, Shader, Vertex2d,
		mesh::Mesh,
		texture::{Texture, TextureSettings},
	},
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
	texture: Texture,
}

impl Test {
	fn new(ctx: &mut Context) -> Self {
		Self {
			mesh: Mesh::new(ctx, VERTICES, INDICES),
			texture: Texture::from_file(ctx, "resources/water.png", TextureSettings::default())
				.unwrap(),
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
		self.texture.draw(ctx);
		self.texture
			.translated_2d((50.0, 50.0))
			.blend_mode(BlendMode::Subtract(BlendAlphaMode::AlphaMultiply))
			.color(LinSrgba::new(1.0, 1.0, 1.0, 0.5))
			.draw(ctx);
	}
}
