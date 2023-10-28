use std::error::Error;

use glam::{UVec2, Vec2};
use micro::{
	graphics::{
		texture::{Texture, TextureSettings},
		ColorConstants, DrawParams, NineSlice,
	},
	math::Rect,
	Context, ContextSettings, State, WindowMode,
};
use palette::LinSrgba;

pub struct MainState {
	texture: Texture,
	nine_slice: NineSlice,
}

impl MainState {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			texture: Texture::from_file(
				ctx,
				"resources/nine_slice.png",
				TextureSettings::default(),
			)
			.unwrap(),
			nine_slice: NineSlice {
				texture_region: Rect::new(Vec2::ZERO, Vec2::splat(16.0)),
				left: 5.0,
				right: 6.0,
				top: 3.0,
				bottom: 6.0,
			},
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		ctx.clear(LinSrgba::BLACK);
		self.texture.draw_nine_slice(
			ctx,
			self.nine_slice,
			Rect::new(Vec2::ZERO, ctx.mouse_position().as_vec2()),
			DrawParams::default(),
		);
		Ok(())
	}
}

fn main() {
	micro::run(
		ContextSettings {
			window_mode: WindowMode::Windowed {
				size: UVec2::new(500, 500),
			},
			resizable: true,
			..Default::default()
		},
		MainState::new,
	);
}
