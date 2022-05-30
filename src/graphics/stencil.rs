#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StencilTest {
	LessThan,
	LessThanOrEqual,
	GreaterThan,
	GreaterThanOrEqual,
	Equal,
	NotEqual,
}

impl StencilTest {
	pub(crate) fn as_glow_stencil_func(&self) -> u32 {
		match self {
			StencilTest::LessThan => glow::LESS,
			StencilTest::LessThanOrEqual => glow::LEQUAL,
			StencilTest::GreaterThan => glow::GREATER,
			StencilTest::GreaterThanOrEqual => glow::GEQUAL,
			StencilTest::Equal => glow::EQUAL,
			StencilTest::NotEqual => glow::NOTEQUAL,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StencilAction {
	Replace(u8),
	Increment,
	IncrementWrap,
	Decrement,
	DecrementWrap,
	Invert,
}

impl StencilAction {
	pub(crate) fn as_glow_stencil_op(&self) -> u32 {
		match self {
			StencilAction::Replace(_) => glow::REPLACE,
			StencilAction::Increment => glow::INCR,
			StencilAction::IncrementWrap => glow::INCR_WRAP,
			StencilAction::Decrement => glow::DECR,
			StencilAction::DecrementWrap => glow::DECR_WRAP,
			StencilAction::Invert => glow::INVERT,
		}
	}
}
