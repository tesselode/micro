use glam::Vec2;
use micro::{
	graphics::{
		color::Rgba,
		mesh::{FilledPolygonPoint, Mesh, MeshBuilder, StrokePoint},
		DrawParams,
	},
	Context, ContextSettings, State,
};

struct MainState {
	mesh: Mesh,
}

impl MainState {
	fn new(ctx: &mut Context) -> Self {
		Self {
			mesh: {
				let mut mesh_builder = MeshBuilder::new();
				mesh_builder.add_polyline(
					[
						StrokePoint {
							position: Vec2::new(50.0, 50.0),
							color: Rgba::GREEN,
							stroke_width: 1.0,
						},
						StrokePoint {
							position: Vec2::new(100.0, 150.0),
							color: Rgba::RED,
							stroke_width: 1.0,
						},
						StrokePoint {
							position: Vec2::new(300.0, 150.0),
							color: Rgba::BLUE,
							stroke_width: 1.0,
						},
					],
					false,
				);
				mesh_builder.build(ctx)
			},
		}
	}
}

impl State for MainState {
	fn draw(&mut self, ctx: &mut Context) {
		self.mesh.draw(ctx, DrawParams::new());
	}
}

fn main() {
	micro::run(ContextSettings::default(), MainState::new)
}
