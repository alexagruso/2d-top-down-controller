use std::collections::HashMap;

use avian2d::prelude::{Collider, CollisionLayers, LayerMask, PhysicsLayer};
use bevy::prelude::*;

use crate::{
    physics::ObjectLayer,
    world::{
        geometry::Wall,
        level_loader::{LevelConfig, LevelConfigPlugin},
    },
};

pub struct LevelLoadPlugin;

#[derive(Resource)]
struct State {
    level_config_handle: Handle<LevelConfig>,
    spawned: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            level_config_handle: Handle::default(),
            spawned: false,
        }
    }
}

impl Plugin for LevelLoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>()
            .add_plugins(LevelConfigPlugin)
            .add_systems(Startup, setup_level)
            .add_systems(Update, update_level);
    }
}

fn setup_level(assets: Res<AssetServer>, mut state: ResMut<State>) {
    state.level_config_handle = assets.load("map/test_level.tmx");
}

fn update_level(
    assets: Res<AssetServer>,
    level_config_assets: Res<Assets<LevelConfig>>,
    mut state: ResMut<State>,
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let level_config = level_config_assets.get(&state.level_config_handle);

    match level_config {
        Some(config) => {
            if !state.spawned {
                state.spawned = true;

                let mut textures: HashMap<&String, Handle<Image>> = HashMap::new();

                for tileset in &config.tilesets {
                    textures.insert(
                        &tileset.source,
                        assets.load(format!("textures/placeholders/{}.png", tileset.source)),
                    );
                }

                let metal_texture = assets.load("textures/placeholders/metal.tsx.png");
                let metal_texture_layout =
                    TextureAtlasLayout::from_grid(uvec2(16, 16), 2, 3, None, None);
                let platform_texture = assets.load("textures/placeholders/platform.tsx.png");
                let platform_texture_layout =
                    TextureAtlasLayout::from_grid(uvec2(16, 16), 5, 1, None, None);
                let waltuh_texture = assets.load("textures/placeholders/waltuh.tsx.png");
                let waltuh_texture_layout =
                    TextureAtlasLayout::from_grid(uvec2(128, 128), 3, 3, None, None);

                for block in &config.blocks {
                    let (texture, layout) = match block.source.as_str() {
                        "metal.tsx" => (metal_texture.clone(), metal_texture_layout.clone()),
                        "platform.tsx" => {
                            (platform_texture.clone(), platform_texture_layout.clone())
                        }
                        "waltuh.tsx" => (waltuh_texture.clone(), waltuh_texture_layout.clone()),
                        _ => panic!("invalid asset path"),
                    };

                    commands.spawn((
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layouts.add(layout),
                                index: block.atlas_index,
                            },
                        ),
                        Transform::from_translation(block.position.extend(0.0))
                            .with_rotation(Quat::from_rotation_z(f32::to_radians(block.angle))),
                        Collider::rectangle(block.size.x, block.size.y),
                        CollisionLayers::new(
                            LayerMask(ObjectLayer::Obstacle.to_bits()),
                            LayerMask(ObjectLayer::None.to_bits()),
                        ),
                        Wall,
                    ));
                }
            }
        }
        None => {}
    }

    gizmos.circle_2d(Vec2::ZERO, 10.0, Color::srgb(1.0, 0.0, 0.0));
}

#[derive(Bundle)]
struct BlockBundle {}
