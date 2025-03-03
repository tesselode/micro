use wgpu::StencilFaceState;
pub use wgpu::{CompareFunction, StencilOperation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StencilState {
	pub compare: CompareFunction,
	pub on_fail: StencilOperation,
	pub on_depth_fail: StencilOperation,
	pub on_pass: StencilOperation,
	pub read_mask: u8,
	pub write_mask: u8,
}

impl StencilState {
	pub(crate) fn as_wgpu_stencil_state(self) -> wgpu::StencilState {
		wgpu::StencilState {
			front: StencilFaceState {
				compare: self.compare,
				fail_op: self.on_fail,
				depth_fail_op: self.on_depth_fail,
				pass_op: self.on_pass,
			},
			back: StencilFaceState {
				compare: self.compare,
				fail_op: self.on_fail,
				depth_fail_op: self.on_depth_fail,
				pass_op: self.on_pass,
			},
			read_mask: self.read_mask.into(),
			write_mask: self.write_mask.into(),
		}
	}
}

impl Default for StencilState {
	fn default() -> Self {
		Self {
			compare: CompareFunction::Never,
			on_fail: StencilOperation::Keep,
			on_depth_fail: StencilOperation::Keep,
			on_pass: StencilOperation::Keep,
			read_mask: 0,
			write_mask: 0,
		}
	}
}
