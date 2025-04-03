use crate::{Context, context::graphics::QueueDrawCommandSettings};

use super::Vertex;

pub trait Drawable {
	type Vertex: Vertex;

	#[allow(private_interfaces)]
	fn draw_instructions(&self, ctx: &mut Context) -> Vec<QueueDrawCommandSettings>;
}
