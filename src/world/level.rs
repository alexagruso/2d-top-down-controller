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

                let texture = assets.load("textures/MetalTextures.png");
                let layout = TextureAtlasLayout::from_grid(uvec2(16, 16), 2, 3, None, None);
                let atlas_layout = texture_atlas_layouts.add(layout);

                for block in &config.blocks {
                    commands.spawn((
                        Sprite::from_atlas_image(
                            texture.clone(),
                            TextureAtlas {
                                layout: atlas_layout.clone(),
                                index: block.sprite_id,
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
