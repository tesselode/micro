use std::error::Error;

use glam::{Mat4, UVec2, Vec2, Vec3};
use micro::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		shader::Shader,
		ColorConstants, DrawParams,
	},
	math::Circle,
	Context, ContextSettings, ScalingMode, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(
		ContextSettings {
			scaling_mode: ScalingMode::Pixelated {
				base_size: UVec2::new(400, 300),
				integer_scale: true,
			},
			..Default::default()
		},
		MainState::new,
	);
}

struct MainState {
	shader: Shader,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(MainState {
			shader: Shader::from_combined_file(ctx, "crates/micro/examples/test.glsl")?,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		{
			let ctx = &mut ctx.transform(Mat4::from_scale(Vec3::splat(3.0)));
			Mesh::circle(
				ctx,
				ShapeStyle::Fill,
				Circle {
					center: Vec2::ZERO,
					radius: 100.0,
				},
				LinSrgba::RED,
			)?
			.draw(ctx, DrawParams::new());
		}
		Mesh::circle(
			ctx,
			ShapeStyle::Fill,
			Circle {
				center: Vec2::ZERO,
				radius: 100.0,
			},
			LinSrgba::WHITE,
		)?
		.draw(ctx, &self.shader);
		Ok(())
	}
}
