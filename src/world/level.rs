use bevy::prelude::*;

use crate::world::level_loader::{LevelConfig, LevelConfigPlugin};

pub struct LevelLoadPlugin;

#[derive(Resource, Default)]
struct State {
    level_config_handle: Handle<LevelConfig>,
    printed: bool,
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
    state.level_config_handle = assets.load("Map/test.xml");
}

fn update_level(level_config_assets: Res<Assets<LevelConfig>>, mut state: ResMut<State>) {
    let level_config = level_config_assets.get(&state.level_config_handle);

    match level_config {
        Some(config) => {
            if !state.printed {
                state.printed = true;
                println!("{}", config.value);
            }
        }
        None => {}
    }
}
