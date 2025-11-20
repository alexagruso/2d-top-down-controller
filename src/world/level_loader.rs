use std::str::FromStr;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use roxmltree::{Document, Node};
use thiserror::Error;

use crate::math::rotate_vec2_radians;

#[derive(Debug)]
pub struct Tileset {
    pub first_gid: usize,
    pub source: String,
}

#[derive(Debug)]
pub struct Block {
    pub position: Vec2,
    pub size: Vec2,
    pub angle: f32,
    pub atlas_index: usize,
    pub source: String,
}

impl Block {
    fn from_object_node(object_node: Node, tilesets: &Vec<Tileset>) -> Option<Self> {
        let x: f32 = get_attribute(object_node, "x")?;
        let y: f32 = get_attribute(object_node, "y")?;
        let width: f32 = get_attribute(object_node, "width")?;
        let height: f32 = get_attribute(object_node, "height")?;
        let angle: f32 = get_attribute(object_node, "rotation").unwrap_or(0.0);
        let gid: usize = get_attribute(object_node, "gid")?;

        let position = vec2(x, y);
        let size = vec2(width, height);

        for (i, tileset) in tilesets.iter().enumerate() {
            if i >= tilesets.len() - 1
                || gid >= tileset.first_gid && gid < tilesets[i + 1].first_gid
            {
                return Some(Block {
                    position: tiled_to_bevy_position_rect(position, size, angle),
                    size,
                    angle: -angle,
                    atlas_index: gid - tileset.first_gid,
                    source: tileset.source.clone(),
                });
            }
        }

        None
    }
}

#[derive(Asset, TypePath, Debug)]
pub struct LevelConfig {
    pub tilesets: Vec<Tileset>,
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
        let document = Document::parse(str::from_utf8(&bytes[..]).unwrap())?;
        let map = document
            .root()
            .first_child()
            .expect("Document should always have a map tag");

        let mut tilesets: Vec<Tileset> = Vec::new();
        let mut blocks: Vec<Block> = Vec::new();

        'tilesets: for node in map.children() {
            if node.tag_name().name() == "tileset" {
                let first_gid = match node.attribute("firstgid") {
                    // TODO: Handle parsing error
                    Some(value) => value.parse::<usize>().unwrap(),
                    None => continue 'tilesets,
                };
                let source = match node.attribute("source") {
                    // TODO: Handle parsing error
                    Some(value) => value.parse::<String>().unwrap(),
                    None => continue 'tilesets,
                };

                tilesets.push(Tileset { first_gid, source });
            }
        }

        tilesets.sort_by(|a, b| a.first_gid.cmp(&b.first_gid));

        for node in map.children() {
            if node.tag_name().name() == "objectgroup" {
                for object in node.descendants() {
                    if object.tag_name().name() == "object" {
                        if let Some(block) = Block::from_object_node(object, &tilesets) {
                            blocks.push(block);
                        }
                    }
                }
            }
        }

        Ok(LevelConfig { tilesets, blocks })
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

/// Converts Tiled rectangles to Bevy rectangles
///
/// # Arguments
///
/// * `position` - The bottom-left corner of the Tiled rectangle
/// * `size` - The total width/height of the Tiled rectangle
/// * `angle` - The angle the Tiled rectangle is rotated in degrees
#[inline]
fn tiled_to_bevy_position_rect(mut position: Vec2, size: Vec2, angle_degrees: f32) -> Vec2 {
    // Rotations in Tiled are centered at the bottom-left corner of the rectangle, and so we need
    // to rotate the half-size of the rectangle by `angle` and then add that to the Tiled
    // rectangle's position.
    position += rotate_vec2_radians(size * 0.5, angle_degrees.to_radians());

    // Tiled uses top-left coordinates and so we must negate the y-coordinate of the Tiled
    // rectangle.
    position.y *= -1.0;

    position
}

/// Retrieves and parses an attribute from an XML node and returns as an [`Option`]
///
/// # Arguments
///
/// * `node` - The XML tree node to parse
/// * `attr` - The attribute label
#[inline]
fn get_attribute<T: FromStr>(node: Node, attr: &str) -> Option<T> {
    match node.attribute(attr) {
        Some(value) => match value.parse::<T>() {
            Ok(value) => Some(value),
            Err(_) => None,
        },
        None => None,
    }
}
