use std::{error::Error, f32::consts::FRAC_PI_4, time::Duration};

use glam::{Mat4, UVec2, Vec2, Vec3};
use micro::{
	graphics::{
		mesh::{Mesh, MeshBuilder, ShapeStyle},
		Camera3d, ColorConstants,
	},
	math::Circle,
	Context, ContextSettings, ScalingMode, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(
		ContextSettings {
			resizable: true,
			scaling_mode: ScalingMode::Pixelated {
				base_size: UVec2::new(800, 600),
				integer_scale: true,
			},
			..Default::default()
		},
		MainState::new,
	)
}

struct MainState {
	mesh: Mesh,
	mesh_position: Vec3,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		let cube = MeshBuilder::from_obj_file("resources/wireframe_cube.obj")?;
		Ok(Self {
			mesh: MeshBuilder::new()
				.with_circle(
					ShapeStyle::Stroke(0.1),
					Circle {
						center: Vec2::ZERO,
						radius: 4.0,
					},
					LinSrgba::RED,
				)?
				.transformed(Mat4::from_rotation_x(1.0))
				.appended_with(cube.clone().transformed(Mat4::from_rotation_z(FRAC_PI_4)))
				.appended_with(cube.transformed(Mat4::from_scale(Vec3::splat(2.0))))
				.build(ctx),
			mesh_position: Vec3::new(0.0, 0.0, 2.0),
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, _ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		self.mesh_position.z += delta_time.as_secs_f32();
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
				self.mesh.draw(ctx, self.mesh_position);
			},
		);
		Ok(())
	}
}
