#[macro_export]
macro_rules! with_child_fns {
	() => {
		pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
			self.children.push(Box::new(child));
			self
		}

		pub fn with_children(
			mut self,
			children: impl IntoIterator<Item = impl Widget + 'static>,
		) -> Self {
			for child in children {
				self.children.push(Box::new(child));
			}
			self
		}

		pub fn with_child_if<T: Widget + 'static>(
			mut self,
			condition: bool,
			child: impl FnOnce() -> T,
		) -> Self {
			if condition {
				self.children.push(Box::new(child()));
			}
			self
		}

		pub fn with_children_if(
			mut self,
			condition: bool,
			children: impl IntoIterator<Item = impl Widget + 'static>,
		) -> Self {
			if !condition {
				return self;
			}
			for child in children {
				self.children.push(Box::new(child));
			}
			self
		}
	};
}

#[macro_export]
macro_rules! with_sizing_fns {
	() => {
		pub fn with_sizing(self, sizing: $crate::ui::Sizing) -> Self {
			Self { sizing, ..self }
		}

		pub fn with_horizontal_sizing(mut self, sizing: $crate::ui::AxisSizing) -> Self {
			self.sizing.horizontal = sizing;
			self
		}

		pub fn with_vertical_sizing(mut self, sizing: $crate::ui::AxisSizing) -> Self {
			self.sizing.vertical = sizing;
			self
		}

		pub fn with_max_size(self, size: impl Into<Vec2>) -> Self {
			let size: Vec2 = size.into();
			Self {
				sizing: $crate::ui::Sizing {
					horizontal: $crate::ui::AxisSizing::Max(size.x),
					vertical: $crate::ui::AxisSizing::Max(size.y),
				},
				..self
			}
		}

		pub fn with_fractional_size(self, fraction: impl Into<Vec2>) -> Self {
			let fraction: Vec2 = fraction.into();
			Self {
				sizing: $crate::ui::Sizing {
					horizontal: $crate::ui::AxisSizing::Fractional(fraction.x),
					vertical: $crate::ui::AxisSizing::Fractional(fraction.y),
				},
				..self
			}
		}
	};
}
