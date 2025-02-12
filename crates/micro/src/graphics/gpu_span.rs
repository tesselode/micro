use std::{cell::RefCell, rc::Rc};

use glow::{HasContext, Query};

pub struct GpuSpan {
	pub(crate) gl: Rc<glow::Context>,
	pub(crate) inner: Rc<RefCell<GpuSpanInner>>,
}

impl Drop for GpuSpan {
	fn drop(&mut self) {
		self.inner.borrow_mut().end(self.gl.clone());
	}
}

pub(crate) struct GpuSpanInner {
	pub(crate) tracy_gpu_span: tracy_client::GpuSpan,
	pub(crate) start_query: Query,
	pub(crate) end_query: Query,
}

impl GpuSpanInner {
	pub(crate) fn end(&mut self, gl: Rc<glow::Context>) {
		self.tracy_gpu_span.end_zone();
		unsafe {
			gl.query_counter(self.end_query, glow::TIMESTAMP);
		}
	}

	pub(crate) fn try_record(&mut self, gl: Rc<glow::Context>) -> bool {
		unsafe {
			let start_available =
				gl.get_query_parameter_u32(self.start_query, glow::QUERY_RESULT_AVAILABLE);
			if start_available == 0 {
				return false;
			}
			let end_available =
				gl.get_query_parameter_u32(self.end_query, glow::QUERY_RESULT_AVAILABLE);
			if end_available == 0 {
				return false;
			}
			let start = gl.get_query_parameter_u64(self.start_query, glow::QUERY_RESULT);
			let end = gl.get_query_parameter_u64(self.end_query, glow::QUERY_RESULT);
			self.tracy_gpu_span.upload_timestamp_start(start as i64);
			self.tracy_gpu_span.upload_timestamp_end(end as i64);
			true
		}
	}
}
