use glow::HasContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Culling {
	#[default]
	None,
	Front,
	Back,
}

impl Culling {
	pub(crate) fn apply(&self, gl: &glow::Context) {
		unsafe {
			match self {
				Culling::None => {
					gl.disable(glow::CULL_FACE);
				}
				Culling::Front => {
					gl.enable(glow::CULL_FACE);
					gl.cull_face(glow::FRONT);
				}
				Culling::Back => {
					gl.enable(glow::CULL_FACE);
					gl.cull_face(glow::BACK);
				}
			}
		}
	}
}
