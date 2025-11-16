use bevy::prelude::*;

use crate::world::{
    geometry::rectangle_wall_bundle,
    level_loader::{LevelConfig, LevelConfigPlugin},
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
    level_config_assets: Res<Assets<LevelConfig>>,
    mut state: ResMut<State>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gizmos: Gizmos,
) {
    let level_config = level_config_assets.get(&state.level_config_handle);

    match level_config {
        Some(config) => {
            if !state.spawned {
                state.spawned = true;

                for block in &config.blocks {
                    commands.spawn(rectangle_wall_bundle(
                        block.size,
                        block.position,
                        block.angle,
                        &mut meshes,
                        &mut materials,
                    ));
                }
            }
        }
        None => {}
    }

    gizmos.circle_2d(Vec2::ZERO, 10.0, Color::srgb(1.0, 0.0, 0.0));
}
