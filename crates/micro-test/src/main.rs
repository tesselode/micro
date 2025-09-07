use std::{f32::consts::FRAC_PI_2, path::Path};

use bytemuck::{Pod, Zeroable};
use micro2::{
	App, ContextSettings, Event,
	color::{ColorConstants, LinSrgba},
	egui::Window,
	graphics::{
		Camera3d, HasVertexAttributes, Shader, Vertex, VertexAttribute, mesh::Mesh,
		vertex_attr_array,
	},
	input::Scancode,
	math::{Vec3, vec3},
	push, quit, window_size,
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
	fn new() -> Self {
		Self {
			mesh: load_3d_mesh("resources/cube.obj"),
			shader: Shader::from_string("3d shader", CUSTOM_SHADER_SOURCE),
			camera: Camera3d::perspective(
				FRAC_PI_2,
				window_size().x as f32 / window_size().y as f32,
				0.1..=1000.0,
				Vec3::ZERO,
				vec3(0.0, 0.0, 10.0),
			),
		}
	}
}

impl App for Test {
	fn debug_ui(&mut self, egui_ctx: &micro2::egui::Context) {
		Window::new("test").show(egui_ctx, |ui| {
			ui.label("Hello, world!");
		});
	}

	fn event(&mut self, event: Event) {
		if let Event::KeyPressed {
			key: Scancode::Escape,
			..
		} = event
		{
			quit();
		}
	}

	fn draw(&mut self) {
		push! {
			transform: self.camera.transform(),
			shader: self.shader.clone(),
			enable_depth_testing: true,
		};
		self.mesh.rotated_y(0.5).translated_z(3.0).draw();
		self.mesh
			.rotated_y(0.5)
			.translated_z(5.0)
			.color(LinSrgba::BLACK)
			.draw();
	}
}

fn load_3d_mesh(path: impl AsRef<Path>) -> Mesh<Vertex3d> {
	let (tobj_models, _) = tobj::load_obj(path.as_ref(), &GPU_LOAD_OPTIONS).unwrap();
	let tobj_model = &tobj_models[0];
	Mesh::new(
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
