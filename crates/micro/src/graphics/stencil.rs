use wgpu::StencilFaceState;
pub use wgpu::{CompareFunction, StencilOperation};

/// How drawing operations interact with the stencil buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StencilState {
	/// Whether drawing operations should affect the color pixels.
	pub enable_color_writes: bool,
	/// The reference value used for some [`CompareFunction`]s and
	/// [`StencilOperation`]s.
	pub reference: u8,
	/// Determines whether the stencil test passes or fails.
	pub compare: CompareFunction,
	/// How the drawing operation affects the stencil buffer if the stencil
	/// test fails.
	pub on_fail: StencilOperation,
	/// How the drawing operation affects the stencil buffer if the stencil
	/// test passes, but the depth test fails.
	pub on_depth_fail: StencilOperation,
	/// How the drawing operation affects the stencil buffer if the stencil
	/// and depth tests pass.
	pub on_pass: StencilOperation,
	/// A bitmask applied when reading from the stencil buffer.
	pub read_mask: u8,
	/// A bitmask applied when writing to the stencil buffer.
	pub write_mask: u8,
}

impl StencilState {
	/// Returns a [`StencilState`] appropriate for writing to the stencil buffer.
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

	/// Returns a [`StencilState`] appropriate for using the stencil buffer to
	/// mask drawing operations.
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
