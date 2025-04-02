use crate::Context;

use super::{RawGraphicsPipeline, Vertex};

pub trait Drawable {
	type Vertex: Vertex;

	#[allow(private_interfaces)]
	fn draw(&self, ctx: &mut Context, graphics_pipeline: RawGraphicsPipeline);
}
