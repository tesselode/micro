use std::error::Error;

use glam::{Mat4, Vec2, Vec3};
use lyon::{
	lyon_tessellation::{
		BuffersBuilder, StrokeGeometryBuilder, StrokeOptions, StrokeTessellator, StrokeVertex,
		VertexBuffers,
	},
	path::{
		traits::{Build, PathBuilder},
		Path,
	},
};
use micro::{
	blend_mode::{BlendAlphaMode, BlendMode},
	color::Rgba,
	context::Context,
	draw_params::DrawParams,
	mesh::{Mesh, Vertex},
	texture::Texture,
	Game, State,
};

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			mesh: {
				let mut path_builder = Path::builder();
				path_builder.begin(lyon::math::point(100.0, 100.0));
				path_builder.line_to(lyon::math::point(100.0, 200.0));
				path_builder.line_to(lyon::math::point(150.0, 150.0));
				path_builder.line_to(lyon::math::point(50.0, 150.0));
				path_builder.close();
				let path = path_builder.build();
				Mesh::from_path_stroke(ctx, path, &StrokeOptions::default().with_line_width(5.0))?
			},
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(Rgba::new(0.1, 0.2, 0.3, 1.0));
		self.mesh.draw(
			ctx,
			DrawParams::new()
				.color(Rgba {
					red: 1.0,
					green: 1.0,
					blue: 1.0,
					alpha: 0.5,
				})
				.blend_mode(BlendMode::Add(BlendAlphaMode::AlphaMultiply)),
		);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	Game::init()?.run(MainState::new)?;
	Ok(())
}
