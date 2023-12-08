use super::{shader::Shader, BlendMode, DrawParams, InstanceParams};

#[derive(Debug, Clone, Default)]
pub struct DrawInstancedSettings<'a> {
	pub instances: Vec<InstanceParams>,
	pub shader: Option<&'a Shader>,
	pub blend_mode: BlendMode,
}

impl<'a> DrawInstancedSettings<'a> {
	pub fn new(instances: impl IntoIterator<Item = impl Into<InstanceParams>>) -> Self {
		Self {
			instances: instances
				.into_iter()
				.map(|instance| instance.into())
				.collect(),
			..Default::default()
		}
	}

	pub fn instances(self, instances: impl IntoIterator<Item = impl Into<InstanceParams>>) -> Self {
		Self {
			instances: instances
				.into_iter()
				.map(|instance| instance.into())
				.collect(),
			..self
		}
	}

	pub fn shader(self, shader: &'a Shader) -> Self {
		Self {
			shader: Some(shader),
			..self
		}
	}

	pub fn blend_mode(self, blend_mode: BlendMode) -> Self {
		Self { blend_mode, ..self }
	}
}

impl<'a, T, P> From<T> for DrawInstancedSettings<'a>
where
	T: IntoIterator<Item = P>,
	P: Into<InstanceParams>,
{
	fn from(instances: T) -> Self {
		Self::new(instances)
	}
}

impl<'a> From<DrawParams<'a>> for DrawInstancedSettings<'a> {
	fn from(draw_params: DrawParams<'a>) -> Self {
		DrawInstancedSettings {
			instances: vec![InstanceParams {
				transform: draw_params.transform,
				normal_transform: draw_params.transform.inverse().transpose(),
				color: draw_params.color,
			}],
			shader: draw_params.shader,
			blend_mode: draw_params.blend_mode,
		}
	}
}
