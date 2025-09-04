use std::{f32::consts::FRAC_PI_2, path::Path};

use bytemuck::{Pod, Zeroable};
use micro2::{
	App, Context, ContextSettings, Event,
	color::{ColorConstants, LinSrgba},
	graphics::{
		Camera3d, DepthStencilState, HasVertexAttributes, Shader, Vertex, VertexAttribute,
		mesh::Mesh, vertex_attr_array,
	},
	input::Scancode,
	math::{Vec3, vec3},
};
use tobj::GPU_LOAD_OPTIONS;

const CUSTOM_SHADER_SOURCE: &str = include_str!("shader.glsl");

fn main() {
	micro2::run(ContextSettings::default(), Test::new);
}

struct Test {
	mesh: Mesh<Vertex3d>,
	shader: Shader,
	camera: Camera3d,
}

impl Test {
	fn new(ctx: &mut Context) -> Self {
		Self {
			mesh: load_3d_mesh(ctx, "resources/cube.obj"),
			shader: Shader::from_string("3d shader", CUSTOM_SHADER_SOURCE),
			camera: Camera3d::perspective(
				FRAC_PI_2,
				ctx.window_size().x as f32 / ctx.window_size().y as f32,
				0.1..=1000.0,
				Vec3::ZERO,
				vec3(0.0, 0.0, 10.0),
			),
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
		let ctx = &mut ctx.push_transform(self.camera.transform(ctx));
		let ctx = &mut ctx.push_depth_stencil_state(DepthStencilState::depth());
		self.mesh
			.shader(&self.shader)
			.rotated_y(0.5)
			.translated_z(3.0)
			.draw(ctx);
		self.mesh
			.shader(&self.shader)
			.rotated_y(0.5)
			.translated_z(5.0)
			.color(LinSrgba::BLACK)
			.draw(ctx);
	}
}

fn load_3d_mesh(ctx: &Context, path: impl AsRef<Path>) -> Mesh<Vertex3d> {
	let (tobj_models, _) = tobj::load_obj(path.as_ref(), &GPU_LOAD_OPTIONS).unwrap();
	let tobj_model = &tobj_models[0];
	Mesh::new(
		ctx,
		&tobj_model
			.mesh
			.positions
			.chunks(3)
			.map(|position| Vertex3d {
				position: vec3(position[0], position[1], position[2]),
			})
			.collect::<Vec<_>>(),
		&tobj_model.mesh.indices,
	)
}

#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
struct Vertex3d {
	position: Vec3,
}

impl Vertex for Vertex3d {}

impl HasVertexAttributes for Vertex3d {
	fn attributes() -> Vec<VertexAttribute> {
		vertex_attr_array![0 => Float32x3].into()
	}
}
