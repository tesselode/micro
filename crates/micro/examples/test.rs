use std::{error::Error, f32::consts::FRAC_PI_4, time::Duration};

use glam::{Vec2, Vec3};
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		Camera3d, ColorConstants,
	},
	math::{Circle, Rect},
	Context, ContextSettings, State,
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
	mesh2: Mesh,
	mesh_position: Vec3,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		tracing::error!("test error");
		Ok(Self {
			mesh: Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: Vec2::ZERO,
					radius: 0.5,
				},
				LinSrgba::WHITE,
			)?,
			mesh2: Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: Vec2::splat(50.0),
					radius: 10.0,
				},
				LinSrgba::WHITE,
			)?,
			mesh_position: Vec3::ZERO,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, _ctx: &mut Context, delta_time: Duration) -> Result<(), Box<dyn Error>> {
		self.mesh_position.z += delta_time.as_secs_f32();
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::BLACK);
		ctx.with_replacement_transform(
			Camera3d::orthographic(
				Rect::from_xywh(-1.0, -1.0, 2.0, 2.0),
				0.0..=100.0,
				Vec3::ZERO,
				Vec3::new(0.0, 0.0, 1.0),
			)
			.transform(),
			|ctx| {
				self.mesh.draw(ctx, self.mesh_position);
			},
		);
		self.mesh2.draw(ctx, LinSrgba::RED);
		Ok(())
	}
}
