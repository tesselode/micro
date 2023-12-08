use std::{error::Error, f32::consts::FRAC_PI_4, time::Duration};

use glam::{Mat4, UVec2, Vec2, Vec3};
use micro::{
	graphics::{
		mesh::{Mesh, MeshBuilder, ShapeStyle},
		shader::Shader,
		Camera3d, ColorConstants, DrawInstancedSettings, DrawParams, InstanceParams,
	},
	math::Circle,
	Context, ContextSettings, ScalingMode, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(
		ContextSettings {
			resizable: true,
			..Default::default()
		},
		MainState::new,
	)
}

struct MainState {
	mesh: Mesh,
	shader: Shader,
	rotation_x: f32,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: Mesh::from_obj_file(ctx, "resources/cube.obj")?,
			shader: {
				let shader = Shader::from_fragment_file(ctx, "resources/fragment.glsl")?;
				shader
					.send_vec3("lightPosition", Vec3::new(0.0, 0.0, 1.0))
					.unwrap();
				shader
			},
			rotation_x: 0.0,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		self.rotation_x += delta_time.as_secs_f32();
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.set_depth_buffer_enabled(true);
		ctx.clear(LinSrgba::BLACK);
		ctx.with_transform(
			Camera3d::perspective(
				FRAC_PI_4,
				ctx.window_size().x as f32 / ctx.window_size().y as f32,
				0.1..=100.0,
				Vec3::ZERO,
				Vec3::new(0.0, 0.0, 1.0),
			)
			.transform(ctx),
			|ctx| {
				self.mesh.draw_instanced(
					ctx,
					DrawInstancedSettings::new([
						InstanceParams::new()
							.rotated_x(self.rotation_x)
							.rotated_y(self.rotation_x * 0.7)
							.translated_3d(Vec3::new(-3.0, 0.0, 10.0))
							.color(LinSrgba::RED),
						InstanceParams::new()
							.rotated_x(self.rotation_x)
							.rotated_y(self.rotation_x * 0.7)
							.translated_3d(Vec3::new(0.0, 0.0, 10.0))
							.color(LinSrgba::GREEN),
						InstanceParams::new()
							.rotated_x(self.rotation_x)
							.rotated_y(self.rotation_x * 0.7)
							.translated_3d(Vec3::new(3.0, 0.0, 10.0))
							.color(LinSrgba::BLUE),
					])
					.shader(&self.shader),
				);
			},
		);
		Ok(())
	}
}
