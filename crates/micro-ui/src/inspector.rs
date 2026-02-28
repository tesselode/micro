use std::{cell::RefCell, rc::Rc};

use micro::math::{Mat4, Rect};

#[derive(Debug, Clone, PartialEq)]
pub struct WidgetInspector(pub(crate) Rc<RefCell<WidgetInspectorInner>>);

impl WidgetInspector {
	pub fn new() -> Self {
		Self(Rc::new(RefCell::new(WidgetInspectorInner::NotInspected)))
	}

	pub fn bounds(&self) -> Option<Rect> {
		match *self.0.borrow() {
			WidgetInspectorInner::NotInspected => None,
			WidgetInspectorInner::Inspected { bounds, .. } => Some(bounds),
		}
	}

	pub fn transform(&self) -> Option<Mat4> {
		match *self.0.borrow() {
			WidgetInspectorInner::NotInspected => None,
			WidgetInspectorInner::Inspected { transform, .. } => Some(transform),
		}
	}
}

impl Default for WidgetInspector {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum WidgetInspectorInner {
	NotInspected,
	Inspected { bounds: Rect, transform: Mat4 },
}
