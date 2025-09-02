use bytemuck::{Pod, Zeroable};
use glam::{Vec2, vec2};
use micro2::{
	App, Context, ContextSettings, Event,
	graphics::{
		HasVertexAttributes, Shader, Vertex,
		mesh::{Mesh, ShapeStyle},
	},
	input::Scancode,
	math::{Circle, Rect},
};
use wgpu::vertex_attr_array;

const CUSTOM_SHADER_SOURCE: &str = include_str!("shader.glsl");

fn main() {
	micro2::run(ContextSettings::default(), Test::new);
}

struct Test {
	mesh: Mesh,
	weird_mesh: Mesh<CustomVertex>,
	shader: Shader,
}

impl Test {
	fn new(ctx: &mut Context) -> Self {
		Self {
			mesh: Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: vec2(50.0, 50.0),
					radius: 50.0,
				},
			)
			.unwrap(),
			weird_mesh: rectangle(ctx, Rect::new((50.0, 50.0), (100.0, 150.0))),
			shader: Shader::from_string("custom shader", CUSTOM_SHADER_SOURCE),
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
		self.mesh.draw(ctx);
		self.weird_mesh.shader(&self.shader).draw(ctx);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
struct CustomVertex {
	position: Vec2,
}

impl HasVertexAttributes for CustomVertex {
	fn attributes() -> Vec<wgpu::VertexAttribute> {
		vertex_attr_array![0 => Float32x2].into()
	}
}

impl Vertex for CustomVertex {}

fn rectangle(ctx: &Context, rect: Rect) -> Mesh<CustomVertex> {
	let _span = tracy_client::span!();
	rectangle_with_texture_region(ctx, rect, Rect::new((0.0, 0.0), (1.0, 1.0)))
}

fn rectangle_with_texture_region(
	ctx: &Context,
	display_rect: Rect,
	texture_region: Rect,
) -> Mesh<CustomVertex> {
	let _span = tracy_client::span!();
	let vertices = display_rect
		.corners()
		.iter()
		.copied()
		.zip(texture_region.corners())
		.map(|(position, _)| CustomVertex { position })
		.collect::<Vec<_>>();
	Mesh::new(ctx, &vertices, &[0, 1, 3, 1, 2, 3])
}
