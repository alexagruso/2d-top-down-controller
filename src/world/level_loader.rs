use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use roxmltree::{Document, Node};
use thiserror::Error;

use crate::view_cone::rotate_vec2_radians;

#[derive(Default, Debug)]
pub struct Block {
    pub position: Vec2,
    pub size: Vec2,
    pub angle: f32,
}

impl Block {
    fn from_node(node: &Node) -> Result<Self, ()> {
        // TODO: find a better way to do this.
        let mut x: Option<f32> = None;
        let mut y: Option<f32> = None;
        let mut width: Option<f32> = None;
        let mut height: Option<f32> = None;
        let mut angle: f32 = 0.0;

        for attr in node.attributes() {
            match attr.name() {
                "x" => x = Some(attr.value().parse::<f32>().unwrap()),
                // Tiled Y-coordinate is opposite
                "y" => y = Some(-attr.value().parse::<f32>().unwrap()),
                "width" => width = Some(attr.value().parse::<f32>().unwrap()),
                "height" => height = Some(attr.value().parse::<f32>().unwrap()),
                // Tiled angle is opposite
                "rotation" => angle = -attr.value().parse::<f32>().unwrap(),
                _ => {}
            }
        }

        if let (Some(x), Some(y), Some(width), Some(height)) = (x, y, width, height) {
            Ok(Block {
                // Convert tiled corner coordinates to bevy center coordinates
                position: vec2(x, y)
                    + rotate_vec2_radians(vec2(width, height) * 0.5, f32::to_radians(angle)),
                size: vec2(width, height),
                angle,
            })
        } else {
            Err(())
        }
    }
}

#[derive(Asset, TypePath, Debug)]
pub struct LevelConfig {
    pub blocks: Vec<Block>,
}

// Needed for [`init_asset_loader`] type bound
#[derive(Default)]
struct LevelConfigLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum LevelConfigLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse XML: {0}")]
    Xml(#[from] roxmltree::Error),
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

        let mut blocks = Vec::new();

        let doc = Document::parse(str::from_utf8(&bytes[..]).unwrap())?;
        for node in doc.descendants() {
            if node.has_tag_name("objectgroup") {
                for block_node in node.children() {
                    match Block::from_node(&block_node) {
                        Ok(block) => blocks.push(block),
                        Err(_) => continue,
                    }
                }
            }
        }

        Ok(LevelConfig { blocks })
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
