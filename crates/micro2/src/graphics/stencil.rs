use wgpu::StencilFaceState;
pub use wgpu::{CompareFunction, StencilOperation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StencilState {
	pub enable_color_writes: bool,
	pub reference: u8,
	pub compare: CompareFunction,
	pub on_fail: StencilOperation,
	pub on_depth_fail: StencilOperation,
	pub on_pass: StencilOperation,
	pub read_mask: u8,
	pub write_mask: u8,
}

impl StencilState {
	pub fn write(operation: StencilOperation, reference: u8) -> Self {
		Self {
			enable_color_writes: false,
			reference,
			compare: CompareFunction::Always,
			on_fail: operation,
			on_depth_fail: operation,
			on_pass: operation,
			read_mask: 255,
			write_mask: 255,
		}
	}

	pub fn read(compare: CompareFunction, reference: u8) -> Self {
		Self {
			enable_color_writes: true,
			reference,
			compare,
			on_fail: StencilOperation::Keep,
			on_depth_fail: StencilOperation::Keep,
			on_pass: StencilOperation::Keep,
			read_mask: 255,
			write_mask: 255,
		}
	}

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
			enable_color_writes: true,
			reference: 0,
			compare: CompareFunction::Always,
			on_fail: StencilOperation::Replace,
			on_depth_fail: StencilOperation::Replace,
			on_pass: StencilOperation::Replace,
			read_mask: 255,
			write_mask: 255,
		}
	}
}
