use wgpu::StencilFaceState;
pub use wgpu::{CompareFunction, StencilOperation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StencilState {
	pub compare: CompareFunction,
	pub fail_op: StencilOperation,
	pub pass_op: StencilOperation,
	pub read_mask: u32,
	pub write_mask: u32,
}

impl StencilState {
	pub(crate) fn to_wgpu_stencil_state(self) -> wgpu::StencilState {
		let stencil_face_state = StencilFaceState {
			compare: self.compare,
			fail_op: self.fail_op,
			depth_fail_op: self.fail_op,
			pass_op: self.pass_op,
		};
		wgpu::StencilState {
			front: stencil_face_state,
			back: stencil_face_state,
			read_mask: self.read_mask,
			write_mask: self.write_mask,
		}
	}
}

impl Default for StencilState {
	fn default() -> Self {
		Self {
			compare: CompareFunction::Always,
			fail_op: StencilOperation::Keep,
			pass_op: StencilOperation::Keep,
			read_mask: 0,
			write_mask: 0,
		}
	}
}
