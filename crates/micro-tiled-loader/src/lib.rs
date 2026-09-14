use micro_asset::AssetLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TiledMapLoader;

impl AssetLoader for TiledMapLoader {
	type Asset = tiled::Map;

	type Error = tiled::Error;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["tmx"];

	fn load(
		&mut self,
		path: &std::path::Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error> {
		let mut loader = tiled::Loader::new();
		loader.load_tmx_map(path)
	}
}
