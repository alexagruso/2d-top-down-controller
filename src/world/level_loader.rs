use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LevelConfig {
    pub value: i32,
}

// Needed for [`init_asset_loader`] type bound
#[derive(Default)]
struct LevelConfigLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum LevelConfigLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for LevelConfigLoader {
    type Asset = LevelConfig;
    type Settings = ();
    type Error = LevelConfigLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        println!("{}", String::from_utf8(bytes.clone()).unwrap());

        let level_config = ron::de::from_bytes::<LevelConfig>(&bytes)?;
        Ok(level_config)
    }

    fn extensions(&self) -> &[&str] {
        &["tmx", "xml"]
    }
}

pub struct LevelConfigPlugin;

impl Plugin for LevelConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LevelConfig>()
            .init_asset_loader::<LevelConfigLoader>();
    }
}
