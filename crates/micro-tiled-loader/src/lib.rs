use micro::resource::loader::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TiledMapLoader;

impl ResourceLoader for TiledMapLoader {
	type Resource = tiled::Map;

	type Error = tiled::Error;

	type Settings = ();

	type Context = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["tmx"];

	fn load(
		&mut self,
		_ctx: &mut (),
		path: &std::path::Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		let mut loader = tiled::Loader::new();
		loader.load_tmx_map(path)
	}
}
