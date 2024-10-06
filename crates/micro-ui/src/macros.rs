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

		pub fn with_child_if<W: Widget + 'static>(
			mut self,
			condition: bool,
			child: impl FnOnce() -> W,
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

		pub fn with_child_if_some<T, W: Widget + 'static>(
			mut self,
			value: &Option<T>,
			child: impl FnOnce(&T) -> W,
		) -> Self {
			if let Some(value) = value {
				self.children.push(Box::new(child(value)));
			}
			self
		}

		pub fn with_children_if_some<T, W: IntoIterator<Item = impl Widget + 'static>>(
			mut self,
			value: &Option<T>,
			children: impl FnOnce(&T) -> W,
		) -> Self {
			if let Some(value) = value {
				for child in children(value) {
					self.children.push(Box::new(child));
				}
			}
			self
		}
	};
}

#[macro_export]
macro_rules! with_sizing_fns {
	() => {
		pub fn with_sizing(self, sizing: $crate::Sizing) -> Self {
			Self { sizing, ..self }
		}

		pub fn with_horizontal_sizing(mut self, sizing: $crate::AxisSizing) -> Self {
			self.sizing.horizontal = sizing;
			self
		}

		pub fn with_vertical_sizing(mut self, sizing: $crate::AxisSizing) -> Self {
			self.sizing.vertical = sizing;
			self
		}

		pub fn with_max_size(self, size: impl Into<Vec2>) -> Self {
			let size: Vec2 = size.into();
			Self {
				sizing: $crate::Sizing {
					horizontal: $crate::AxisSizing::Max(size.x),
					vertical: $crate::AxisSizing::Max(size.y),
				},
				..self
			}
		}

		pub fn with_fractional_size(self, fraction: impl Into<Vec2>) -> Self {
			let fraction: Vec2 = fraction.into();
			Self {
				sizing: $crate::Sizing {
					horizontal: $crate::AxisSizing::Fractional(fraction.x),
					vertical: $crate::AxisSizing::Fractional(fraction.y),
				},
				..self
			}
		}
	};
}
